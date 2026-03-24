pub mod conditions;
mod tests;
pub mod token_type;
pub mod tokens;
pub mod types;
pub mod utils;
use conditions::get_condition_for_token_type;
use token_type::{ComplexTokenType, TokenType};
use tokens::Token;
use types::{Error, LexerLocation, LexerResult, LexerSuccessResult};

use self::tokens::TokenLocation;

pub struct Lexer<'buffer> {
    buffer: &'buffer str,
    token_type: Option<ComplexTokenType>,
    location: LexerLocation,
}

impl<'buffer> Lexer<'buffer> {
    pub fn new(buffer: &'buffer str) -> Self {
        return Self {
            buffer,
            token_type: None,
            location: LexerLocation::default(),
        };
    }
}

impl<'buffer> Lexer<'buffer> {
    pub fn feed(self: &mut Self, c: char) -> LexerResult<'buffer> {
        if c == '\n' {
            let maybe_token = self.maybe_extract_token();
            self.location.advance();
            return Ok(maybe_token.map(|t| t.into()).into());
        }
        let mut maybe_token: Option<Token<'buffer>> = None;
        if let Some(token_type) = self.token_type {
            maybe_token = self.feed_with_type(token_type, c);
            if maybe_token.is_none() {
                return Ok(None.into());
            }
            if token_type == ComplexTokenType::String
                && let Some(token) = maybe_token
            {
                self.location.advance();
                return Ok(Some(token.into()).into());
            };
        }
        let result = self.feed_new(c)?;
        return Ok(LexerSuccessResult::from((maybe_token, result)));
    }

    fn feed_new(
        self: &mut Self,
        c: char,
    ) -> Result<Option<Token<'buffer>>, Error> {
        if self.location.start != 0 {
            self.location.advance();
        }
        if c == ' ' || c == '\r' || c == '\t' {
            return Ok(None);
        }
        let Ok(token_type) = TokenType::try_from(c) else {
            return Err(Error::UnexpectedChar {
                c: c,
                location: self.location.create_token_location(),
            });
        };
        if let TokenType::Complex(complex_token_type) = token_type {
            self.token_type = Some(complex_token_type);
            self.location.lock_start();
            return Ok(None);
        }
        let token = Token {
            token_type: token_type,
            lexeme: &self.buffer
                [self.location.start..(self.location.start + 1)],
            location: self.location.create_token_location(),
        };
        return Ok(Some(token));
    }

    fn feed_with_type(
        self: &mut Self,
        token_type: ComplexTokenType,
        c: char,
    ) -> Option<Token<'buffer>> {
        let condition = get_condition_for_token_type(token_type);
        if !condition(c, &self.buffer) {
            return Some(self.extract_token(token_type));
        }
        self.location.advance();
        return None;
    }

    fn extract_token(
        self: &mut Self,
        mut token_type: ComplexTokenType,
    ) -> Token<'buffer> {
        let lexeme_start = if token_type == ComplexTokenType::String {
            self.location.start + 1
        } else {
            self.location.start
        };
        let lexeme = &self.buffer[lexeme_start..(self.location.end + 1)];
        if token_type == ComplexTokenType::Identifier
            && (lexeme == "true" || lexeme == "false")
        {
            token_type = ComplexTokenType::Boolean;
        }
        let token = Token {
            token_type: TokenType::Complex(token_type),
            lexeme,
            location: TokenLocation {
                start: self.location.start,
                end: if token_type == ComplexTokenType::String {
                    self.location.end + 1
                } else {
                    self.location.end
                },
            },
        };
        self.token_type = None;
        self.location.unlock_start();
        return token;
    }

    fn maybe_extract_token(self: &mut Self) -> Option<Token<'buffer>> {
        self.token_type.map(|t| self.extract_token(t))
    }
}
