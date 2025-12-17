use std::rc::Rc;

use super::shared;
use crate::lexer;

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
}

pub trait TokensSource {
    fn lookahead(_: &Self) -> Option<&lexer::tokens::Token>;
    fn advance(_: &mut Self);
    fn consume(
        _: &mut Self,
        token_type: lexer::token_type::TokenType,
    ) -> Result<&lexer::tokens::Token, ConsumeError>;
    fn consume_identifier(
        _: &mut Self,
    ) -> Result<&lexer::tokens::Token, ConsumeError>;
    fn consume_if_is_ahead(
        _: &mut Self,
        token_type: lexer::token_type::TokenType,
    ) -> bool;
    fn consume_identifier_by_lexeme<'a>(
        _: &'a mut Self,
        lexeme: &str,
    ) -> Result<&'a lexer::tokens::Token, ConsumeError>;
    fn consume_identifier_by_lexeme_if_is_ahead(
        _: &mut Self,
        lexeme: &str,
    ) -> bool;
    fn get_current_token(_: &Self) -> &lexer::tokens::Token;
    fn get_source_file(_: &Self) -> Rc<shared::ast::SourceFile>;
    fn is_ahead(_: &Self, token_type: lexer::token_type::TokenType) -> bool;
    fn is_ahead_by_lexeme(_: &Self, lexeme: &str) -> bool;
}
