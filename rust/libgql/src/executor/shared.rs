use crate::parsers::schema::client;

use super::ast::{GraphqlError, LiteralValue, NonNullableValue, Value};
use super::scalar::Scalar;

pub fn execute_typename_field<S: Scalar>(
    object_name: &str,
    field: &client::ast::TypenameField,
) -> Result<(String, Value<S>), Vec<GraphqlError>> {
    let field_name = field
        .alias
        .as_ref()
        .map(|v| v.as_str())
        .unwrap_or("__typename");
    Ok((
        field_name.to_string(),
        Value::NonNullable(NonNullableValue::Literal(LiteralValue::Scalar(
            S::from_str(&object_name.to_string()).map_err(|e| {
                vec![GraphqlError {
                    path: vec![field_name.to_string()],
                    message: e.into(),
                }]
            })?,
        ))),
    ))
}
