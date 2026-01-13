use std::rc::Rc;

use super::shared;
use crate::lexer;

#[derive(Debug)]
pub enum ConsumeError {
    EOF {
        token: lexer::tokens::Token,
    },
    WrongType {
        expected_token_type: lexer::token_type::TokenType,
        received_token: lexer::tokens::Token,
    },
    UnexpectedLexeme {
        expected_lexeme: String,
        received_token: lexer::tokens::Token,
    },
}

impl ConsumeError {
    pub fn is_eof(self: &Self) -> bool {
        match self {
            Self::EOF { token: _ } => true,
            _ => false,
        }
    }

    pub fn get_location(self: &Self) -> lexer::tokens::Location {
        match self {
            Self::EOF { token } => token.location.clone(),
            Self::WrongType { received_token, .. } => {
                received_token.location.clone()
            }
            Self::UnexpectedLexeme { received_token, .. } => {
                received_token.location.clone()
            }
        }
    }
}

pub trait TokensSource {
    fn lookahead(self: &Self) -> Option<&lexer::tokens::Token>;
    fn advance(self: &mut Self) -> Result<(), ConsumeError>;
    fn consume(
        self: &mut Self,
        token_type: lexer::token_type::TokenType,
    ) -> Result<&lexer::tokens::Token, ConsumeError>;
    fn get_current_token(self: &Self) -> &lexer::tokens::Token;
    fn get_source_file(self: &Self) -> Rc<shared::ast::SourceFile>;

    fn consume_identifier(
        self: &mut Self,
    ) -> Result<&lexer::tokens::Token, ConsumeError> {
        return Self::consume(
            self,
            lexer::token_type::TokenType::Complex(
                lexer::token_type::ComplexTokenType::Identifier,
            ),
        );
    }

    fn consume_if_is_ahead(
        self: &mut Self,
        token_type: lexer::token_type::TokenType,
    ) -> bool {
        let Some(token) = Self::lookahead(self) else {
            return false;
        };
        if token.token_type != token_type {
            return false;
        }
        Self::advance(self).unwrap();
        return true;
    }

    fn consume_identifier_by_lexeme<'a>(
        self: &'a mut Self,
        lexeme: &str,
    ) -> Result<&'a lexer::tokens::Token, ConsumeError> {
        let token = self.consume_identifier()?;
        if token.lexeme.as_str() != lexeme {
            return Err(ConsumeError::UnexpectedLexeme {
                expected_lexeme: lexeme.to_string(),
                received_token: token.clone(),
            });
        }
        return Ok(token);
    }

    fn consume_identifier_by_lexeme_if_is_ahead(
        self: &mut Self,
        lexeme: &str,
    ) -> bool {
        let Some(next_token) = Self::lookahead(self) else {
            return false;
        };
        if next_token.lexeme.as_str() != lexeme {
            return false;
        }
        Self::advance(self).unwrap();
        return true;
    }

    fn is_ahead(self: &Self, token_type: lexer::token_type::TokenType) -> bool {
        let Some(next_token) = Self::lookahead(self) else {
            return false;
        };
        return next_token.token_type == token_type;
    }

    fn is_ahead_by_lexeme(self: &Self, lexeme: &str) -> bool {
        let Some(next_token) = Self::lookahead(self) else {
            return false;
        };
        return next_token.lexeme.as_str() == lexeme;
    }
}
