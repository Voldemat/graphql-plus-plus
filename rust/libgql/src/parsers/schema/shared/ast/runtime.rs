use crate::parsers::file;

use super::AsStr;

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

impl super::traits::Enum for Enum {
    fn get_name(self: &Self) -> &str {
        &self.name
    }

    fn has_value(self: &Self, value: &str) -> bool {
        self.values.iter().any(|v| v == value)
    }
}

#[derive(Debug, Clone)]
pub enum InputTypeSpec<S = String> {
    InputType(S),
    Scalar(S),
    Enum(S),
}

impl<S: std::fmt::Display> std::fmt::Display for InputTypeSpec<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputType(s) => s.fmt(f),
            Self::Scalar(s) => s.fmt(f),
            Self::Enum(s) => s.fmt(f),
        }
    }
}

impl<'s, S: AsStr<'s>> super::traits::InputTypeSpec for InputTypeSpec<S> {
    fn get_ref(self: &Self) -> super::traits::InputTypeSpecRef<'_> {
        match self {
            Self::InputType(s) => {
                super::traits::InputTypeSpecRef::InputType(s.to_str())
            }
            Self::Scalar(s) => {
                super::traits::InputTypeSpecRef::Scalar(s.to_str())
            }
            Self::Enum(s) => super::traits::InputTypeSpecRef::Enum(s.to_str()),
        }
    }
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
pub enum Literal<S> {
    Null,
    Int(i64),
    Float(f64),
    String(S),
    Boolean(bool),
    EnumValue(S),
}

impl<S: std::fmt::Display> std::fmt::Display for Literal<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::String(s) => s.fmt(f),
            Self::Int(i) => f.write_fmt(format_args!("{}", i)),
            Self::Float(v) => f.write_fmt(format_args!("{}", v)),
            Self::Boolean(b) => f.write_fmt(format_args!("{}", b)),
            Self::EnumValue(s) => f.write_fmt(format_args!("{}", s)),
        }
    }
}

impl<'s1, S: AsStr<'s1>> Literal<S> {
    pub fn clone_with_string_type<'s2, NS: AsStr<'s2>>(
        self: &'s1 Self,
    ) -> Literal<NS>
    where
        's1: 's2,
    {
        match self {
            Self::Null => Literal::Null,
            Self::String(s) => Literal::String(NS::from_str(s.to_str())),
            Self::Int(i) => Literal::Int(*i),
            Self::Float(f) => Literal::Float(*f),
            Self::Boolean(b) => Literal::Boolean(*b),
            Self::EnumValue(s) => Literal::EnumValue(NS::from_str(s.to_str())),
        }
    }
}

impl<S> From<i64> for Literal<S> {
    fn from(value: i64) -> Self {
        return Self::Int(value);
    }
}

impl<S> From<f64> for Literal<S> {
    fn from(value: f64) -> Self {
        return Self::Float(value);
    }
}

impl<S> From<bool> for Literal<S> {
    fn from(value: bool) -> Self {
        return Self::Boolean(value);
    }
}

