use super::scalar::Scalar;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<S: Scalar> {
    Null,
    NonNullable(NonNullableValue<S>),
}

impl<S: Scalar> Value<S> {
    pub fn to_non_nullable_option(self: Self) -> Option<NonNullableValue<S>> {
        match self {
            Self::Null => None,
            Self::NonNullable(n) => Some(n),
        }
    }
}

pub fn extract_array<S: Scalar, E, F: Fn(Value<S>) -> Result<E, String>>(
    value: NonNullableValue<S>,
    element_validate_func: F,
) -> Result<Vec<E>, String> {
    value
        .get_array()
        .ok_or("Unexpected literal value for array".to_string())
        .map(|array_values| {
            array_values
                .into_iter()
                .map(element_validate_func)
                .collect::<Result<Vec<_>, String>>()
        })
        .flatten()
}

#[derive(Debug)]
pub enum NonNullableValue<S: Scalar> {
    Array(Vec<Value<S>>),
    Literal(LiteralValue<S>),
}

impl<S: Scalar> NonNullableValue<S> {
    pub fn get_literal(self: Self) -> Option<LiteralValue<S>> {
        match self {
            Self::Literal(literal) => Some(literal),
            Self::Array(_) => None,
        }
    }

    pub fn get_array(self: Self) -> Option<Vec<Value<S>>> {
        match self {
            Self::Literal(_) => None,
            Self::Array(array) => Some(array),
        }
    }
}

#[derive(Debug)]
pub enum LiteralValue<S: Scalar> {
    Object(Values<S>),
    Scalar(S),
}

pub type Values<S> = HashMap<String, Value<S>>;

pub enum NonNullableResolverIntrospectionValue<'a, S: Scalar> {
    Scalar(S),
    Object(
        &'a ResolverRoot<S>,
        &'a str,
        HashMap<&'a str, &'a ResolverRoot<S>>,
    ),
    Array(Vec<ResolverIntrospectionValue<'a, S>>),
}

pub type ResolverIntrospectionValue<'a, S> =
    Option<NonNullableResolverIntrospectionValue<'a, S>>;

pub trait ResolverValue<S: Scalar> {
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<ResolverIntrospectionValue<'a, S>, String>;
}

impl<S: Scalar> ResolverValue<S> for &() {
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<ResolverIntrospectionValue<'a, S>, String> {
        Ok(None)
    }
}

impl<S: Scalar, T: ResolverValue<S>> ResolverValue<S> for Option<T> {
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<ResolverIntrospectionValue<'a, S>, String> {
        match self {
            None => Ok(None),
            Some(v) => ResolverValue::<S>::to_value(v),
        }
    }
}

impl<S: Scalar, T: ResolverValue<S> + 'static> ResolverValue<S> for Vec<T> {
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<ResolverIntrospectionValue<'a, S>, String> {
        self.iter()
            .map(|element| ResolverValue::<S>::to_value(element))
            .collect::<Result<Vec<_>, String>>()
            .map(|array| {
                Some(NonNullableResolverIntrospectionValue::Array(array))
            })
    }
}

pub trait ResolverValueSuperTrait<S: Scalar>:
    std::any::Any + ResolverValue<S>
{
}
impl<S: Scalar, T: std::any::Any + ResolverValue<S>> ResolverValueSuperTrait<S>
    for T
{
}

pub type ResolverRoot<S> = dyn ResolverValueSuperTrait<S>;

pub type ResolverFuture<'a, S> = std::pin::Pin<
    Box<dyn Future<Output = Result<Box<ResolverRoot<S>>, String>> + 'a>,
>;
