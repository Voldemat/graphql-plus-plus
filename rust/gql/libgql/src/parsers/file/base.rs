use crate::lexer;
use crate::lexer::token_type::ComplexTokenType;
use crate::lexer::token_type::SimpleTokenType;
use crate::lexer::token_type::TokenType;

use super::shared;
use super::tokens_source;

pub enum Error {
    Consume(tokens_source::ConsumeError),
    IdentifierIsKeyword { token: lexer::tokens::Token },
    ExpectedComplexType { token: lexer::tokens::Token },
    CannotParseNumberLiteral { token: lexer::tokens::Token },
    UnexpectedSpreadInLitearl { token: lexer::tokens::Token },
}

impl From<tokens_source::ConsumeError> for Error {
    fn from(value: tokens_source::ConsumeError) -> Self {
        return Self::Consume(value);
    }
}

pub struct BaseParser<T: tokens_source::TokensSource> {
    tokens_source: T,
}

impl<T: tokens_source::TokensSource> BaseParser<T> {
    fn parse_name_node(
        self: &mut Self,
        err_on_keyword: bool,
    ) -> Result<shared::ast::NameNode, Error> {
        let token = T::consume_identifier(&mut self.tokens_source)?;
        if err_on_keyword && token.is_keyword() {
            return Err(Error::IdentifierIsKeyword {
                token: token.clone(),
            });
        }
        let name = token.lexeme.clone();
        return Ok(shared::ast::NameNode {
            location: shared::ast::NodeLocation {
                start_token: token.clone(),
                end_token: token.clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            name,
        });
    }

    fn parse_type_node(
        self: &mut Self,
    ) -> Result<shared::ast::TypeNode, Error> {
        if T::is_ahead(&self.tokens_source, SimpleTokenType::LeftBracket.into())
        {
            return self.parse_list_type_node().map(|v| v.into());
        }
        return self.parse_named_type_node().map(|v| v.into());
    }

    fn parse_named_type_node(
        self: &mut Self,
    ) -> Result<shared::ast::NamedTypeNode, Error> {
        let name_node = self.parse_name_node(false)?;
        let nullable = T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Bang.into(),
        );
        return Ok(shared::ast::NamedTypeNode {
            location: shared::ast::NodeLocation {
                start_token: name_node.location.start_token.clone(),
                end_token: T::get_current_token(&self.tokens_source).clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            name: name_node,
            nullable,
        });
    }

    fn parse_list_type_node(
        self: &mut Self,
    ) -> Result<shared::ast::ListTypeNode, Error> {
        let start_token = T::consume(
            &mut self.tokens_source,
            SimpleTokenType::LeftBracket.into(),
        )?
        .clone();
        let type_node = self.parse_named_type_node()?;
        T::consume(
            &mut self.tokens_source,
            SimpleTokenType::RightBracket.into(),
        )?;
        let nullable = T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Bang.into(),
        );
        return Ok(shared::ast::ListTypeNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.tokens_source).clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            r#type: type_node,
            nullable,
        });
    }

    fn parse_input_value_definition_node(
        self: &mut Self,
    ) -> Result<shared::ast::InputValueDefinitionNode, Error> {
        let name_node = self.parse_name_node(false)?;
        let start_token = T::get_current_token(&self.tokens_source).clone();
        T::consume(&mut self.tokens_source, SimpleTokenType::Colon.into())?;
        let type_node = self.parse_type_node()?;
        let mut default_value: Option<shared::ast::LiteralNode> = None;
        if T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Equal.into(),
        ) {
            default_value = Some(self.parse_literal_node()?);
        }
        return Ok(shared::ast::InputValueDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token: start_token,
                end_token: T::get_current_token(&self.tokens_source).clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            name: name_node,
            r#type: type_node,
            default_value,
            directives: Vec::new(),
        });
    }

    fn parse_input_value_definition_nodes(
        self: &mut Self,
    ) -> Result<Vec<shared::ast::InputValueDefinitionNode>, Error> {
        let mut arguments = Vec::<shared::ast::InputValueDefinitionNode>::new();
        if T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            while T::is_ahead(
                &self.tokens_source,
                ComplexTokenType::Identifier.into(),
            ) {
                arguments.push(self.parse_input_value_definition_node()?);
                T::consume_if_is_ahead(
                    &mut self.tokens_source,
                    SimpleTokenType::Comma.into(),
                );
            }
            T::consume(
                &mut self.tokens_source,
                SimpleTokenType::RightParen.into(),
            )?;
        };
        return Ok(arguments);
    }

    fn parse_literal_node(
        self: &mut Self,
    ) -> Result<shared::ast::LiteralNode, Error> {
        T::advance(&mut self.tokens_source);
        let current_token = T::get_current_token(&self.tokens_source).clone();
        let TokenType::Complex(token_type) = current_token.token_type else {
            return Err(Error::ExpectedComplexType {
                token: current_token,
            }
            .into());
        };
        let location = shared::ast::NodeLocation {
            start_token: current_token.clone(),
            end_token: current_token.clone(),
            source: T::get_source_file(&self.tokens_source),
        };
        match token_type {
            ComplexTokenType::Number => {
                if let Some(int_node) = self.parse_literal_int_node() {
                    return Ok(int_node.into());
                };
                if let Some(float_node) = self.parse_literal_float_node() {
                    return Ok(float_node.into());
                };
                return Err(Error::CannotParseNumberLiteral {
                    token: current_token,
                });
            }
            ComplexTokenType::Boolean => {
                return Ok(shared::ast::LiteralBooleanNode {
                    location,
                    value: current_token.lexeme == "true",
                }
                .into());
            }
            ComplexTokenType::String => {
                return Ok(shared::ast::LiteralStringNode {
                    location,
                    value: current_token.lexeme.clone(),
                }
                .into());
            }
            ComplexTokenType::Identifier => {
                return Ok(shared::ast::LiteralEnumValueNode {
                    location,
                    value: current_token.lexeme.clone(),
                }
                .into());
            }
            ComplexTokenType::Spread => {
                return Err(Error::UnexpectedSpreadInLitearl {
                    token: current_token.clone(),
                });
            }
        }
    }

    fn parse_literal_int_node(
        self: &mut Self,
    ) -> Option<shared::ast::LiteralIntNode> {
        let current_token = T::get_current_token(&self.tokens_source);
        let value = current_token.lexeme.parse::<i32>().ok()?;
        return Some(shared::ast::LiteralIntNode {
            location: shared::ast::NodeLocation {
                start_token: current_token.clone(),
                end_token: current_token.clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            value,
        });
    }

    fn parse_literal_float_node(
        self: &Self,
    ) -> Option<shared::ast::LiteralFloatNode> {
        let current_token = T::get_current_token(&self.tokens_source);
        let value = current_token.lexeme.parse::<f32>().ok()?;
        return Some(shared::ast::LiteralFloatNode {
            location: shared::ast::NodeLocation {
                start_token: current_token.clone(),
                end_token: current_token.clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            value,
        });
    }

    fn parse_arguments(
        self: &mut Self,
    ) -> Result<Vec<shared::ast::Argument>, Error> {
        let mut arguments = Vec::<shared::ast::Argument>::new();
        while T::is_ahead(
            &self.tokens_source,
            ComplexTokenType::Identifier.into(),
        ) {
            arguments.push(self.parse_argument()?);
            T::consume_if_is_ahead(
                &mut self.tokens_source,
                SimpleTokenType::Comma.into(),
            );
        }
        return Ok(arguments);
    }

    fn parse_argument(self: &mut Self) -> Result<shared::ast::Argument, Error> {
        let name = self.parse_name_node(false)?;
        T::consume(&mut self.tokens_source, SimpleTokenType::Colon.into())?;
        let value = self.parse_argument_value()?;
        return Ok(shared::ast::Argument {
            location: shared::ast::NodeLocation {
                start_token: name.location.start_token.clone(),
                end_token: T::get_current_token(&self.tokens_source).clone(),
                source: T::get_source_file(&self.tokens_source),
            },
            name,
            value,
        });
    }

    fn parse_argument_value(
        self: &mut Self,
    ) -> Result<shared::ast::ArgumentValue, Error> {
        let Some(token) = T::lookahead(&self.tokens_source) else {
            return Err(tokens_source::ConsumeError::EOF {
                token: T::get_current_token(&self.tokens_source).clone(),
            }
            .into());
        };
        if token.token_type == ComplexTokenType::Identifier.into() {
            return self.parse_name_node(false).map(|v| v.into());
        }
        return self.parse_literal_node().map(|v| v.into());
    }
}
