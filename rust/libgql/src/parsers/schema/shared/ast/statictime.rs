use crate::parsers::file;

use super::AsStr;

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: &'static str,
    pub values: &'static [&'static str],
}

impl super::traits::Enum for Enum {
    fn get_name(self: &Self) -> &str {
        &self.name
    }

    fn has_value(self: &Self, value: &str) -> bool {
        self.values.iter().any(|v| *v == value)
    }
}

#[derive(Debug)]
pub enum InputTypeSpec {
    InputType(&'static str),
    Scalar(&'static str),
    Enum(&'static str),
}

impl super::traits::InputTypeSpec for InputTypeSpec {
    fn get_ref(self: &Self) -> super::traits::InputTypeSpecRef<'static> {
        match self {
            Self::InputType(i) => super::traits::InputTypeSpecRef::InputType(i),
            Self::Scalar(s) => super::traits::InputTypeSpecRef::Scalar(s),
            Self::Enum(e) => super::traits::InputTypeSpecRef::Enum(e),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(&'static str),
    Boolean(bool),
}

impl super::traits::Literal for Literal {
    fn get_ref(self: &Self) -> super::traits::LiteralRef<'_> {
        match self {
            Self::Int(i) => super::traits::LiteralRef::Int(i),
            Self::Float(f) => super::traits::LiteralRef::Float(f),
            Self::Boolean(b) => super::traits::LiteralRef::Boolean(b),
            Self::String(s) => super::traits::LiteralRef::String(s),
        }
    }
}

impl Literal {
    pub fn parse(node: &file::shared::ast::LiteralNode<'static>) -> Self {
        match node {
            file::shared::ast::LiteralNode::Int(i) => Self::Int(i.value),
            file::shared::ast::LiteralNode::Float(i) => Self::Float(i.value),
            file::shared::ast::LiteralNode::Boolean(i) => {
                Self::Boolean(i.value)
            }
            file::shared::ast::LiteralNode::String(i) => Self::String(i.value),
            file::shared::ast::LiteralNode::EnumValue(i) => {
                Self::String(i.value)
            }
        }
    }
}

#[derive(Debug)]
pub enum ArrayLiteral {
    Int(&'static [i64]),
    Float(&'static [f64]),
    String(&'static [&'static str]),
    Boolean(&'static [bool]),
}

impl super::traits::ArrayLiteral for ArrayLiteral {
    fn get_ref(
        self: &Self,
    ) -> super::traits::ArrayLiteralRef<'_, impl AsStr<'_>> {
        match self {
            Self::Int(i) => super::traits::ArrayLiteralRef::Int(i),
            Self::Float(f) => super::traits::ArrayLiteralRef::Float(f),
            Self::Boolean(b) => super::traits::ArrayLiteralRef::Boolean(b),
            Self::String(s) => super::traits::ArrayLiteralRef::String(s),
        }
    }
}

#[derive(Debug)]
pub struct FieldDefinition<T> {
    pub name: &'static str,
    pub spec: T,
    pub nullable: bool,
}

#[derive(Debug)]
pub struct LiteralFieldSpec<T> {
    pub r#type: T,
    pub default_value: Option<Option<Literal>>,
    pub directive_invocations:
        phf::OrderedMap<&'static str, ServerDirectiveInvocation>,
}

#[derive(Debug)]
pub struct ArrayFieldSpec<T> {
    pub r#type: Box<NonCallableFieldSpec<T>>,
    pub nullable: bool,
    pub default_value: Option<Option<ArrayLiteral>>,
    pub directive_invocations: Vec<ServerDirectiveInvocation>,
}

#[derive(Debug, derive_more::From)]
pub enum NonCallableFieldSpec<T> {
    Literal(LiteralFieldSpec<T>),
    Array(ArrayFieldSpec<T>),
}

pub type InputFieldSpec = NonCallableFieldSpec<InputTypeSpec>;

#[derive(Debug)]
pub enum ArgumentLiteralValue {
    String(&'static str),
    Int(i64),
    Float(f64),
    Boolean(bool),
    EnumValue(&'static str),
}

impl From<i64> for ArgumentLiteralValue {
    fn from(value: i64) -> Self {
        return Self::Int(value);
    }
}

impl From<f64> for ArgumentLiteralValue {
    fn from(value: f64) -> Self {
        return Self::Float(value);
    }
}

impl From<bool> for ArgumentLiteralValue {
    fn from(value: bool) -> Self {
        return Self::Boolean(value);
    }
}

#[derive(Debug, derive_more::From)]
pub enum ArgumentValue {
    Ref(&'static str),
    Literal(ArgumentLiteralValue),
}

#[derive(Debug)]
pub struct FieldSelectionArgument {
    pub name: &'static str,
    pub value: ArgumentValue,
    pub r#type: FieldDefinition<InputFieldSpec>,
}

#[derive(Debug)]
pub struct ServerDirectiveInvocation {
    pub directive: &'static str,
    pub arguments: phf::OrderedMap<&'static str, FieldSelectionArgument>,
}

#[derive(Debug)]
pub struct ServerDirective {
    pub name: &'static str,
    pub arguments:
        phf::OrderedMap<&'static str, FieldDefinition<InputFieldSpec>>,
    pub locations: Vec<file::server::ast::DirectiveLocation>,
}

#[derive(Debug)]
pub struct InputType {
    pub name: &'static str,
    pub fields: phf::OrderedMap<&'static str, FieldDefinition<InputFieldSpec>>,
}
