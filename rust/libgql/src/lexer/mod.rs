pub mod conditions;
mod tests;
pub mod token_type;
pub mod tokens;
pub mod types;
pub mod utils;
use std::ops::SubAssign;

use conditions::get_condition_for_token_type;
use token_type::{ComplexTokenType, TokenType};
use tokens::Token;
use types::{Error, LexerLocation, LexerResult, LexerSuccessResult};

use self::{
    conditions::{Condition, ConditionResult},
    tokens::TokenLocation,
};

struct PendingTokenState {
    token_type: ComplexTokenType,
    condition: Box<dyn Condition>,
}

pub struct Lexer<'buffer> {
    buffer: &'buffer str,
    pending_token_state: Option<PendingTokenState>,
    location: LexerLocation,
    pending_empty_string_token: Option<Token<'buffer>>,
}

impl<'buffer> Lexer<'buffer> {
    pub fn new(buffer: &'buffer str) -> Self {
        return Self {
            buffer,
            pending_token_state: None,
            location: LexerLocation::default(),
            pending_empty_string_token: None,
        };
    }
}

impl<'buffer> Lexer<'buffer> {
    pub fn is_in_multiline_token(self: &Self) -> bool {
        self.pending_token_state
            .as_ref()
            .map(|state| state.token_type == ComplexTokenType::MultilineString)
            .unwrap_or(false)
    }

    pub fn feed(self: &mut Self, c: char) -> LexerResult<'buffer> {
        let mut maybe_token: Option<Token<'buffer>> = None;
        if let Some(state) = self.pending_token_state.take() {
            if let Some((token, is_char_part_of_token)) =
                self.feed_with_type(state, c)
            {
                if is_char_part_of_token {
                    return Ok(Some(token.into()).into());
                } else {
                    maybe_token = Some(token);
                }
            } else {
                return Ok(None.into());
            }
        }
        if c != '"'
            && let Some(pending_token) = self.pending_empty_string_token.take()
        {
            assert!(maybe_token.is_none());
            maybe_token = Some(pending_token);
        }
        let result = self.feed_new(c)?;
        return Ok(LexerSuccessResult::from((maybe_token, result)));
    }

    fn feed_new(
        self: &mut Self,
        c: char,
    ) -> Result<Option<Token<'buffer>>, Error> {
        self.location.advance();
        if c == ' ' || c == '\r' || c == '\t' || c == '\n' {
            return Ok(None);
        }
        let Ok(token_type) = TokenType::try_from(c) else {
            return Err(Error::UnexpectedChar {
                c: c,
                location: self.location.create_token_location(),
            });
        };
        if let TokenType::Complex(mut complex_token_type) = token_type {
            if complex_token_type == ComplexTokenType::String
                && self.pending_empty_string_token.is_some()
            {
                complex_token_type = ComplexTokenType::MultilineString;
                self.pending_empty_string_token = None;
                std::num::Wrapping::sub_assign(&mut self.location.start, 2);
            }
            self.pending_token_state = Some(PendingTokenState {
                token_type: complex_token_type,
                condition: get_condition_for_token_type(complex_token_type),
            });
            self.location.lock_start();
            return Ok(None);
        }
        let token = Token {
            token_type: token_type,
            lexeme: &self.buffer[self.location.start.0..=self.location.start.0],
            location: self.location.create_token_location(),
        };
        return Ok(Some(token));
    }

    fn feed_with_type(
        self: &mut Self,
        mut state: PendingTokenState,
        c: char,
    ) -> Option<(Token<'buffer>, bool)> {
        match state.condition.evaluate(c) {
            ConditionResult::False {
                is_char_part_of_token,
            } => {
                if is_char_part_of_token {
                    self.location.advance();
                }
                if state.token_type == ComplexTokenType::String
                    && self.location.start
                        == self.location.end - std::num::Wrapping(1)
                {
                    self.pending_empty_string_token =
                        Some(self.extract_token(state));
                    return None;
                }
                Some((self.extract_token(state), is_char_part_of_token))
            }
            ConditionResult::True => {
                self.pending_token_state = Some(state);
                self.location.advance();
                None
            }
        }
    }

    fn extract_token(
        self: &mut Self,
        mut state: PendingTokenState,
    ) -> Token<'buffer> {
        let lexeme_start = match state.token_type {
            ComplexTokenType::String => self.location.start.0 + 1,
            ComplexTokenType::MultilineString => self.location.start.0 + 3,
            _ => self.location.start.0,
        };
        let lexeme_end = match state.token_type {
            ComplexTokenType::String => self.location.end.0 - 1,
            ComplexTokenType::MultilineString => self.location.end.0 - 3,
            _ => self.location.end.0,
        };
        let lexeme = &self.buffer[lexeme_start..=lexeme_end];
        if state.token_type == ComplexTokenType::Identifier
            && (lexeme == "true" || lexeme == "false")
        {
            state.token_type = ComplexTokenType::Boolean;
        }
        let token = Token {
            token_type: TokenType::Complex(state.token_type),
            lexeme,
            location: TokenLocation {
                start: self.location.start.0,
                end: self.location.end.0,
            },
        };
        self.location.unlock_start();
        return token;
    }

    fn maybe_extract_token(self: &mut Self) -> Option<Token<'buffer>> {
        self.pending_token_state
            .take()
            .map(|t| self.extract_token(t))
    }
}
