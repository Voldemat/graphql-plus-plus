mod conditions;
mod tests;
pub mod token_type;
pub mod tokens;
use conditions::get_condition_for_token_type;
use token_type::{ComplexTokenType, TokenType};
use tokens::{Location, Token};

pub struct Lexer {
    buffer: String,
    token_type: Option<ComplexTokenType>,
    location: Location,
}

impl Lexer {
    fn new() -> Self {
        return Self {
            buffer: "".into(),
            token_type: None,
            location: Location::default(),
        };
    }
}

pub enum LexerSuccessTokenResult {
    One(Token),
    Two(Token, Token),
}

impl From<Token> for LexerSuccessTokenResult {
    fn from(value: Token) -> Self {
        return Self::One(value);
    }
}

pub struct LexerSuccessResult(Option<LexerSuccessTokenResult>);
impl From<Option<LexerSuccessTokenResult>> for LexerSuccessResult {
    fn from(value: Option<LexerSuccessTokenResult>) -> Self {
        return Self(value);
    }
}

impl From<(Option<Token>, Option<Token>)> for LexerSuccessResult {
    fn from((first, second): (Option<Token>, Option<Token>)) -> Self {
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

pub type LexerResult = Result<LexerSuccessResult, Error>;

impl Lexer {
    fn extract_token(
        self: &mut Self,
        mut token_type: ComplexTokenType,
    ) -> Token {
        if self.buffer == "true" || self.buffer == "false" {
            token_type = ComplexTokenType::Boolean;
        }
        let token = Token {
            token_type: TokenType::Complex(token_type),
            lexeme: self.buffer.clone(),
            location: self.location.clone(),
        };
        self.token_type = None;
        self.buffer.clear();
        self.location.unlock_start();
        return token;
    }

    fn maybe_extract_token(self: &mut Self) -> Option<Token> {
        self.token_type.map(|t| self.extract_token(t))
    }

    fn feed_with_type(
        self: &mut Self,
        token_type: ComplexTokenType,
        c: char,
    ) -> Option<Token> {
        let condition = get_condition_for_token_type(token_type);
        if !condition(c, &self.buffer) {
            return Some(self.extract_token(token_type));
        }
        self.buffer.push(c);
        self.location.advance();
        return None;
    }

    fn feed_new(self: &mut Self, c: char) -> Result<Option<Token>, Error> {
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
            if complex_token_type != ComplexTokenType::String {
                self.buffer = c.to_string()
            }
            self.location.lock_start();
            return Ok(None);
        }
        return Ok(Some(Token {
            token_type: token_type,
            lexeme: c.to_string(),
            location: self.location.clone(),
        }));
    }

    pub fn feed(self: &mut Self, c: char) -> LexerResult {
        if c == '\n' {
            let maybe_token = self.maybe_extract_token();
            self.location.new_line();
            return Ok(maybe_token.map(|t| t.into()).into());
        }
        let mut maybe_token: Option<Token> = None;
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
