mod conditions;
mod tests;
pub mod token_type;
pub mod tokens;
pub mod utils;
use conditions::get_condition_for_token_type;
use token_type::{ComplexTokenType, TokenType};
use tokens::{Location, Token};

pub struct Lexer<'buffer> {
    buffer: &'buffer str,
    token_type: Option<ComplexTokenType>,
    location: Location,
    buffer_head: usize,
}

impl<'buffer> Lexer<'buffer> {
    pub fn new(buffer: &'buffer str) -> Self {
        return Self {
            buffer,
            token_type: None,
            location: Location::default(),
            buffer_head: 0,
        };
    }
}

pub enum LexerSuccessTokenResult<'buffer> {
    One(Token<'buffer>),
    Two(Token<'buffer>, Token<'buffer>),
}

impl<'buffer> From<Token<'buffer>> for LexerSuccessTokenResult<'buffer> {
    fn from(value: Token<'buffer>) -> Self {
        return Self::One(value);
    }
}

pub struct LexerSuccessResult<'buffer>(
    Option<LexerSuccessTokenResult<'buffer>>,
);
impl<'buffer> From<Option<LexerSuccessTokenResult<'buffer>>>
    for LexerSuccessResult<'buffer>
{
    fn from(value: Option<LexerSuccessTokenResult<'buffer>>) -> Self {
        return Self(value);
    }
}

impl<'buffer> From<(Option<Token<'buffer>>, Option<Token<'buffer>>)>
    for LexerSuccessResult<'buffer>
{
    fn from(
        (first, second): (Option<Token<'buffer>>, Option<Token<'buffer>>),
    ) -> Self {
        match (first, second) {
            (None, None) => None.into(),
            (Some(t1), Some(t2)) => {
                Some(LexerSuccessTokenResult::Two(t1, t2)).into()
            }
            (Some(t1), None) => Some(LexerSuccessTokenResult::One(t1)).into(),
            (None, Some(t2)) => Some(LexerSuccessTokenResult::One(t2)).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnexpectedChar { c: char, location: Location },
}

pub type LexerResult<'buffer> = Result<LexerSuccessResult<'buffer>, Error>;

impl<'buffer> Lexer<'buffer> {
    fn extract_token(
        self: &mut Self,
        mut token_type: ComplexTokenType,
    ) -> Token<'buffer> {
        if self.buffer == "true" || self.buffer == "false" {
            token_type = ComplexTokenType::Boolean;
        }
        let lexeme_start = {
            let start = self.buffer_head + self.location.get_start() as usize;
            if token_type == ComplexTokenType::String {
                start + 1
            } else {
                start
            }
        };
        let lexeme_end = {
            let end = self.buffer_head + self.location.get_end() as usize;
            if token_type == ComplexTokenType::String {
                end
            } else {
                end + 1
            }
        };
        let lexeme = &self.buffer[lexeme_start..lexeme_end];
        println!("extract_token: lexeme: \"{}\"", lexeme);
        let token = Token {
            token_type: TokenType::Complex(token_type),
            lexeme: &self.buffer[lexeme_start..lexeme_end],
            location: self.location.clone(),
        };
        self.token_type = None;
        self.location.unlock_start();
        return token;
    }

    fn maybe_extract_token(self: &mut Self) -> Option<Token<'buffer>> {
        self.token_type.map(|t| self.extract_token(t))
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

    fn feed_new(
        self: &mut Self,
        c: char,
    ) -> Result<Option<Token<'buffer>>, Error> {
        self.location.advance();
        if c == ' ' || c == '\r' || c == '\t' {
            return Ok(None);
        }
        let Ok(token_type) = TokenType::try_from(c) else {
            return Err(Error::UnexpectedChar {
                c: c,
                location: self.location.clone(),
            });
        };
        if let TokenType::Complex(complex_token_type) = token_type {
            self.token_type = Some(complex_token_type);
            self.location.lock_start();
            return Ok(None);
        }
        return Ok(Some(Token {
            token_type: token_type,
            lexeme: &self.buffer[(self.buffer_head
                + self.location.get_start() as usize)
                ..(self.buffer_head + self.location.get_start() as usize + 1)],
            location: self.location.clone(),
        }));
    }

    pub fn feed(self: &mut Self, c: char) -> LexerResult<'buffer> {
        if c == '\n' {
            let maybe_token = self.maybe_extract_token();
            self.buffer_head += self.location.get_start() as usize + 2;
            self.location.new_line();
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
                return Ok(Some(token.into()).into());
            };
        }
        let result = self.feed_new(c)?;
        return Ok(LexerSuccessResult::from((maybe_token, result)));
    }
}
