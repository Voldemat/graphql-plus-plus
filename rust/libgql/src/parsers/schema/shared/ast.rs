use crate::parsers::file;

pub trait AsStr<'s>:
    Ord
    + std::hash::Hash
    + std::borrow::Borrow<str>
    + Clone
    + Send
    + Sync
    + std::fmt::Debug
{
    fn to_str(self: &Self) -> &str;
    fn from_str(s: &'s str) -> Self;
}

impl<'s> AsStr<'s> for &'s str {
    fn to_str(self: &Self) -> &str {
        *self
    }

    fn from_str(s: &'s str) -> Self {
        s
    }
}

impl<'s> AsStr<'s> for String {
    fn to_str(self: &Self) -> &str {
        self.as_str()
    }

    fn from_str(s: &'s str) -> Self {
        s.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum InputTypeSpec<S = String> {
    InputType(S),
    Scalar(S),
    Enum(S),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum ArrayLiteral {
    Int(Vec<i64>),
    Float(Vec<f64>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}

#[derive(Debug, Clone)]
pub struct LiteralFieldSpec<T, S = String> {
    pub r#type: T,
    pub default_value: Option<Option<Literal>>,
    pub directive_invocations:
        indexmap::IndexMap<S, ServerDirectiveInvocation<S>>,
}

#[derive(Debug, Clone)]
pub struct ArrayFieldSpec<T, S = String> {
    pub r#type: Box<NonCallableFieldSpec<T, S>>,
    pub nullable: bool,
    pub default_value: Option<Option<ArrayLiteral>>,
    pub directive_invocations: Vec<ServerDirectiveInvocation<S>>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum NonCallableFieldSpec<T, S = String> {
    Literal(LiteralFieldSpec<T, S>),
    Array(ArrayFieldSpec<T, S>),
}

impl<T, S> NonCallableFieldSpec<T, S> {
    pub fn has_default_value(self: &Self) -> bool {
        match self {
            Self::Literal(literal) => {
                literal.default_value.is_some()
                    && literal.default_value.as_ref().unwrap().is_some()
            }
            Self::Array(array) => {
                array.default_value.is_some()
                    && array.default_value.as_ref().unwrap().is_some()
            }
        }
    }

    pub fn get_type_spec(self: &Self) -> &T {
        match self {
            Self::Literal(literal) => &literal.r#type,
            Self::Array(array) => &array.r#type.get_type_spec(),
        }
    }
}

pub type InputFieldSpec<S = String> = NonCallableFieldSpec<InputTypeSpec<S>, S>;

#[derive(Debug, Clone)]
pub struct FieldDefinition<T, S = String> {
    pub name: S,
    pub spec: T,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct InputType<S = String> {
    pub name: S,
    pub fields: indexmap::IndexMap<S, FieldDefinition<InputFieldSpec<S>, S>>,
}

#[derive(Debug, Clone)]
pub enum ArgumentLiteralValue<S = String> {
    String(S),
    Int(i64),
    Float(f64),
    Boolean(bool),
    EnumValue(S),
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

#[derive(Debug, Clone, derive_more::From)]
pub enum ArgumentValue<S = String> {
    Ref(S),
    Literal(ArgumentLiteralValue<S>),
}

#[derive(Debug, Clone)]
pub struct FieldSelectionArgument<S = String> {
    pub name: S,
    pub value: ArgumentValue<S>,
    pub r#type: FieldDefinition<InputFieldSpec<S>, S>,
}

#[derive(Debug, Clone)]
pub struct ServerDirective<S = String> {
    pub name: S,
    pub arguments: indexmap::IndexMap<S, FieldDefinition<InputFieldSpec<S>, S>>,
    pub locations: Vec<file::server::ast::DirectiveLocation>,
}

#[derive(Debug, Clone)]
pub struct ServerDirectiveInvocation<S = String> {
    pub directive: S,
    pub arguments: indexmap::IndexMap<S, FieldSelectionArgument<S>>,
}
