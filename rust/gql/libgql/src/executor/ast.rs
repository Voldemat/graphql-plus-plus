use super::scalar::Scalar;
use std::collections::HashMap;

pub trait TryGetStr {
    fn try_get_str(self: &Self) -> Option<&str>;
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

#[derive(Debug)]
pub enum LiteralValue<S: Scalar> {
    Object(Values<S>),
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

pub type Values<S> = HashMap<String, Value<S>>;

pub trait ToValue<S: Scalar> {
    fn to_value(self: &Self) -> Value<S>;
}

pub trait FromValue<S: Scalar> {
    fn from_value(value: &Value<S>) -> Self;
}
