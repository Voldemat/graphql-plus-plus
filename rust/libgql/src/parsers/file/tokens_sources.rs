use std::sync::Arc;

use crate::{
    lexer::tokens::Token,
    parsers::file::{
        shared::ast::SourceFile,
        tokens_source::{ConsumeError, TokensSource},
    },
};

pub struct VecTokensSource<'buffer, 'tokens> {
    tokens: &'tokens Vec<Token<'buffer>>,
    source_file: Arc<SourceFile<'buffer>>,
    current_index: usize,
}

impl<'buffer, 'tokens> VecTokensSource<'buffer, 'tokens> {
    pub fn new(
        tokens: &'tokens Vec<Token<'buffer>>,
        source_file: Arc<SourceFile<'buffer>>,
    ) -> Self {
        assert!(tokens.len() > 0);
        return Self {
            tokens,
            source_file,
            current_index: 0,
        };
    }
}

impl<'buffer, 'tokens> TokensSource<'buffer, 'tokens>
    for VecTokensSource<'buffer, 'tokens>
{
    fn lookahead(
        self: &Self,
    ) -> Option<&'tokens crate::lexer::tokens::Token<'buffer>> {
        self.tokens.get(self.current_index + 1)
    }

    fn advance(self: &mut Self) -> Result<(), ConsumeError<'buffer>> {
        if self.current_index + 1 == self.tokens.len() {
            return Err(ConsumeError::EOF(self.get_current_token().clone()));
        }
        self.current_index += 1;
        return Ok(());
    }

    fn consume(
        self: &mut Self,
        token_type: crate::lexer::token_type::TokenType,
    ) -> Result<
        &'tokens crate::lexer::tokens::Token<'buffer>,
        super::tokens_source::ConsumeError<'buffer>,
    > {
        if self.current_index == self.tokens.len() {
            return Err(ConsumeError::EOF(self.get_current_token().clone()));
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

    fn get_current_token(
        self: &Self,
    ) -> &'tokens crate::lexer::tokens::Token<'buffer> {
        return self.tokens.get(self.current_index).unwrap();
    }

    fn get_source_file(self: &Self) -> Arc<SourceFile<'buffer>> {
        return self.source_file.clone();
    }
}
