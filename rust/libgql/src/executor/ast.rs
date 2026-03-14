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

pub trait ToValue<S: Scalar> {
    fn to_value(self: &Self) -> Value<S>;
}

pub trait FromValue<S: Scalar> {
    fn from_value(value: &Value<S>) -> Self;
}
