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
    Object(String, Values<S>),
    Scalar(S),
}

pub type Values<S> = HashMap<String, Value<S>>;
pub type ResolverRoot<S> = Values<S>;
