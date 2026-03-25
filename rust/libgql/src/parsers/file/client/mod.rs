use std::sync::Arc;

use crate::{
    lexer::{
        self,
        token_type::{ComplexTokenType, SimpleTokenType},
    },
    parsers::file::{
        base::{self, BaseParser},
        shared, tokens_source,
    },
};

pub mod ast;

#[derive(Debug, derive_more::From)]
pub enum Error<'buffer> {
    Base(base::Error<'buffer>),
    UnexpectedOpType(lexer::tokens::Token<'buffer>),
    DuplicateParameter {
        existing_parameter: shared::ast::InputFieldDefinitionNode<'buffer>,
        duplicate_parameter: shared::ast::InputFieldDefinitionNode<'buffer>,
    },
}

impl<'buffer> Error<'buffer> {
    pub fn is_eof(self: &Self) -> bool {
        match self {
            Error::Base(error) => error.is_eof(),
            _ => false,
        }
    }

    pub fn get_location(self: &Self) -> (usize, usize) {
        match self {
            Self::Base(b) => {
                let location = b.get_location();
                (location.start, location.end)
            }
            Self::UnexpectedOpType(token) => {
                (token.location.start, token.location.end)
            }
            Self::DuplicateParameter {
                duplicate_parameter,
                ..
            } => (
                duplicate_parameter.location.start,
                duplicate_parameter.location.end,
            ),
        }
    }
}

impl<'buffer> From<tokens_source::ConsumeError<'buffer>> for Error<'buffer> {
    fn from(value: tokens_source::ConsumeError<'buffer>) -> Self {
        return Self::Base(value.into());
    }
}

pub struct Parser<'buffer, T: tokens_source::TokensSource<'buffer>> {
    base: BaseParser<'buffer, T, ast::DirectiveLocation>,
}

