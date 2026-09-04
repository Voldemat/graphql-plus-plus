use super::tokens::{Token, TokenLocation};

#[derive(Debug, PartialEq, Eq)]
pub struct LexerLocation {
    pub start: std::num::Wrapping<usize>,
    pub end: std::num::Wrapping<usize>,
    is_start_locked: bool,
}

impl Default for LexerLocation {
    fn default() -> Self {
        Self {
            start: std::num::Wrapping(usize::MAX),
            end: std::num::Wrapping(usize::MAX),
            is_start_locked: false,
        }
    }
}

impl LexerLocation {
    pub fn create_token_location(self: &Self) -> TokenLocation {
        TokenLocation {
            start: self.start.0,
            end: self.end.0,
        }
    }

    pub fn lock_start(self: &mut Self) {
        self.is_start_locked = true;
    }

    pub fn advance(self: &mut Self) {
        self.end += 1;
        if !self.is_start_locked {
            self.start += 1;
        }
    }

    pub fn unlock_start(self: &mut Self) {
        self.start = self.end;
        self.is_start_locked = false;
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
    pub Option<LexerSuccessTokenResult<'buffer>>,
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
    UnexpectedChar { c: char, location: TokenLocation },
}

impl Error {
    pub fn get_location(self: &Self) -> &TokenLocation {
        match self {
            Self::UnexpectedChar { location, .. } => location,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar { .. } => f.write_str("Unexpected char"),
        }
    }
}

pub type LexerResult<'buffer> = Result<LexerSuccessResult<'buffer>, Error>;
