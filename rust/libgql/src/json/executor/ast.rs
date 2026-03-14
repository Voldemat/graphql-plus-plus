pub enum JSONScalar<'a> {
    String(&'a String),
    Bool(bool),
    Number(&'a serde_json::Number),
}

pub fn parse_variable_from_json<S: crate::executor::Scalar>(
    value: &serde_json::Value,
    scalar_parser: &impl Fn(JSONScalar) -> Result<S, String>,
) -> Result<crate::executor::Value<S>, String> {
    match value {
        serde_json::Value::Null => Ok(crate::executor::Value::Null),
        serde_json::Value::String(s) => {
            Ok(crate::executor::Value::NonNullable(
                crate::executor::NonNullableValue::Literal(
                    crate::executor::LiteralValue::Scalar(scalar_parser(
                        JSONScalar::String(s),
                    )?),
                ),
            ))
        }
        serde_json::Value::Bool(b) => Ok(crate::executor::Value::NonNullable(
            crate::executor::NonNullableValue::Literal(
                crate::executor::LiteralValue::Scalar(scalar_parser(
                    JSONScalar::Bool(*b),
                )?),
            ),
        )),
        serde_json::Value::Number(n) => {
            Ok(crate::executor::Value::NonNullable(
                crate::executor::NonNullableValue::Literal(
                    crate::executor::LiteralValue::Scalar(scalar_parser(
                        JSONScalar::Number(n),
                    )?),
                ),
            ))
        }
        serde_json::Value::Array(a) => Ok(crate::executor::Value::NonNullable(
            crate::executor::NonNullableValue::Array(
                a.iter()
                    .map(|element| {
                        parse_variable_from_json::<S>(element, scalar_parser)
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
        )),
        serde_json::Value::Object(o) => {
            let mut variables = crate::executor::Values::<S>::new();
            for (key, value) in o {
                variables.insert(
                    key.clone(),
                    parse_variable_from_json(value, scalar_parser)?,
                );
            }
            Ok(crate::executor::Value::NonNullable(
                crate::executor::NonNullableValue::Literal(
                    crate::executor::LiteralValue::Object(
                        "".to_string(),
                        variables,
                    ),
                ),
            ))
        }
    }
}

pub fn parse_variables_from_json<S: crate::executor::Scalar>(
    value: &serde_json::Value,
    scalar_parser: &impl Fn(JSONScalar) -> Result<S, String>,
) -> Result<crate::executor::Values<S>, String> {
    match value {
        serde_json::Value::Null => Ok(crate::executor::Values::new()),
        serde_json::Value::String(_) => {
            Err("Variables must be json object, received a string".into())
        }
        serde_json::Value::Number(_) => {
            Err("Variables must be json object, received a number".into())
        }
        serde_json::Value::Bool(_) => {
            Err("Variables must be json object, received a bool".into())
        }
        serde_json::Value::Array(_) => {
            Err("Variables must be json object, received an array".into())
        }
        serde_json::Value::Object(o) => {
            let mut variables = crate::executor::Values::<S>::new();
            for (key, value) in o {
                variables.insert(
                    key.clone(),
                    parse_variable_from_json(value, scalar_parser)?,
                );
            }
            Ok(variables)
        }
    }
}

pub fn serialize_literal_value_to_json<S: crate::executor::Scalar>(
    value: &crate::executor::LiteralValue<S>,
    scalar_serializer: &impl Fn(&S) -> Result<serde_json::Value, String>,
) -> Result<serde_json::Value, String> {
    match value {
        crate::executor::LiteralValue::Scalar(scalar) => {
            scalar_serializer(scalar)
        }
        crate::executor::LiteralValue::Object(_, object_value) => {
            serialize_values_to_json(object_value, scalar_serializer)
        }
    }
}

pub fn serialize_array_value_to_json<S: crate::executor::Scalar>(
    values: &[crate::executor::Value<S>],
    scalar_serializer: &impl Fn(&S) -> Result<serde_json::Value, String>,
) -> Result<serde_json::Value, String> {
    let mut array = Vec::<serde_json::Value>::new();
    for value in values {
        array.push(serialize_value_to_json(value, scalar_serializer)?);
    }
    return Ok(serde_json::Value::Array(array));
}

pub fn serialize_value_to_json<S: crate::executor::Scalar>(
    value: &crate::executor::Value<S>,
    scalar_serializer: &impl Fn(&S) -> Result<serde_json::Value, String>,
) -> Result<serde_json::Value, String> {
    match value {
        crate::executor::Value::Null => Ok(serde_json::Value::Null),
        crate::executor::Value::NonNullable(non_nullable) => match non_nullable
        {
            crate::executor::NonNullableValue::Literal(literal) => {
                serialize_literal_value_to_json(literal, scalar_serializer)
            }
            crate::executor::NonNullableValue::Array(array) => {
                serialize_array_value_to_json(array, scalar_serializer)
            }
        },
    }
}

pub fn serialize_values_to_json<S: crate::executor::Scalar>(
    values: &crate::executor::Values<S>,
    scalar_serializer: &impl Fn(&S) -> Result<serde_json::Value, String>,
) -> Result<serde_json::Value, String> {
    let mut map = serde_json::Map::new();
    for (key, value) in values {
        map.insert(
            key.clone(),
            serialize_value_to_json(value, scalar_serializer)?,
        );
    }
    return Ok(serde_json::Value::Object(map));
}
