use super::scalar::Scalar;
use std::collections::{HashMap, HashSet};

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
    Object(String, Values<S>),
    Scalar(S),
}

impl<S: Scalar> From<(String, Values<S>)> for LiteralValue<S> {
    fn from((a, b): (String, Values<S>)) -> Self {
        Self::Object(a, b)
    }
}
pub type Values<S> = HashMap<String, Value<S>>;

pub enum NonNullableResolverIntrospectionValue<'a, S> {
    Literal(String, &'a ResolverRoot<S>),
    Array(Vec<&'a ResolverRoot<S>>),
}

pub type ResolverIntrospectionValue<'a, S> =
    Option<NonNullableResolverIntrospectionValue<'a, S>>;

pub trait ResolverValue<S: Scalar> {
    fn create_introspection_value<'a>(
        self: &'a Self,
    ) -> ResolverIntrospectionValue<'a, S>;

    fn to_value(
        self: &Self,
        callable_fields: Vec<(String, Value<S>)>,
    ) -> Result<Value<S>, String>;

    fn get_existing_fields(self: &Self) -> HashSet<String>;
}

impl<S: Scalar> ResolverValue<S> for &() {
    fn create_introspection_value<'a>(
        self: &'a Self,
    ) -> ResolverIntrospectionValue<'a, S> {
        panic!("Unexpected create_introspection_value on root value");
    }

    fn to_value(
        self: &Self,
        _: Vec<(String, Value<S>)>,
    ) -> Result<Value<S>, String> {
        Ok(Value::Null)
    }

    fn get_existing_fields(self: &Self) -> HashSet<String> {
        HashSet::new()
    }
}

impl<S: Scalar, T: ResolverValue<S>> ResolverValue<S> for Option<T> {
    fn to_value(
        self: &Self,
        callable_fields: Vec<(String, Value<S>)>,
    ) -> Result<Value<S>, String> {
        match self {
            None => Ok(Value::Null),
            Some(v) => ResolverValue::<S>::to_value(v, callable_fields),
        }
    }

    fn create_introspection_value<'a>(
        self: &'a Self,
    ) -> ResolverIntrospectionValue<'a, S> {
        match self {
            None => None,
            Some(v) => ResolverValue::<S>::create_introspection_value(v),
        }
    }

    fn get_existing_fields(self: &Self) -> HashSet<String> {
        match self {
            None => HashSet::new(),
            Some(v) => ResolverValue::<S>::get_existing_fields(v),
        }
    }
}

impl<S: Scalar, T: ResolverValue<S> + 'static> ResolverValue<S> for Vec<T> {
    fn to_value(
        self: &Self,
        _: Vec<(String, Value<S>)>,
    ) -> Result<Value<S>, String> {
        self.iter()
            .map(|element| ResolverValue::<S>::to_value(element, Vec::new()))
            .collect::<Result<Vec<_>, String>>()
            .map(|v| Value::NonNullable(NonNullableValue::Array(v)))
    }

    fn create_introspection_value<'a>(
        self: &'a Self,
    ) -> ResolverIntrospectionValue<'a, S> {
        Some(NonNullableResolverIntrospectionValue::Array(
            self.iter()
                .map(|element: &'a T| {
                    element as &'a dyn ResolverValueSuperTrait<S>
                })
                .collect(),
        ))
    }

    fn get_existing_fields(self: &Self) -> HashSet<String> {
        panic!("Unexpected get_existing_fields on Vec")
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
