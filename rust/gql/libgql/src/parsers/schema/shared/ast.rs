use std::{cell::RefCell, rc::Rc};

use crate::parsers::file;

pub struct Scalar {
    pub name: String,
}

pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, derive_more::From)]
pub enum InputTypeSpec {
    InputType(Rc<RefCell<InputType>>),
    Scalar(Rc<RefCell<Scalar>>),
    Enum(Rc<RefCell<Enum>>),
}

#[derive(Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
}

#[derive(Clone)]
pub enum ArrayLiteral {
    Int(Vec<i32>),
    Float(Vec<f32>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}

#[derive(Clone)]
pub struct LiteralFieldSpec<T> {
    pub r#type: T,
    pub default_value: Option<Literal>,
    pub directive_invocations: Vec<ServerDirectiveInvocation>,
}

#[derive(Clone)]
pub struct ArrayFieldSpec<T> {
    pub r#type: T,
    pub nullable: bool,
    pub default_value: Option<ArrayLiteral>,
    pub directive_invocations: Vec<ServerDirectiveInvocation>,
}

#[derive(Clone, derive_more::From)]
pub enum NonCallableFieldSpec<T> {
    Literal(LiteralFieldSpec<T>),
    Array(ArrayFieldSpec<T>),
}

pub type InputFieldSpec = NonCallableFieldSpec<InputTypeSpec>;

#[derive(Clone)]
pub struct FieldDefinition<T> {
    pub name: String,
    pub spec: T,
    pub nullable: bool,
}

pub struct InputType {
    pub name: String,
    pub fields: indexmap::IndexMap<String, FieldDefinition<InputFieldSpec>>,
}

#[derive(Clone)]
pub enum ArgumentLiteralValue {
    String(String),
    Int(i32),
    Float(f32),
    Boolean(bool),
    EnumValue(String),
}

impl From<i32> for ArgumentLiteralValue {
    fn from(value: i32) -> Self {
        return Self::Int(value);
    }
}

impl From<f32> for ArgumentLiteralValue {
    fn from(value: f32) -> Self {
        return Self::Float(value);
    }
}

impl From<bool> for ArgumentLiteralValue {
    fn from(value: bool) -> Self {
        return Self::Boolean(value);
    }
}

#[derive(Clone, derive_more::From)]
pub enum ArgumentValue {
    Ref(String),
    Literal(ArgumentLiteralValue),
}

#[derive(Clone)]
pub struct FieldSelectionArgument {
    pub name: String,
    pub value: ArgumentValue,
    pub r#type: FieldDefinition<InputFieldSpec>,
}

pub struct ServerDirective {
    pub name: String,
    pub arguments:
        indexmap::IndexMap<String, FieldDefinition<InputFieldSpec>>,
    pub locations: Vec<file::server::ast::DirectiveLocation>,
}

#[derive(Clone)]
pub struct ServerDirectiveInvocation {
    pub directive: Rc<RefCell<ServerDirective>>,
    pub arguments: indexmap::IndexMap<String, FieldSelectionArgument>,
}
