use std::{cell::RefCell, rc::Rc};

use crate::parsers::file;

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, derive_more::From)]
pub enum InputTypeSpec {
    InputType(Rc<RefCell<InputType>>),
    Scalar(String),
    Enum(Rc<Enum>),
}

impl std::fmt::Debug for InputTypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputType(arg0) => f
                .debug_tuple("InputType")
                .field(&arg0.borrow().name)
                .finish(),
            Self::Scalar(arg0) => f.debug_tuple("Scalar").field(arg0).finish(),
            Self::Enum(arg0) => {
                f.debug_tuple("Enum").field(&arg0.name).finish()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum ArrayLiteral {
    Int(Vec<i32>),
    Float(Vec<f32>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}

#[derive(Debug, Clone)]
pub struct LiteralFieldSpec<T> {
    pub r#type: T,
    pub default_value: Option<Option<Literal>>,
    pub directive_invocations:
        indexmap::IndexMap<String, ServerDirectiveInvocation>,
}

#[derive(Debug, Clone)]
pub struct ArrayFieldSpec<T> {
    pub r#type: T,
    pub nullable: bool,
    pub default_value: Option<Option<ArrayLiteral>>,
    pub directive_invocations: Vec<ServerDirectiveInvocation>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum NonCallableFieldSpec<T> {
    Literal(LiteralFieldSpec<T>),
    Array(ArrayFieldSpec<T>),
}

impl<T> NonCallableFieldSpec<T> {
    pub fn get_type_spec(self: &Self) -> &T {
        match self {
            Self::Literal(literal) => &literal.r#type,
            Self::Array(array) => &array.r#type,
        }
    }
}

pub type InputFieldSpec = NonCallableFieldSpec<InputTypeSpec>;

#[derive(Debug, Clone)]
pub struct FieldDefinition<T> {
    pub name: String,
    pub spec: T,
    pub nullable: bool,
}

#[derive(Debug)]
pub struct InputType {
    pub name: String,
    pub fields: indexmap::IndexMap<String, FieldDefinition<InputFieldSpec>>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, derive_more::From)]
pub enum ArgumentValue {
    Ref(String),
    Literal(ArgumentLiteralValue),
}

#[derive(Debug, Clone)]
pub struct FieldSelectionArgument {
    pub name: String,
    pub value: ArgumentValue,
    pub r#type: FieldDefinition<InputFieldSpec>,
}

#[derive(Debug)]
pub struct ServerDirective {
    pub name: String,
    pub arguments: indexmap::IndexMap<String, FieldDefinition<InputFieldSpec>>,
    pub locations: Vec<file::server::ast::DirectiveLocation>,
}

#[derive(Debug, Clone)]
pub struct ServerDirectiveInvocation {
    pub directive: Rc<RefCell<ServerDirective>>,
    pub arguments: indexmap::IndexMap<String, FieldSelectionArgument>,
}
