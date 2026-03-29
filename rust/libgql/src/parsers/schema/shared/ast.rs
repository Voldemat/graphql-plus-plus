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

impl<'s1, S: AsStr<'s1>> InputTypeSpec<S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>>(
        self: &'s1 Self,
    ) -> InputTypeSpec<NS>
    where
        's1: 's2,
    {
        match self {
            Self::InputType(s) => {
                InputTypeSpec::InputType(NS::from_str(s.to_str()))
            }
            Self::Scalar(s) => InputTypeSpec::Scalar(NS::from_str(s.to_str())),
            Self::Enum(s) => InputTypeSpec::Enum(NS::from_str(s.to_str())),
        }
    }
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

impl<'s1, T, S: AsStr<'s1>> LiteralFieldSpec<T, S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>, T2>(
        self: &'s1 Self,
        clone_t: impl Fn(&'s1 T) -> T2,
    ) -> LiteralFieldSpec<T2, NS>
    where
        's1: 's2,
    {
        LiteralFieldSpec {
            r#type: clone_t(&self.r#type),
            default_value: self.default_value.clone(),
            directive_invocations: self
                .directive_invocations
                .iter()
                .map(|(key, invocation)| {
                    (
                        NS::from_str(key.to_str()),
                        invocation.clone_with_string_type::<NS>(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayFieldSpec<T, S = String> {
    pub r#type: Box<NonCallableFieldSpec<T, S>>,
    pub nullable: bool,
    pub default_value: Option<Option<ArrayLiteral>>,
    pub directive_invocations: Vec<ServerDirectiveInvocation<S>>,
}

impl<'s1, T, S: AsStr<'s1>> ArrayFieldSpec<T, S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>, T2>(
        self: &'s1 Self,
        clone_t: impl Fn(&'s1 T) -> T2,
    ) -> ArrayFieldSpec<T2, NS>
    where
        's1: 's2,
    {
        ArrayFieldSpec {
            r#type: Box::new(self.r#type.clone_with_string_type(clone_t)),
            nullable: self.nullable,
            default_value: self.default_value.clone(),
            directive_invocations: self
                .directive_invocations
                .iter()
                .map(ServerDirectiveInvocation::clone_with_string_type::<NS>)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum NonCallableFieldSpec<T, S = String> {
    Literal(LiteralFieldSpec<T, S>),
    Array(ArrayFieldSpec<T, S>),
}

impl<'s1, T, S: AsStr<'s1>> NonCallableFieldSpec<T, S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>, T2>(
        self: &'s1 Self,
        clone_t: impl Fn(&'s1 T) -> T2,
    ) -> NonCallableFieldSpec<T2, NS>
    where
        's1: 's2,
    {
        match self {
            Self::Literal(l) => {
                NonCallableFieldSpec::Literal(l.clone_with_string_type(clone_t))
            }
            Self::Array(a) => {
                NonCallableFieldSpec::Array(a.clone_with_string_type(clone_t))
            }
        }
    }
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

impl<'s1, T, S: AsStr<'s1>> FieldDefinition<T, S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>, T2>(
        self: &'s1 Self,
        clone_spec: impl Fn(&'s1 T) -> T2,
    ) -> FieldDefinition<T2, NS>
    where
        's1: 's2,
    {
        FieldDefinition {
            name: NS::from_str(self.name.to_str()),
            spec: clone_spec(&self.spec),
            nullable: self.nullable,
        }
    }
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

impl<'s1, S: AsStr<'s1>> ArgumentLiteralValue<S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>>(
        self: &'s1 Self,
    ) -> ArgumentLiteralValue<NS>
    where
        's1: 's2,
    {
        match self {
            Self::String(s) => {
                ArgumentLiteralValue::String(NS::from_str(s.to_str()))
            }
            Self::Int(i) => ArgumentLiteralValue::Int(*i),
            Self::Float(f) => ArgumentLiteralValue::Float(*f),
            Self::Boolean(b) => ArgumentLiteralValue::Boolean(*b),
            Self::EnumValue(s) => {
                ArgumentLiteralValue::EnumValue(NS::from_str(s.to_str()))
            }
        }
    }
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

impl<'s1, S: AsStr<'s1>> ArgumentValue<S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>>(
        self: &'s1 Self,
    ) -> ArgumentValue<NS>
    where
        's1: 's2,
    {
        match self {
            Self::Ref(r) => ArgumentValue::Ref(NS::from_str(r.to_str())),
            Self::Literal(value) => {
                ArgumentValue::Literal(value.clone_with_string_type())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSelectionArgument<S = String> {
    pub name: S,
    pub value: ArgumentValue<S>,
    pub r#type: FieldDefinition<InputFieldSpec<S>, S>,
}

impl<'s1, S: AsStr<'s1>> FieldSelectionArgument<S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>>(
        self: &'s1 Self,
    ) -> FieldSelectionArgument<NS>
    where
        's1: 's2,
    {
        FieldSelectionArgument {
            name: NS::from_str(self.name.to_str()),
            value: self.value.clone_with_string_type::<NS>(),
            r#type: self.r#type.clone_with_string_type(|s| {
                InputFieldSpec::clone_with_string_type(
                    s,
                    InputTypeSpec::clone_with_string_type::<NS>,
                )
            }),
        }
    }
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

impl<'s1, S: AsStr<'s1>> ServerDirectiveInvocation<S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>>(
        self: &'s1 Self,
    ) -> ServerDirectiveInvocation<NS>
    where
        's1: 's2,
    {
        ServerDirectiveInvocation {
            directive: NS::from_str(self.directive.to_str()),
            arguments: self
                .arguments
                .iter()
                .map(|(key, argument)| {
                    (
                        NS::from_str(key.to_str()),
                        argument.clone_with_string_type::<NS>(),
                    )
                })
                .collect(),
        }
    }
}
