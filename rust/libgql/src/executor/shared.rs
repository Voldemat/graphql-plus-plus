use crate::parsers::schema::client;

use super::ast::{LiteralValue, NonNullableValue, Value};
use super::scalar::Scalar;

pub fn execute_typename_field<S: Scalar>(
    object_name: &str,
    field: &client::ast::TypenameField,
) -> Result<(String, Value<S>), String> {
    Ok((
        field
            .alias
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or("__typename")
            .into(),
        Value::NonNullable(NonNullableValue::Literal(LiteralValue::Scalar(
            S::from_str(&object_name.to_string())?,
        ))),
    ))
}
