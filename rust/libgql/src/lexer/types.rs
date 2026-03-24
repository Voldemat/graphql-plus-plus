use super::tokens::{Token, TokenLocation};

#[derive(Debug, PartialEq, Eq)]
pub struct LexerLocation {
    pub start: usize,
    pub end: usize,
    is_start_locked: bool,
}

impl Default for LexerLocation {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            is_start_locked: false,
        }
    }
}

impl LexerLocation {
    pub fn create_token_location(self: &Self) -> TokenLocation {
        TokenLocation {
            start: self.start,
            end: self.end,
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

pub type LexerResult<'buffer> = Result<LexerSuccessResult<'buffer>, Error>;
