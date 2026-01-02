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
pub enum Error {
    Base(base::Error),
    UnknownStartOfAstNode { token: lexer::tokens::Token },
}

impl From<tokens_source::ConsumeError> for Error {
    fn from(value: tokens_source::ConsumeError) -> Self {
        return Self::Base(value.into());
    }
}

impl Error {
    pub fn is_eof(self: &Self) -> bool {
        match self {
            Self::Base(error) => error.is_eof(),
            _ => false,
        }
    }

    pub fn get_location(self: &Self) -> lexer::tokens::Location {
        match self {
        Self::Base(b) => b.get_location(),
        Self::UnknownStartOfAstNode { token } => token.location.clone()
        }
    }
}

pub struct Parser<T: tokens_source::TokensSource> {
    base: BaseParser<T, ast::DirectiveLocation>,
}

impl<T: tokens_source::TokensSource> Parser<T> {
    pub fn new(tokens_source: T) -> Self {
        return Self {
            base: BaseParser::new(tokens_source),
        };
    }

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
            if let Some(e) = self.base.tokens_source.advance().err() {
                if e.is_eof() {
                    break 'l;
                }
                return Err(e.into());
            }
        }
        return Ok(nodes);
    }

    fn parse_ast_node(self: &mut Self) -> Result<ast::ASTNode, Error> {
        let current_token = T::get_current_token(&self.base.tokens_source);
        match current_token.lexeme.as_str() {
            "scalar" => Ok(self.parse_scalar_type_definition_node()?.into()),
            "union" => Ok(self.parse_union_type_definition_node()?.into()),
            "enum" => Ok(self.parse_enum_type_definition_node()?.into()),
            "interface" => {
                Ok(self.parse_interface_type_definition_node()?.into())
            }
            "type" => Ok(self.parse_object_type_definition_node()?.into()),
            "directive" => Ok(self.base.parse_directive_node()?.into()),
            "input" => Ok(self.parse_input_type_definition_node()?.into()),
            "extend" => Ok(self.parse_extend_type_node()?.into()),
            _ => {
                return Err(Error::UnknownStartOfAstNode {
                    token: current_token.clone(),
                });
            }
        }
    }

    fn parse_scalar_type_definition_node(
        self: &mut Self,
    ) -> Result<ast::ScalarDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        return Ok(ast::ScalarDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: name.location.end_token.clone(),
                source: name.location.source.clone(),
            },
            name,
        });
    }

    fn parse_union_type_definition_node(
        self: &mut Self,
    ) -> Result<ast::UnionDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::Equal.into(),
        )?;
        let mut values = vec![self.base.parse_name_node(false)?];
        while T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::Vslash.into(),
        ) {
            values.push(self.base.parse_name_node(false)?);
        }
        return Ok(ast::UnionDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: values.last().unwrap().location.end_token.clone(),
                source: name.location.source.clone(),
            },
            name,
            values,
            directives: Vec::new(),
        });
    }

    fn parse_extend_type_node(
        self: &mut Self,
    ) -> Result<ast::ExtendTypeNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        T::consume_identifier_by_lexeme(&mut self.base.tokens_source, "type")?;
        let type_node = self.parse_object_type_definition_node()?;
        return Ok(ast::ExtendTypeNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: T::get_source_file(&self.base.tokens_source),
            },
            type_node,
        });
    }

    fn parse_enum_type_definition_node(
        self: &mut Self,
    ) -> Result<ast::EnumDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        )?;
        let mut values = Vec::<ast::EnumValueDefinitionNode>::new();
        while T::is_ahead(
            &self.base.tokens_source,
            ComplexTokenType::Identifier.into(),
        ) {
            values.push(self.parse_enum_value_definition_node()?);
        }
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::RightBrace.into(),
        )?;
        return Ok(ast::EnumDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: name.location.source.clone(),
            },
            name,
            values,
            directives: Vec::new(),
        });
    }

    fn parse_enum_value_definition_node(
        self: &mut Self,
    ) -> Result<ast::EnumValueDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        return Ok(ast::EnumValueDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: name.location.end_token.clone(),
                source: name.location.source.clone(),
            },
            value: name,
            directives: Vec::new(),
        });
    }

    fn parse_interface_type_definition_node(
        self: &mut Self,
    ) -> Result<ast::InterfaceDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        )?;
        let mut fields = Vec::<ast::FieldDefinitionNode>::new();
        while T::is_ahead(
            &self.base.tokens_source,
            ComplexTokenType::Identifier.into(),
        ) {
            fields.push(self.parse_field_definition_node()?)
        }
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::RightBrace.into(),
        )?;
        return Ok(ast::InterfaceDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: name.location.source.clone(),
            },
            name,
            fields,
            directives: Vec::new(),
        });
    }

    fn parse_input_type_definition_node(
        self: &mut Self,
    ) -> Result<ast::InputObjectDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        )?;
        let mut fields = Vec::<shared::ast::InputFieldDefinitionNode>::new();
        while T::is_ahead(
            &self.base.tokens_source,
            ComplexTokenType::Identifier.into(),
        ) {
            fields.push(self.base.parse_input_field_definition_node()?)
        }
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::RightBrace.into(),
        )?;
        return Ok(ast::InputObjectDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: name.location.source.clone(),
            },
            name,
            fields,
            directives: Vec::new(),
        });
    }

    fn parse_field_definition_node(
        self: &mut Self,
    ) -> Result<ast::FieldDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        let arguments = self.base.parse_input_field_definition_nodes()?;
        T::consume(
            &mut self.base.tokens_source,
            SimpleTokenType::Colon.into(),
        )?;
        let type_node = self.base.parse_type_node()?;
        let _ = self.base.parse_default_value()?;
        let mut directives = Vec::<shared::ast::DirectiveInvocationNode>::new();
        while T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::AtSign.into(),
        ) {
            directives.push(self.base.parse_directive_invocation_node()?);
        }
        return Ok(ast::FieldDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: T::get_source_file(&self.base.tokens_source),
            },
            name,
            r#type: type_node,
            arguments,
            directives,
        });
    }

    fn parse_object_type_definition_node(
        self: &mut Self,
    ) -> Result<ast::ObjectDefinitionNode, base::Error> {
        let start_token =
            T::get_current_token(&self.base.tokens_source).clone();
        let name = self.base.parse_name_node(false)?;
        let interfaces = self.parse_implements_clause()?;
        let mut fields = Vec::<ast::FieldDefinitionNode>::new();
        if T::consume_if_is_ahead(
            &mut self.base.tokens_source,
            SimpleTokenType::LeftBrace.into(),
        ) {
            while T::is_ahead(
                &self.base.tokens_source,
                ComplexTokenType::Identifier.into(),
            ) {
                fields.push(self.parse_field_definition_node()?)
            }
            T::consume(
                &mut self.base.tokens_source,
                SimpleTokenType::RightBrace.into(),
            )?;
        }
        return Ok(ast::ObjectDefinitionNode {
            location: shared::ast::NodeLocation {
                start_token,
                end_token: T::get_current_token(&self.base.tokens_source)
                    .clone(),
                source: name.location.source.clone(),
            },
            name,
            interfaces,
            fields,
            directives: Vec::new(),
        });
    }

    fn parse_implements_clause(
        self: &mut Self,
    ) -> Result<Vec<shared::ast::NameNode>, base::Error> {
        let mut interfaces = Vec::<shared::ast::NameNode>::new();
        if T::consume_identifier_by_lexeme_if_is_ahead(
            &mut self.base.tokens_source,
            "implements",
        ) {
            interfaces.push(self.base.parse_name_node(false)?);
            while T::consume_if_is_ahead(
                &mut self.base.tokens_source,
                SimpleTokenType::Comma.into(),
            ) {
                interfaces.push(self.base.parse_name_node(false)?);
            }
        }
        return Ok(interfaces);
    }
}