impl<'buffer, T: tokens_source::TokensSource<'buffer>> Parser<'buffer, T> {
    pub fn new(tokens_source: T) -> Self {
        Self {
            base: BaseParser::new(tokens_source),
        }
    }

    pub fn parse_ast_nodes(
        self: &mut Self,
    ) -> Result<Vec<ast::ASTNode<'buffer>>, Error<'buffer>> {
        let mut nodes = Vec::<ast::ASTNode<'buffer>>::new();
        'l: loop {
            match self.parse_ast_node() {
                Ok(node) => nodes.push(node),
                Err(error) => {
                    if error.is_eof() {
                        break 'l;
                    }
                    return Err(error);
                }
            }
            if let Some(e) = self.base.tokens_source.advance().err() {
                if e.is_eof() {
                    break 'l;
                }
                return Err(e.into());
            }
        }
        return Ok(nodes);
    }

    pub fn parse_ast_node(
        self: &mut Self,
    ) -> Result<ast::ASTNode<'buffer>, Error<'buffer>> {
        let token = T::get_current_token(&self.base.tokens_source);
        match token.lexeme {
            "fragment" => Ok(self.parse_fragment_definition()?.into()),
            "directive" => Ok(self.base.parse_directive_node()?.into()),
            _ => Ok(self.parse_operation_definition()?.into()),
        }
    }

    fn parse_fragment_definition(
        self: &mut Self,
    ) -> Result<ast::FragmentDefinition<'buffer>, Error<'buffer>> {
        let start = T::get_current_token(&self.base.tokens_source)
            .location
            .start;
        let name = self.base.parse_name_node(false)?;
        T::consume_identifier_by_lexeme(&mut self.base.tokens_source, "on")?;
        let type_name = self.base.parse_name_node(false)?;
        let spec = self.parse_fragment_spec()?;
        return Ok(ast::FragmentDefinition {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.base.tokens_source)
                    .location
                    .end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            name,
            type_name,
            spec,
        });
    }

    fn parse_operation_definition(
        self: &mut Self,
    ) -> Result<ast::OperationDefinition<'buffer>, Error<'buffer>> {
        let start_token = T::get_current_token(&self.base.tokens_source);
        let Ok(optype) = ast::OpType::try_from(start_token.lexeme) else {
            return Err(Error::UnexpectedOpType(start_token.clone()));
        };
        let start = start_token.location.start;
        let name = self.base.parse_name_node(false)?;
        let parameters = self.parse_operation_parameters()?;
        let fragment = self.parse_fragment_spec()?;
        return Ok(ast::OperationDefinition {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.base.tokens_source)
                    .location
                    .end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            r#type: optype,
            name,
            parameters,
            fragment,
        });
    }

    fn parse_operation_parameters(
        self: &mut Self,
    ) -> Result<
        Vec<shared::ast::InputFieldDefinitionNode<'buffer>>,
        Error<'buffer>,
    > {
        let mut parameters =
            Vec::<shared::ast::InputFieldDefinitionNode<'buffer>>::new();
        if T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            while T::is_ahead(
                &self.base.tokens_source,
                ComplexTokenType::Identifier.into(),
            ) {
                let node = self.base.parse_input_field_definition_node()?;
                if let Some(existing_parameter) =
                    parameters.iter().find(|p| p.name.name == node.name.name)
                {
                    return Err(Error::DuplicateParameter {
                        existing_parameter: existing_parameter.clone(),
                        duplicate_parameter: node,
                    });
                };
                parameters.push(node);
                T::consume_if_is_ahead(
                    &mut self.base.tokens_source,
                    SimpleTokenType::Comma.into(),
                );
            }
            T::consume(
                &mut self.base.tokens_source,
                SimpleTokenType::RightParen.into(),
            )?;
        }
        return Ok(parameters);
    }

    fn parse_fragment_spec(
        self: &mut Self,
    ) -> Result<ast::FragmentSpec<'buffer>, Error<'buffer>> {
        let start = T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        )?
        .location
        .start;
        let selections = self.parse_selection_nodes()?;
        let end_token = T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::RightBrace.into(),
        )?;
        return Ok(ast::FragmentSpec {
            location: shared::ast::NodeLocation {
                start,
                end: end_token.location.end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            selections,
        });
    }

    fn parse_selection_nodes(
        self: &mut Self,
    ) -> Result<Vec<ast::SelectionNode<'buffer>>, Error<'buffer>> {
        let mut selections = Vec::<ast::SelectionNode<'buffer>>::new();
        while T::is_ahead(
            &self.base.tokens_source,
            ComplexTokenType::Identifier.into(),
        ) || T::is_ahead(
            &self.base.tokens_source,
            ComplexTokenType::Spread.into(),
        ) {
            selections.push(self.parse_selection_node()?);
            T::consume_if_is_ahead(
                &mut self.base.tokens_source,
                SimpleTokenType::Comma.into(),
            );
        }
        return Ok(selections);
    }

    fn parse_selection_node(
        self: &mut Self,
    ) -> Result<ast::SelectionNode<'buffer>, Error<'buffer>> {
        if !T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            ComplexTokenType::Spread.into(),
        ) {
            return Ok(self.parse_field_selection_node()?.into());
        }
        if T::is_ahead_by_lexeme(&self.base.tokens_source, "on") {
            return Ok(self.parse_conditional_spread_selection_node()?.into());
        }
        let start = T::get_current_token(&self.base.tokens_source)
            .location
            .start;
        let fragment_name = self.base.parse_name_node(false)?;
        return Ok(ast::SpreadSelectionNode {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.base.tokens_source)
                    .location
                    .end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            fragment_name,
        }
        .into());
    }

    fn parse_field_selection_node(
        self: &mut Self,
    ) -> Result<ast::FieldSelectionNode<'buffer>, Error<'buffer>> {
        let start = T::get_current_token(&self.base.tokens_source)
            .location
            .start;
        let field_spec = self.parse_object_field_spec()?;
        let mut spec: Option<Arc<ast::FragmentSpec>> = None;
        if T::is_ahead(
            &self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        ) {
            spec = Some(Arc::new(self.parse_fragment_spec()?));
        }
        return Ok(ast::FieldSelectionNode {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.base.tokens_source)
                    .location
                    .end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            field: field_spec,
            spec,
        });
    }

    fn parse_conditional_spread_selection_node(
        self: &mut Self,
    ) -> Result<ast::ConditionalSpreadSelectionNode<'buffer>, Error<'buffer>>
    {
        let start = T::get_current_token(&self.base.tokens_source)
            .location
            .start;
        T::consume_identifier_by_lexeme(&mut self.base.tokens_source, "on")?;
        let type_name = self.base.parse_name_node(false)?;
        let fragment_spec = self.parse_fragment_spec()?;
        return Ok(ast::ConditionalSpreadSelectionNode {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.base.tokens_source)
                    .location
                    .end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            type_name,
            fragment: Arc::new(fragment_spec),
        });
    }

    fn parse_object_field_spec(
        self: &mut Self,
    ) -> Result<ast::ObjectFieldSpec<'buffer>, Error<'buffer>> {
        let start = T::get_current_token(&self.base.tokens_source)
            .location
            .start;
        let (name, selection_name) = self.parse_name_and_selection_name()?;
        if !T::is_ahead(
            &self.base.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            return Ok(ast::ObjectLiteralFieldSpec {
                location: shared::ast::NodeLocation {
                    start,
                    end: T::get_current_token(&self.base.tokens_source)
                        .location
                        .end,
                    source: T::get_source_file(&self.base.tokens_source),
                },
                selection_name,
                name,
            }
            .into());
        }
        let arguments = self.base.parse_arguments()?;
        return Ok(ast::ObjectCallableFieldSpec {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.base.tokens_source)
                    .location
                    .end,
                source: T::get_source_file(&self.base.tokens_source),
            },
            selection_name,
            name,
            arguments,
        }
        .into());
    }

    fn parse_name_and_selection_name(
        self: &mut Self,
    ) -> Result<
        (
            shared::ast::NameNode<'buffer>,
            shared::ast::NameNode<'buffer>,
        ),
        Error<'buffer>,
    > {
        let selection_name = self.base.parse_name_node(false)?;
        let field_name = if T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::Colon.into(),
        ) {
            self.base.parse_name_node(false)?
        } else {
            selection_name.clone()
        };
        return Ok((field_name, selection_name));
    }
}
