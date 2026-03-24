use std::sync::Arc;

use super::shared;
use crate::lexer;

#[derive(Debug)]
pub enum ConsumeError<'buffer> {
    EOF(lexer::tokens::Token<'buffer>),
    WrongType {
        expected_token_type: lexer::token_type::TokenType,
        received_token: lexer::tokens::Token<'buffer>,
    },
    UnexpectedLexeme {
        expected_lexeme: &'static str,
        received_token: lexer::tokens::Token<'buffer>,
    },
}

impl<'buffer, 'tokens> ConsumeError<'buffer> {
    pub fn is_eof(self: &Self) -> bool {
        match self {
            Self::EOF(_) => true,
            _ => false,
        }
    }

    pub fn get_location(self: &Self) -> &lexer::tokens::TokenLocation {
        match self {
            Self::EOF(token) => &token.location,
            Self::WrongType { received_token, .. } => &received_token.location,
            Self::UnexpectedLexeme { received_token, .. } => {
                &received_token.location
            }
        }
    }
}

pub trait TokensSource<'buffer> {
    fn lookahead(self: &Self) -> Option<&lexer::tokens::Token<'buffer>>;
    fn advance(self: &mut Self) -> Result<(), ConsumeError<'buffer>>;
    fn consume(
        self: &mut Self,
        token_type: lexer::token_type::TokenType,
    ) -> Result<&lexer::tokens::Token<'buffer>, ConsumeError<'buffer>>;
    fn get_current_token(self: &Self) -> &lexer::tokens::Token<'buffer>;
    fn get_source_file(self: &Self) -> Arc<shared::ast::SourceFile<'buffer>>;

    fn consume_identifier(
        self: &mut Self,
    ) -> Result<&lexer::tokens::Token<'buffer>, ConsumeError<'buffer>> {
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

    fn consume_identifier_by_lexeme(
        self: &mut Self,
        lexeme: &'static str,
    ) -> Result<&lexer::tokens::Token<'buffer>, ConsumeError<'buffer>> {
        let token = self.consume_identifier()?;
        if token.lexeme != lexeme {
            return Err(ConsumeError::UnexpectedLexeme {
                expected_lexeme: lexeme,
                received_token: token.clone(),
            });
        }
        return Ok(token);
    }

    fn consume_identifier_by_lexeme_if_is_ahead(
        self: &mut Self,
        lexeme: &'static str,
    ) -> bool {
        let Some(next_token) = Self::lookahead(self) else {
            return false;
        };
        if next_token.lexeme != lexeme {
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
        return next_token.lexeme == lexeme;
    }
}
