use std::{path::PathBuf, rc::Rc};

use crate::lexer;

pub struct SourceFile {
    filepath: PathBuf,
    buffer: String,
}

#[derive(Clone)]
pub struct NodeLocation {
    pub start_token: lexer::tokens::Token,
    pub end_token: lexer::tokens::Token,
    pub source: Rc<SourceFile>,
}

#[derive(Clone)]
pub struct NameNode {
    pub location: NodeLocation,
    pub name: String,
}

#[derive(Clone)]
pub struct LiteralIntNode {
    pub location: NodeLocation,
    pub value: i32,
}

#[derive(Clone)]
pub struct LiteralFloatNode {
    pub location: NodeLocation,
    pub value: f32,
}

#[derive(Clone)]
pub struct LiteralStringNode {
    pub location: NodeLocation,
    pub value: String,
}

#[derive(Clone)]
pub struct LiteralBooleanNode {
    pub location: NodeLocation,
    pub value: bool,
}

#[derive(Clone)]
pub struct LiteralEnumValueNode {
    pub location: NodeLocation,
    pub value: String,
}

#[derive(Clone, derive_more::From)]
pub enum LiteralNode {
    Int(LiteralIntNode),
    Float(LiteralFloatNode),
    String(LiteralStringNode),
    Boolean(LiteralBooleanNode),
    EnumValue(LiteralEnumValueNode),
}

#[derive(Clone)]
pub struct NamedTypeNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub nullable: bool,
}

#[derive(Clone)]
pub struct ListTypeNode {
    pub location: NodeLocation,
    pub r#type: NamedTypeNode,
    pub nullable: bool,
}

#[derive(Clone, derive_more::From)]
pub enum TypeNode {
    Named(NamedTypeNode),
    List(ListTypeNode),
}

#[derive(Clone, derive_more::From)]
pub enum ArgumentValue {
    NameNode(NameNode),
    LiteralNode(LiteralNode),
}

#[derive(Clone)]
pub struct Argument {
    pub location: NodeLocation,
    pub name: NameNode,
    pub value: ArgumentValue,
}

#[derive(Clone)]
pub struct DirectiveInvocationNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub arguments: Vec<Argument>,
}

#[derive(Clone)]
pub struct InputFieldDefinitionNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub r#type: TypeNode,
    pub default_value: Option<LiteralNode>,
    pub directives: Vec<DirectiveInvocationNode>,
}

pub struct DirectiveLocationNode<T> {
    pub location: NodeLocation,
    pub directive_location: T,
}

pub struct DirectiveNode<T> {
    pub location: NodeLocation,
    pub name: NameNode,
    pub targets: Vec<DirectiveLocationNode<T>>,
    pub arguments: Vec<InputFieldDefinitionNode>,
}
