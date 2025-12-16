use std::{path::PathBuf, rc::Rc};

use crate::lexer;

pub struct SourceFile {
    filepath: PathBuf,
    buffer: String,
}

pub struct NodeLocation {
    pub start_token: lexer::tokens::Token,
    pub end_token: lexer::tokens::Token,
    pub source: Rc<SourceFile>,
}

pub struct NameNode {
    pub location: NodeLocation,
    pub name: String,
}

pub struct LiteralIntNode {
    pub location: NodeLocation,
    pub value: i32,
}

pub struct LiteralFloatNode {
    pub location: NodeLocation,
    pub value: f32,
}

pub struct LiteralStringNode {
    pub location: NodeLocation,
    pub value: String,
}

pub struct LiteralBooleanNode {
    pub location: NodeLocation,
    pub value: bool,
}

pub struct LiteralEnumValueNode {
    pub location: NodeLocation,
    pub value: String,
}

#[derive(derive_more::From)]
pub enum LiteralNode {
    Int(LiteralIntNode),
    Float(LiteralFloatNode),
    String(LiteralStringNode),
    Boolean(LiteralBooleanNode),
    EnumValue(LiteralEnumValueNode),
}

pub struct NamedTypeNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub nullable: bool,
}

pub struct ListTypeNode {
    pub location: NodeLocation,
    pub r#type: NamedTypeNode,
    pub nullable: bool,
}

#[derive(derive_more::From)]
pub enum TypeNode {
    Named(NamedTypeNode),
    List(ListTypeNode),
}

#[derive(derive_more::From)]
pub enum ArgumentValue {
    NameNode(NameNode),
    LiteralNode(LiteralNode),
}

pub struct Argument {
    pub location: NodeLocation,
    pub name: NameNode,
    pub value: ArgumentValue,
}

pub struct DirectiveInvocationNode {
    location: NodeLocation,
    name: NameNode,
    arguments: Vec<Argument>,
}

pub struct InputValueDefinitionNode {
    pub location: NodeLocation,
    pub name: NameNode,
    pub r#type: TypeNode,
    pub default_value: Option<LiteralNode>,
    pub directives: Vec<DirectiveInvocationNode>,
}

pub struct DirectiveLocationNode<T> {
    location: NodeLocation,
    directive_location: T,
}

pub struct DirectiveNode<T> {
    location: NodeLocation,
    name: NameNode,
    targets: Vec<DirectiveLocationNode<T>>,
    arguments: Vec<InputValueDefinitionNode>,
}
