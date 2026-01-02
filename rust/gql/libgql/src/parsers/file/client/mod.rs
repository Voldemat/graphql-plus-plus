use std::rc::Rc;

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

#[derive(derive_more::From)]
pub enum Error {
    Base(base::Error),
    UnexpectedOpType {
        token: lexer::tokens::Token,
    },
    DuplicateParameter {
        existing_parameter: shared::ast::InputFieldDefinitionNode,
        duplicate_parameter: shared::ast::InputFieldDefinitionNode,
    },
}

impl Error {
    pub fn is_eof(self: &Self) -> bool {
        match self {
            Error::Base(error) => error.is_eof(),
            _ => false,
        }
    }
}

impl From<tokens_source::ConsumeError> for Error {
    fn from(value: tokens_source::ConsumeError) -> Self {
        return Self::Base(value.into());
    }
}

pub struct Parser<T: tokens_source::TokensSource> {
    base: BaseParser<T, ast::DirectiveLocation>,
}

impl<T: tokens_source::TokensSource> Parser<T> {
    pub fn parse_ast_nodes(
        self: &mut Self,
    ) -> Result<Vec<ast::ASTNode>, Error> {
        let mut nodes = Vec::<ast::ASTNode>::new();
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
        }
        return Ok(nodes);
    }

    pub fn parse_ast_node(self: &mut Self) -> Result<ast::ASTNode, Error> {
        let token = T::get_current_token(&self.base.tokens_source);
        match token.lexeme.as_str() {
            "fragment" => Ok(self.parse_fragment_definition()?.into()),
            "directive" => Ok(self.base.parse_directive_node()?.into()),
            _ => Ok(self.parse_operation_definition()?.into()),
        }
    }

    fn parse_fragment_definition(
        self: &mut Self,
    ) -> Result<ast::FragmentDefinition, Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        T::consume_identifier_by_lexeme(&mut self.base.tokens_source, "on")?;
        let type_name = self.base.parse_name_node(false)?;
        let spec = self.parse_fragment_spec()?;
        return Ok(ast::FragmentDefinition {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: T::get_source_file(&self.base.tokens_source),
            },
            name,
            type_name,
            spec,
        });
    }

    fn parse_operation_definition(
        self: &mut Self,
    ) -> Result<ast::OperationDefinition, Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let Ok(optype) = ast::OpType::try_from(start_token.lexeme.as_str())
        else {
            return Err(Error::UnexpectedOpType { token: start_token });
        };
        let name = self.base.parse_name_node(false)?;
        let parameters = self.parse_operation_parameters()?;
        let fragment = self.parse_fragment_spec()?;
        return Ok(ast::OperationDefinition {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
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
        Vec<shared::ast::InputFieldDefinitionNode>,
        Error,
    > {
        let mut parameters = Vec::<shared::ast::InputFieldDefinitionNode>::new();
        if T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            while T::is_ahead(
                &self.base.tokens_source,
                ComplexTokenType::Identifier.into(),
            ) {
                let node = self.base.parse_input_field_definition_node()?;
                if let Some(existing_parameter) = parameters.iter().find(|p| p.name.name == node.name.name) {
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
    ) -> Result<ast::FragmentSpec, Error> {
        let start_token = T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        )?
        .clone();
        let selections = self.parse_selection_nodes()?;
        let end_token = T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::RightBrace.into(),
        )?
        .clone();
        return Ok(ast::FragmentSpec {
            location: shared::ast::NodeLocation {
                start_token,
                end_token,
                source: T::get_source_file(&self.base.tokens_source),
            },
            selections,
        });
    }

    fn parse_selection_nodes(
        self: &mut Self,
    ) -> Result<Vec<ast::SelectionNode>, Error> {
        let mut selections = Vec::<ast::SelectionNode>::new();
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
    ) -> Result<ast::SelectionNode, Error> {
        if !T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            ComplexTokenType::Spread.into(),
        ) {
            return Ok(self.parse_field_selection_node()?.into());
        }
        if T::is_ahead_by_lexeme(&self.base.tokens_source, "on") {
            return Ok(self.parse_conditional_spread_selection_node()?.into());
        }
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let fragment_name = self.base.parse_name_node(false)?;
        return Ok(ast::SpreadSelectionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: T::get_source_file(&self.base.tokens_source),
            },
            fragment_name,
        }
        .into());
    }

    fn parse_field_selection_node(
        self: &mut Self,
    ) -> Result<ast::FieldSelectionNode, Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let field_spec = self.parse_object_field_spec()?;
        let mut spec: Option<Rc<ast::FragmentSpec>> = None;
        if T::is_ahead(
            &self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        ) {
            spec = Some(Rc::new(self.parse_fragment_spec()?));
        }
        return Ok(ast::FieldSelectionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: T::get_source_file(&self.base.tokens_source),
            },
            field: field_spec,
            spec,
        });
    }

    fn parse_conditional_spread_selection_node(
        self: &mut Self,
    ) -> Result<ast::ConditionalSpreadSelectionNode, Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        T::consume_identifier_by_lexeme(&mut self.base.tokens_source, "on")?;
        let type_name = self.base.parse_name_node(false)?;
        let fragment_spec = self.parse_fragment_spec()?;
        return Ok(ast::ConditionalSpreadSelectionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: T::get_source_file(&self.base.tokens_source),
            },
            type_name,
            fragment: Rc::new(fragment_spec),
        });
    }

    fn parse_object_field_spec(
        self: &mut Self,
    ) -> Result<ast::ObjectFieldSpec, Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let (name, selection_name) = self.parse_name_and_selection_name()?;
        if !T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            return Ok(ast::ObjectLiteralFieldSpec {
                location: shared::ast::NodeLocation {
                    start_token,
                    end_token: T::get_current_token(&self.base.tokens_source)
                        .clone(),
                    source: T::get_source_file(&self.base.tokens_source),
                },
                selection_name,
                name,
            }
            .into());
        }
        let arguments = self.base.parse_arguments()?;
        let end_token = T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::RightParen.into(),
        )?
        .clone();
        return Ok(ast::ObjectCallableFieldSpec {
            location: shared::ast::NodeLocation {
                start_token,
                end_token,
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
    ) -> Result<(shared::ast::NameNode, shared::ast::NameNode), Error> {
        let selection_name = self.base.parse_name_node(false)?;
        let field_name = if T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::Colon.into(),
        ) {
            self.base.parse_name_node(false)?
        } else {
            selection_name.clone()
        };
        return Ok((selection_name, field_name));
    }
}
