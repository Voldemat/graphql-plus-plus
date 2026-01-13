use std::rc::Rc;

use crate::{
    lexer::tokens::Token,
    parsers::file::{
        shared::ast::SourceFile,
        tokens_source::{ConsumeError, TokensSource},
    },
};

pub struct VecTokensSource {
    tokens: Vec<Token>,
    source_file: Rc<SourceFile>,
    current_index: usize,
}

impl VecTokensSource {
    pub fn new(tokens: Vec<Token>, source_file: Rc<SourceFile>) -> Self {
        assert!(tokens.len() > 0);
        return Self {
            tokens,
            source_file,
            current_index: 0
        }
    }
}

impl TokensSource for VecTokensSource {
    fn lookahead(self: &Self) -> Option<&crate::lexer::tokens::Token> {
        self.tokens.get(self.current_index + 1)
    }

    fn advance(self: &mut Self) -> Result<(), ConsumeError> {
        if self.current_index + 1 == self.tokens.len() {
            return Err(ConsumeError::EOF{
                token: self.get_current_token().clone()
            })
        }
        self.current_index += 1;
        return Ok(())
    }

    fn consume(
        self: &mut Self,
        token_type: crate::lexer::token_type::TokenType,
    ) -> Result<&crate::lexer::tokens::Token, super::tokens_source::ConsumeError>
    {
        if self.current_index == self.tokens.len() {
            return Err(ConsumeError::EOF {
                token: self.get_current_token().clone(),
            });
        };
        self.advance()?;
        let token = self.get_current_token();
        if token.token_type != token_type {
            return Err(ConsumeError::WrongType {
                expected_token_type: token_type,
                received_token: token.clone(),
            });
        }
        return Ok(token);
    }

    fn get_current_token(self: &Self) -> &crate::lexer::tokens::Token {
        return self.tokens.get(self.current_index).unwrap();
    }

    fn get_source_file(self: &Self) -> Rc<SourceFile> {
        return self.source_file.clone();
    }
}
