use std::{path::PathBuf, rc::Rc};

use crate::lexer;

pub struct SourceFile {
    pub filepath: PathBuf,
    pub buffer: String,
}

impl std::fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceFile").field("filepath", &self.filepath).finish()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeLocation {
    pub start_token: lexer::tokens::Token,
    pub end_token: lexer::tokens::Token,
    #[serde(skip_serializing)]
    pub source: Rc<SourceFile>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NameNode {
    pub location: NodeLocation,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralIntNode {
    pub location: NodeLocation,
    pub value: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralFloatNode {
    pub location: NodeLocation,
    pub value: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralStringNode {
    pub location: NodeLocation,
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralBooleanNode {
    pub location: NodeLocation,
    pub value: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LiteralEnumValueNode {
    pub location: NodeLocation,
    pub value: String,
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
pub enum LiteralNode {
    Int(LiteralIntNode),
    Float(LiteralFloatNode),
    String(LiteralStringNode),
    Boolean(LiteralBooleanNode),
    EnumValue(LiteralEnumValueNode),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NamedTypeNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub nullable: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ListTypeNode {
    pub location: NodeLocation,
    pub r#type: NamedTypeNode,
    pub nullable: bool,
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
pub enum TypeNode {
    Named(NamedTypeNode),
    List(ListTypeNode),
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
pub enum ArgumentValue {
    NameNode(NameNode),
    LiteralNode(LiteralNode),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Argument {
    pub location: NodeLocation,
    pub name: NameNode,
    pub value: ArgumentValue,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectiveInvocationNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct InputFieldDefinitionNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub r#type: TypeNode,
    pub default_value: Option<LiteralNode>,
    pub directives: Vec<DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct DirectiveLocationNode<T> {
    pub location: NodeLocation,
    pub directive_location: T,
}

#[derive(Debug, serde::Serialize)]
pub struct DirectiveNode<T: serde::Serialize> {
    pub location: NodeLocation,
    pub name: NameNode,
    pub targets: Vec<DirectiveLocationNode<T>>,
    pub arguments: Vec<InputFieldDefinitionNode>,
}
