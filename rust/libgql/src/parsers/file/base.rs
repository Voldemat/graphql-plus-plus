use std::marker::PhantomData;

use crate::lexer;
use crate::lexer::token_type::ComplexTokenType;
use crate::lexer::token_type::SimpleTokenType;
use crate::lexer::token_type::TokenType;

use super::shared;
use super::tokens_source;

#[derive(Debug)]
pub enum Error<'buffer> {
    Consume(tokens_source::ConsumeError<'buffer>),
    IdentifierIsKeyword(lexer::tokens::Token<'buffer>),
    ExpectedComplexType(lexer::tokens::Token<'buffer>),
    CannotParseNumberLiteral(lexer::tokens::Token<'buffer>),
    UnexpectedSpreadInLiteral(lexer::tokens::Token<'buffer>),
    UnknownDirectiveLocation(lexer::tokens::Token<'buffer>),
    DuplicateDocumentationString {
        first: shared::ast::DocumentationNode<'buffer>,
        second: lexer::tokens::Token<'buffer>,
    },
}

impl<'buffer> std::fmt::Display for Error<'buffer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Consume(error) => error.fmt(f),
            Self::IdentifierIsKeyword(_) => {
                f.write_str("Identifier is reserved keyword")
            }
            Self::ExpectedComplexType(token) => f.write_fmt(format_args!(
                "Expected complex token, but received: {:?}",
                token.token_type
            )),
            Self::CannotParseNumberLiteral(_) => {
                f.write_str("Cannot parse number literal")
            }
            Self::UnexpectedSpreadInLiteral(_) => {
                f.write_str("Unexpected spread in literal")
            }
            Self::UnknownDirectiveLocation(_) => {
                f.write_str("Unknown directive location")
            }
            Self::DuplicateDocumentationString { .. } => {
                f.write_str("Duplicate documentation string")
            }
        }
    }
}

impl<'buffer> Error<'buffer> {
    pub fn is_eof(self: &Self) -> bool {
        match self {
            Self::Consume(error) => error.is_eof(),
            _ => false,
        }
    }

    pub fn get_location(self: &Self) -> &lexer::tokens::TokenLocation {
        match self {
            Self::Consume(e) => e.get_location(),
            Self::IdentifierIsKeyword(token) => &token.location,
            Self::ExpectedComplexType(token) => &token.location,
            Self::CannotParseNumberLiteral(token) => &token.location,
            Self::UnexpectedSpreadInLiteral(token) => &token.location,
            Self::UnknownDirectiveLocation(token) => &token.location,
            Self::DuplicateDocumentationString { second, .. } => {
                &second.location
            }
        }
    }
}

impl<'buffer> From<tokens_source::ConsumeError<'buffer>> for Error<'buffer> {
    fn from(value: tokens_source::ConsumeError<'buffer>) -> Self {
        return Self::Consume(value);
    }
}

pub struct BaseParser<
    'buffer,
    T: tokens_source::TokensSource<'buffer>,
    TDirectiveLocation: for<'a> TryFrom<&'a str> + serde::Serialize,
> {
    pub tokens_source: T,
    pub documentation_node: Option<shared::ast::DocumentationNode<'buffer>>,
    _v: PhantomData<TDirectiveLocation>,
    _y: PhantomData<&'buffer ()>,
}

impl<
    'buffer,
    T: tokens_source::TokensSource<'buffer>,
    TDirectiveLocation: for<'a> TryFrom<&'a str> + serde::Serialize,
> BaseParser<'buffer, T, TDirectiveLocation>
{
    pub fn new(tokens_source: T) -> Self {
        return Self {
            tokens_source,
            documentation_node: None,
            _v: PhantomData::default(),
            _y: PhantomData::default(),
        };
    }

    pub fn process_potential_documentation_string(
        self: &mut Self,
        consume: bool,
    ) -> Result<(), Error<'buffer>> {
        let current_token = T::get_current_token(&self.tokens_source);
        if current_token.token_type
            == TokenType::Complex(ComplexTokenType::String)
            && T::lookback(&self.tokens_source)
                .map(|prev_token| {
                    prev_token.token_type != SimpleTokenType::Equal.into()
                })
                .unwrap_or(true)
        {
            if let Some(current) = self.documentation_node.as_ref() {
                return Err(Error::DuplicateDocumentationString {
                    first: current.clone(),
                    second: current_token.clone(),
                });
            };
            self.documentation_node = Some(shared::ast::DocumentationNode {
                location: shared::ast::NodeLocation {
                    start: current_token.location.start,
                    end: current_token.location.end,
                    source: T::get_source_file(&self.tokens_source),
                },
                string: current_token.lexeme,
            });
            if consume {
                self.tokens_source.advance()?;
            }
        }
        Ok(())
    }

    pub fn parse_name_node(
        self: &mut Self,
        err_on_keyword: bool,
    ) -> Result<shared::ast::NameNode<'buffer>, Error<'buffer>> {
        let token = T::consume_identifier(&mut self.tokens_source)?;
        if err_on_keyword && token.is_keyword() {
            return Err(Error::IdentifierIsKeyword(token.clone()));
        }
        let name = token.lexeme;
        return Ok(shared::ast::NameNode::<'buffer> {
            location: shared::ast::NodeLocation {
                start: token.location.start,
                end: token.location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            name,
        });
    }

    pub fn parse_type_node(
        self: &mut Self,
    ) -> Result<shared::ast::TypeNode<'buffer>, Error<'buffer>> {
        if T::is_ahead(&self.tokens_source, SimpleTokenType::LeftBracket.into())
        {
            return self.parse_list_type_node().map(|v| v.into());
        }
        return self.parse_named_type_node().map(|v| v.into());
    }

    fn parse_named_type_node(
        self: &mut Self,
    ) -> Result<shared::ast::NamedTypeNode<'buffer>, Error<'buffer>> {
        let name_node = self.parse_name_node(false)?;
        let nullable = !T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Bang.into(),
        );
        return Ok(shared::ast::NamedTypeNode {
            location: shared::ast::NodeLocation {
                start: name_node.location.start,
                end: T::get_current_token(&self.tokens_source).location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            name: name_node,
            nullable,
        });
    }

    fn parse_list_type_node(
        self: &mut Self,
    ) -> Result<shared::ast::ListTypeNode<'buffer>, Error<'buffer>> {
        let start = T::consume(
            &mut self.tokens_source,
            SimpleTokenType::LeftBracket.into(),
        )?
        .location
        .start;
        let type_node = self.parse_type_node()?;
        T::consume(
            &mut self.tokens_source,
            SimpleTokenType::RightBracket.into(),
        )?;
        let nullable = !T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Bang.into(),
        );
        return Ok(shared::ast::ListTypeNode {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.tokens_source).location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            r#type: Box::new(type_node),
            nullable,
        });
    }

    pub fn parse_input_field_definition_node(
        self: &mut Self,
    ) -> Result<shared::ast::InputFieldDefinitionNode<'buffer>, Error<'buffer>>
    {
        self.process_potential_documentation_string(false)?;
        let documentation = self.documentation_node.take();
        let name_node = self.parse_name_node(false)?;
        let start = match &documentation {
            Some(d) => d.location.start,
            None => name_node.location.start,
        };
        T::consume(&mut self.tokens_source, SimpleTokenType::Colon.into())?;
        let type_node = self.parse_type_node()?;
        let default_value = self.parse_default_value()?;
        return Ok(shared::ast::InputFieldDefinitionNode {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.tokens_source).location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            documentation,
            name: name_node,
            r#type: type_node,
            default_value,
            directives: Vec::new(),
        });
    }

    pub fn parse_input_field_definition_nodes(
        self: &mut Self,
    ) -> Result<
        Vec<shared::ast::InputFieldDefinitionNode<'buffer>>,
        Error<'buffer>,
    > {
        let mut arguments =
            Vec::<shared::ast::InputFieldDefinitionNode<'buffer>>::new();
        if T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            T::consume_if_is_ahead(
                &mut self.tokens_source,
                ComplexTokenType::String.into(),
            );
            while T::is_ahead(
                &self.tokens_source,
                ComplexTokenType::Identifier.into(),
            ) {
                arguments.push(self.parse_input_field_definition_node()?);
                T::consume_if_is_ahead(
                    &mut self.tokens_source,
                    SimpleTokenType::Comma.into(),
                );
                T::consume_if_is_ahead(
                    &mut self.tokens_source,
                    ComplexTokenType::String.into(),
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
    ) -> Result<Option<shared::ast::LiteralNode<'buffer>>, Error<'buffer>> {
        T::advance(&mut self.tokens_source)?;
        let current_token = T::get_current_token(&self.tokens_source).clone();
        let TokenType::Complex(token_type) = current_token.token_type else {
            return Err(Error::ExpectedComplexType(current_token));
        };
        let location = shared::ast::NodeLocation {
            start: current_token.location.start,
            end: current_token.location.end,
            source: T::get_source_file(&self.tokens_source),
        };
        match token_type {
            ComplexTokenType::Number => {
                if let Some(int_node) = self.parse_literal_int_node() {
                    Ok(Some(int_node.into()))
                } else if let Some(float_node) = self.parse_literal_float_node()
                {
                    Ok(Some(float_node.into()))
                } else {
                    Err(Error::CannotParseNumberLiteral(current_token))
                }
            }
            ComplexTokenType::Boolean => Ok(Some(
                shared::ast::LiteralBooleanNode {
                    location,
                    value: current_token.lexeme == "true",
                }
                .into(),
            )),
            ComplexTokenType::String => Ok(Some(
                shared::ast::LiteralStringNode {
                    location,
                    value: current_token.lexeme,
                }
                .into(),
            )),
            ComplexTokenType::Identifier => {
                if current_token.lexeme == "null" {
                    Ok(None)
                } else {
                    Ok(Some(
                        shared::ast::LiteralEnumValueNode {
                            location,
                            value: current_token.lexeme,
                        }
                        .into(),
                    ))
                }
            }
            ComplexTokenType::Spread => {
                Err(Error::UnexpectedSpreadInLiteral(current_token.clone()))
            }
        }
    }

    fn parse_literal_int_node(
        self: &mut Self,
    ) -> Option<shared::ast::LiteralIntNode<'buffer>> {
        let current_token = T::get_current_token(&self.tokens_source);
        let value = current_token.lexeme.parse::<i64>().ok()?;
        return Some(shared::ast::LiteralIntNode {
            location: shared::ast::NodeLocation {
                start: current_token.location.start,
                end: current_token.location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            value,
        });
    }

    fn parse_literal_float_node(
        self: &Self,
    ) -> Option<shared::ast::LiteralFloatNode<'buffer>> {
        let current_token = T::get_current_token(&self.tokens_source);
        let value = current_token.lexeme.parse::<f64>().ok()?;
        return Some(shared::ast::LiteralFloatNode {
            location: shared::ast::NodeLocation {
                start: current_token.location.start,
                end: current_token.location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            value,
        });
    }

    pub fn parse_arguments(
        self: &mut Self,
    ) -> Result<Vec<shared::ast::Argument<'buffer>>, Error<'buffer>> {
        let mut arguments = Vec::<shared::ast::Argument<'buffer>>::new();
        if T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::LeftParen.into(),
        ) {
            while T::is_ahead(
                &self.tokens_source,
                ComplexTokenType::Identifier.into(),
            ) {
                if let Some(argument) = self.parse_argument()? {
                    arguments.push(argument);
                }
                T::consume_if_is_ahead(
                    &mut self.tokens_source,
                    SimpleTokenType::Comma.into(),
                );
            }
            T::consume(
                &mut self.tokens_source,
                SimpleTokenType::RightParen.into(),
            )?;
        }
        return Ok(arguments);
    }

    fn parse_argument(
        self: &mut Self,
    ) -> Result<Option<shared::ast::Argument<'buffer>>, Error<'buffer>> {
        let name = self.parse_name_node(false)?;
        T::consume(&mut self.tokens_source, SimpleTokenType::Colon.into())?;
        let Some(value) = self.parse_argument_value()? else {
            return Ok(None);
        };
        return Ok(Some(shared::ast::Argument {
            location: shared::ast::NodeLocation {
                start: name.location.start,
                end: T::get_current_token(&self.tokens_source).location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            name,
            value,
        }));
    }

    fn parse_argument_value(
        self: &mut Self,
    ) -> Result<Option<shared::ast::ArgumentValue<'buffer>>, Error<'buffer>>
    {
        let Some(token) = T::lookahead(&self.tokens_source) else {
            return Err(tokens_source::ConsumeError::EOF(
                T::get_current_token(&self.tokens_source).clone(),
            )
            .into());
        };
        if token.token_type == ComplexTokenType::Identifier.into() {
            return self.parse_name_node(false).map(|v| Some(v.into()));
        }
        return self.parse_literal_node().map(|v| v.map(|i| i.into()));
    }

    pub fn parse_default_value(
        self: &mut Self,
    ) -> Result<Option<shared::ast::LiteralNode<'buffer>>, Error<'buffer>> {
        if T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Equal.into(),
        ) {
            return Ok(self.parse_literal_node()?);
        }
        return Ok(None);
    }

    fn parse_directive_location_node(
        self: &mut Self,
    ) -> Result<
        shared::ast::DirectiveLocationNode<'buffer, TDirectiveLocation>,
        Error<'buffer>,
    > {
        let directive_location = self.parse_directive_location()?;
        let current_token = T::get_current_token(&self.tokens_source);
        return Ok(shared::ast::DirectiveLocationNode::<TDirectiveLocation> {
            location: shared::ast::NodeLocation {
                start: current_token.location.start,
                end: current_token.location.end,
                source: T::get_source_file(&self.tokens_source),
            },
            directive_location,
        });
    }

    fn parse_directive_locations(
        self: &mut Self,
    ) -> Result<
        Vec<shared::ast::DirectiveLocationNode<'buffer, TDirectiveLocation>>,
        Error<'buffer>,
    > {
        let mut locations = vec![self.parse_directive_location_node()?];
        while T::consume_if_is_ahead(
            &mut self.tokens_source,
            SimpleTokenType::Comma.into(),
        ) {
            locations.push(self.parse_directive_location_node()?);
        }
        return Ok(locations);
    }

    pub fn parse_directive_node(
        self: &mut Self,
    ) -> Result<
        shared::ast::DirectiveNode<'buffer, TDirectiveLocation>,
        Error<'buffer>,
    > {
        let documentation = self.documentation_node.take();
        T::consume(&mut self.tokens_source, SimpleTokenType::AtSign.into())?;
        let name_node = self.parse_name_node(false)?;
        let start = match &documentation {
            Some(d) => d.location.start,
            None => name_node.location.start,
        };
        let arguments = self.parse_input_field_definition_nodes()?;
        T::consume_identifier_by_lexeme(&mut self.tokens_source, "on")?;
        let locations = self.parse_directive_locations()?;
        return Ok(shared::ast::DirectiveNode::<'buffer, TDirectiveLocation> {
            location: shared::ast::NodeLocation {
                start,
                end: locations.last().unwrap().location.end,
                source: name_node.location.source.clone(),
            },
            documentation,
            name: name_node,
            targets: locations,
            arguments,
        });
    }

    pub fn parse_directive_invocation_node(
        self: &mut Self,
    ) -> Result<shared::ast::DirectiveInvocationNode<'buffer>, Error<'buffer>>
    {
        let start = T::get_current_token(&self.tokens_source).location.start;
        let name = self.parse_name_node(false)?;
        let arguments = self.parse_arguments()?;
        return Ok(shared::ast::DirectiveInvocationNode {
            location: shared::ast::NodeLocation {
                start,
                end: T::get_current_token(&self.tokens_source).location.end,
                source: T::get_source_file(&self.tokens_source).clone(),
            },
            name,
            arguments,
        });
    }

    pub fn parse_directive_location(
        self: &mut Self,
    ) -> Result<TDirectiveLocation, Error<'buffer>> {
        let token = T::consume_identifier(&mut self.tokens_source)?;
        let Ok(value) = TDirectiveLocation::try_from(token.lexeme) else {
            return Err(Error::UnknownDirectiveLocation(token.clone()));
        };
        return Ok(value);
    }
}
