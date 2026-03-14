use std::num::Wrapping;

use crate::lexer::TokenType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    line: u32,
    start: Wrapping<u32>,
    end: Wrapping<u32>,
    is_start_locked: bool,
}

impl Default for Location {
    fn default() -> Self {
        return Self {
            line: 1,
            start: Wrapping(u32::MAX),
            end: Wrapping(u32::MAX),
            is_start_locked: false,
        };
    }
}

impl Location {
    pub fn new(line: u32, start: Wrapping<u32>, end: Wrapping<u32>) -> Self {
        return Self {
            line,
            start,
            end,
            is_start_locked: false,
        };
    }

    #[inline]
    pub fn get_line(self: &Self) -> u32 {
        return self.line;
    }

    #[inline]
    pub fn get_start(self: &Self) -> u32 {
        return self.start.0;
    }

    #[inline]
    pub fn get_end(self: &Self) -> u32 {
        return self.end.0;
    }

    pub fn new_line(self: &mut Self) {
        self.line += 1;
        self.start = Wrapping(u32::MAX);
        self.end = Wrapping(u32::MAX);
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

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
            && self.start == other.start
            && self.end == other.end
    }
}

impl Eq for Location {}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub location: Location,
}

impl Token {
    pub fn is_keyword(self: &Self) -> bool {
        self.lexeme == "type"
            || self.lexeme == "query"
            || self.lexeme == "input"
            || self.lexeme == "extend"
            || self.lexeme == "directive"
    }
}