impl<'s, S: AsStr<'s>> super::traits::Literal for Literal<S> {
    fn get_ref(self: &Self) -> super::traits::LiteralRef<'_> {
        match self {
            Self::Null => super::traits::LiteralRef::Null,
            Self::Int(i) => super::traits::LiteralRef::Int(i),
            Self::Float(f) => super::traits::LiteralRef::Float(f),
            Self::Boolean(b) => super::traits::LiteralRef::Boolean(b),
            Self::String(s) => super::traits::LiteralRef::String(s.to_str()),
            Self::EnumValue(s) => {
                super::traits::LiteralRef::EnumValue(s.to_str())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArrayLiteral {
    Int(Vec<i64>),
    Float(Vec<f64>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}

impl super::traits::ArrayLiteral for ArrayLiteral {
    fn get_ref(
        self: &Self,
    ) -> super::traits::ArrayLiteralRef<'_, impl AsStr<'_>> {
        match self {
            Self::Int(i) => super::traits::ArrayLiteralRef::Int(&i),
            Self::Float(f) => super::traits::ArrayLiteralRef::Float(&f),
            Self::Boolean(b) => super::traits::ArrayLiteralRef::Boolean(&b),
            Self::String(s) => {
                super::traits::ArrayLiteralRef::String(&s.as_ref())
            }
        }
    }
}

fn format_slice<T: std::fmt::Display>(
    v: &[T],
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    f.write_str("[")?;
    for (index, value) in v.iter().enumerate() {
        f.write_fmt(format_args!("{}", value))?;
        if index != v.len() - 1 {
            f.write_str(", ")?;
        }
    }
    f.write_str("]")
}

impl std::fmt::Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => format_slice(v, f),
            Self::Float(v) => format_slice(v, f),
            Self::Boolean(v) => format_slice(v, f),
            Self::String(v) => format_slice(v, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralFieldSpec<T, S = String> {
    pub r#type: T,
    pub default_value: Option<Literal<S>>,
    pub directive_invocations:
        indexmap::IndexMap<S, ServerDirectiveInvocation<S>>,
}

impl<T: std::fmt::Display, S: std::fmt::Display> std::fmt::Display
    for LiteralFieldSpec<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.r#type))?;
        if let Some(default_value) = &self.default_value {
            f.write_fmt(format_args!(" = {}", default_value))?;
        }
        for invocation in self.directive_invocations.values() {
            f.write_fmt(format_args!(" {}", invocation))?;
        }
        Ok(())
    }
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
            default_value: self
                .default_value
                .as_ref()
                .map(|v| v.clone_with_string_type()),
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
    pub default_value: Option<ArrayLiteral>,
    pub directive_invocations: Vec<ServerDirectiveInvocation<S>>,
}

impl<T: std::fmt::Display, S: std::fmt::Display> std::fmt::Display
    for ArrayFieldSpec<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            self.r#type,
            if self.nullable { "" } else { "!" }
        ))?;
        if let Some(default_value) = &self.default_value {
            f.write_fmt(format_args!(" = {}", default_value))?;
        }
        for invocation in &self.directive_invocations {
            f.write_fmt(format_args!(" {}", invocation))?;
        }
        Ok(())
    }
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

impl<T: std::fmt::Display, S: std::fmt::Display> std::fmt::Display
    for NonCallableFieldSpec<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => literal.fmt(f),
            Self::Array(array) => array.fmt(f),
        }
    }
}

impl<'s1, T, S: AsStr<'s1>> NonCallableFieldSpec<T, S> {
    pub fn has_default_value(self: &Self) -> bool {
        match self {
            Self::Literal(literal) => literal.default_value.is_some(),
            Self::Array(array) => array.default_value.is_some(),
        }
    }

    pub fn get_type_spec(self: &Self) -> &T {
        match self {
            Self::Literal(literal) => &literal.r#type,
            Self::Array(array) => &array.r#type.get_type_spec(),
        }
    }

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

pub type InputFieldSpec<S = String> = NonCallableFieldSpec<InputTypeSpec<S>, S>;

#[derive(Debug, Clone)]
pub struct FieldDefinition<T, S = String> {
    pub name: S,
    pub spec: T,
    pub nullable: bool,
}

impl<T: std::fmt::Display, S: std::fmt::Display> std::fmt::Display
    for FieldDefinition<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}{}",
            self.name,
            self.spec,
            if self.nullable { "" } else { "!" }
        ))
    }
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

#[derive(Debug, Clone, derive_more::From)]
pub enum ArgumentValue<S = String> {
    Ref(S),
    Literal(Literal<S>),
}

impl<S: std::fmt::Display> std::fmt::Display for ArgumentValue<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ref(v) => v.fmt(f),
            Self::Literal(v) => v.fmt(f),
        }
    }
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

impl<S: std::fmt::Display> std::fmt::Display for FieldSelectionArgument<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)?;
        f.write_str(": ")?;
        self.value.fmt(f)
    }
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

impl<S: std::fmt::Display> std::fmt::Display for ServerDirectiveInvocation<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("@{}", self.directive))?;
        if self.arguments.len() == 0 {
            return Ok(());
        }
        f.write_str("(")?;
        for (index, argument) in self.arguments.values().enumerate() {
            argument.fmt(f)?;
            if index != self.arguments.len() - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
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
