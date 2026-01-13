use std::borrow::Cow;

use derive_more::with_trait::{Deref, Display, Error, From, TryInto};
use juniper::ScalarValue;

#[derive(
    Clone,
    Display,
    Debug,
    From,
    TryInto,
    PartialEq,
    juniper::ScalarValue,
    serde::Serialize,
)]
#[serde(untagged)]
pub enum MyScalarValue {
    #[from]
    Long(i64),
    #[from]
    #[value(to_float, to_int)]
    Int(i32),
    #[from]
    #[value(to_float)]
    Float(f64),
    #[from(&str, Cow<'_, str>, String)]
    #[value(as_str, to_string)]
    String(String),
    #[from]
    #[value(to_bool)]
    Boolean(bool),
}

impl<'de> serde::Deserialize<'de> for MyScalarValue {
    fn deserialize<D: serde::Deserializer<'de>>(
        de: D,
    ) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MyScalarValue;

            fn expecting(
                &self,
                f: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                f.write_str("a valid input value")
            }

            fn visit_bool<E: serde::de::Error>(
                self,
                b: bool,
            ) -> Result<Self::Value, E> {
                Ok(MyScalarValue::Boolean(b))
            }

            fn visit_i32<E: serde::de::Error>(
                self,
                n: i32,
            ) -> Result<Self::Value, E> {
                Ok(MyScalarValue::Int(n))
            }

            fn visit_i64<E: serde::de::Error>(
                self,
                n: i64,
            ) -> Result<Self::Value, E> {
                if n <= i64::from(i32::MAX) {
                    self.visit_i32(n.try_into().unwrap())
                } else {
                    Ok(MyScalarValue::Long(n))
                }
            }

            fn visit_u32<E: serde::de::Error>(
                self,
                n: u32,
            ) -> Result<Self::Value, E> {
                if n <= i32::MAX as u32 {
                    self.visit_i32(n.try_into().unwrap())
                } else {
                    self.visit_u64(n.into())
                }
            }

            fn visit_u64<E: serde::de::Error>(
                self,
                n: u64,
            ) -> Result<Self::Value, E> {
                if n <= i64::MAX as u64 {
                    self.visit_i64(n.try_into().unwrap())
                } else {
                    // Browser's `JSON.stringify()` serialize all numbers
                    // having no fractional part as integers (no decimal
                    // point), so we must parse large integers as floating
                    // point, otherwise we would error on transferring large
                    // floating point numbers.
                    Ok(MyScalarValue::Float(n as f64))
                }
            }

            fn visit_f64<E: serde::de::Error>(
                self,
                f: f64,
            ) -> Result<Self::Value, E> {
                Ok(MyScalarValue::Float(f))
            }

            fn visit_str<E: serde::de::Error>(
                self,
                s: &str,
            ) -> Result<Self::Value, E> {
                self.visit_string(s.into())
            }

            fn visit_string<E: serde::de::Error>(
                self,
                s: String,
            ) -> Result<Self::Value, E> {
                Ok(MyScalarValue::String(s))
            }
        }

        de.deserialize_any(Visitor)
    }
}

#[juniper::graphql_scalar]
#[graphql(scalar = MyScalarValue, with = int64_impl)]
type Int64 = i64;

mod int64_impl {
    use super::*;

    pub(super) fn to_output(value: &Int64) -> MyScalarValue {
        (*value).into()
    }

    pub(super) fn from_input(
        v: &juniper::Scalar<MyScalarValue>,
    ) -> Result<Int64, Box<str>> {
        let value = v.try_as_str().unwrap();
        println!("from_input: {}", value);
        Ok(value.parse::<i64>().unwrap())
    }

    pub(super) fn parse_token(
        value: juniper::ScalarToken<'_>,
    ) -> juniper::ParseScalarResult<MyScalarValue> {
        match value {
            juniper::ScalarToken::Int(a) => {
                juniper::ParseScalarResult::Ok(MyScalarValue::from(a))
            }
            _ => juniper::ParseScalarResult::Err(
                juniper::ParseError::UnexpectedToken(
                    "Only int is accepted".into(),
                ),
            ),
        }
    }
}
