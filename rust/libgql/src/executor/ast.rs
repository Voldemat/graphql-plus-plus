use super::scalar::Scalar;
use std::collections::HashMap;

pub trait TryGetStr {
    fn try_get_str(self: &Self) -> Option<&str>;
}

pub trait TryGetScalar<S: Scalar> {
    fn try_get_scalar(self: &Self) -> Option<&S>;
}

#[derive(Debug)]
pub enum Value<S: Scalar> {
    Null,
    NonNullable(NonNullableValue<S>),
}

impl<S: Scalar> TryGetStr for Value<S> {
    fn try_get_str(self: &Self) -> Option<&str> {
        match self {
            Self::NonNullable(n) => n.try_get_str(),
            _ => None,
        }
    }
}

impl<S: Scalar> TryGetScalar<S> for Value<S> {
    fn try_get_scalar(self: &Self) -> Option<&S> {
        match self {
            Self::Null => None,
            Self::NonNullable(non_nullable) => non_nullable.try_get_scalar(),
        }
    }
}

impl<S: Scalar> Value<S> {
    pub fn to_non_nullable_option(self: &Self) -> Option<&NonNullableValue<S>> {
        match self {
            Self::Null => None,
            Self::NonNullable(n) => Some(n),
        }
    }
}

pub fn extract_array<S: Scalar, E, F: Fn(&Value<S>) -> Result<E, String>>(
    value: &NonNullableValue<S>,
    element_validate_func: F,
) -> Result<Vec<E>, String> {
    value.get_array()
        .ok_or("Unexpected literal value for array".to_string())
        .map(|array_values| {
            array_values
                .iter()
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

impl<S: Scalar> TryGetStr for NonNullableValue<S> {
    fn try_get_str(self: &Self) -> Option<&str> {
        match self {
            NonNullableValue::Literal(l) => l.try_get_str(),
            _ => None,
        }
    }
}

impl<S: Scalar> TryGetScalar<S> for NonNullableValue<S> {
    fn try_get_scalar(self: &Self) -> Option<&S> {
        match self {
            Self::Array(_) => None,
            Self::Literal(literal) => literal.try_get_scalar(),
        }
    }
}

impl<S: Scalar> NonNullableValue<S> {
    pub fn get_literal(self: &Self) -> Option<&LiteralValue<S>> {
        match self {
            Self::Literal(literal) => Some(literal),
            Self::Array(_) => None,
        }
    }

    pub fn get_array(self: &Self) -> Option<&Vec<Value<S>>> {
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

impl<S: Scalar> TryGetStr for LiteralValue<S> {
    fn try_get_str(self: &Self) -> Option<&str> {
        match self {
            LiteralValue::Scalar(s) => s.get_str(),
            _ => None,
        }
    }
}

impl<S: Scalar> TryGetScalar<S> for LiteralValue<S> {
    fn try_get_scalar(self: &Self) -> Option<&S> {
        match self {
            Self::Object(_, _) => None,
            Self::Scalar(scalar) => Some(scalar),
        }
    }
}

pub type Values<S> = HashMap<String, Value<S>>;
pub type ResolverRoot<S> = Values<S>;
