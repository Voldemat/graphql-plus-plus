use std::{path::PathBuf, sync::Arc};

use crate::lexer;

pub struct SourceFile<'buffer> {
    pub filepath: PathBuf,
    pub buffer: &'buffer str,
}

impl<'buffer> std::fmt::Debug for SourceFile<'buffer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceFile")
            .field("filepath", &self.filepath)
            .finish()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeLocation<'buffer> {
    pub start_token: lexer::tokens::Token<'buffer>,
    pub end_token: lexer::tokens::Token<'buffer>,
    #[serde(skip_serializing)]
    pub source: Arc<SourceFile<'buffer>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NameNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub name: &'buffer str,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralIntNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub value: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralFloatNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub value: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralStringNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralBooleanNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub value: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralEnumValueNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub value: String,
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
pub enum LiteralNode<'buffer> {
    Int(LiteralIntNode<'buffer>),
    Float(LiteralFloatNode<'buffer>),
    String(LiteralStringNode<'buffer>),
    Boolean(LiteralBooleanNode<'buffer>),
    EnumValue(LiteralEnumValueNode<'buffer>),
}

impl<'buffer> LiteralNode<'buffer> {
    pub fn get_location(self: &Self) -> &lexer::tokens::TokenLocation {
        match self {
            Self::Int(node) => &node.location.start_token.location,
            Self::Float(node) => &node.location.start_token.location,
            Self::String(node) => &node.location.start_token.location,
            Self::Boolean(node) => &node.location.start_token.location,
            Self::EnumValue(node) => &node.location.start_token.location,
        }
    }
    pub fn get_source_file(self: &Self) -> &Arc<SourceFile<'buffer>> {
        match self {
            Self::Int(node) => &node.location.source,
            Self::Float(node) => &node.location.source,
            Self::String(node) => &node.location.source,
            Self::Boolean(node) => &node.location.source,
            Self::EnumValue(node) => &node.location.source,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NamedTypeNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub name: NameNode<'buffer>,
    pub nullable: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ListTypeNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub r#type: Box<TypeNode<'buffer>>,
    pub nullable: bool,
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
pub enum TypeNode<'buffer> {
    Named(NamedTypeNode<'buffer>),
    List(ListTypeNode<'buffer>),
}

impl<'buffer> TypeNode<'buffer> {
    pub fn get_nullable(self: &Self) -> bool {
        match self {
            Self::Named(named) => named.nullable,
            Self::List(list) => list.nullable,
        }
    }
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
pub enum ArgumentValue<'buffer> {
    NameNode(NameNode<'buffer>),
    LiteralNode(LiteralNode<'buffer>),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Argument<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub name: NameNode<'buffer>,
    pub value: ArgumentValue<'buffer>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectiveInvocationNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub name: NameNode<'buffer>,
    pub arguments: Vec<Argument<'buffer>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct InputFieldDefinitionNode<'buffer> {
    pub location: NodeLocation<'buffer>,
    pub name: NameNode<'buffer>,
    pub r#type: TypeNode<'buffer>,
    pub default_value: Option<LiteralNode<'buffer>>,
    pub directives: Vec<DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct DirectiveLocationNode<'buffer, T> {
    pub location: NodeLocation<'buffer>,
    pub directive_location: T,
}

#[derive(Debug, serde::Serialize)]
pub struct DirectiveNode<'buffer, T: serde::Serialize> {
    pub location: NodeLocation<'buffer>,
    pub name: NameNode<'buffer>,
    pub targets: Vec<DirectiveLocationNode<'buffer, T>>,
    pub arguments: Vec<InputFieldDefinitionNode<'buffer>>,
}
