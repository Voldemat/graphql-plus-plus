use crate::lexer::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Token<'lexeme> {
    pub token_type: TokenType,
    pub lexeme: &'lexeme str,
    pub location: TokenLocation,
}

impl<'lexeme> Token<'lexeme> {
    pub fn is_keyword(self: &Self) -> bool {
        self.lexeme == "type"
            || self.lexeme == "query"
            || self.lexeme == "input"
            || self.lexeme == "extend"
            || self.lexeme == "directive"
    }
}
