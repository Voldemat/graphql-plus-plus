pub enum JSONScalar<'a> {
    String(&'a String),
    Bool(bool),
    Number(&'a serde_json::Number),
}

pub trait JSONSerializableScalar {
    fn to_json_value(self: &Self) -> Result<serde_json::Value, String>;
}

pub trait JSONParsableScalar: Sized {
    fn from_json_scalar(json_scalar: JSONScalar) -> Result<Self, String>;
}

pub trait InputScalar: crate::executor::Scalar + JSONParsableScalar {}
impl<T: crate::executor::Scalar + JSONParsableScalar> InputScalar for T {}

pub trait OutputScalar:
    crate::executor::Scalar + JSONSerializableScalar
{
}
impl<T: crate::executor::Scalar + JSONSerializableScalar> OutputScalar for T {}

pub fn parse_variable_from_json<S: InputScalar>(
    value: &serde_json::Value,
) -> Result<crate::executor::Value<S>, String> {
    match value {
        serde_json::Value::Null => Ok(crate::executor::Value::Null),
        serde_json::Value::String(s) => {
            Ok(crate::executor::Value::NonNullable(
                crate::executor::NonNullableValue::Literal(
                    crate::executor::LiteralValue::Scalar(S::from_json_scalar(
                        JSONScalar::String(s),
                    )?),
                ),
            ))
        }
        serde_json::Value::Bool(b) => Ok(crate::executor::Value::NonNullable(
            crate::executor::NonNullableValue::Literal(
                crate::executor::LiteralValue::Scalar(S::from_json_scalar(
                    JSONScalar::Bool(*b),
                )?),
            ),
        )),
        serde_json::Value::Number(n) => {
            Ok(crate::executor::Value::NonNullable(
                crate::executor::NonNullableValue::Literal(
                    crate::executor::LiteralValue::Scalar(S::from_json_scalar(
                        JSONScalar::Number(n),
                    )?),
                ),
            ))
        }
        serde_json::Value::Array(a) => Ok(crate::executor::Value::NonNullable(
            crate::executor::NonNullableValue::Array(
                a.iter()
                    .map(|element| parse_variable_from_json::<S>(element))
                    .collect::<Result<Vec<_>, String>>()?,
            ),
        )),
        serde_json::Value::Object(o) => {
            let mut variables = crate::executor::Values::<S>::new();
            for (key, value) in o {
                variables.insert(key.clone(), parse_variable_from_json(value)?);
            }
            Ok(crate::executor::Value::NonNullable(
                crate::executor::NonNullableValue::Literal(
                    crate::executor::LiteralValue::Object(
                        variables,
                    ),
                ),
            ))
        }
    }
}

pub fn parse_variables_from_json<S: InputScalar>(
    value: &serde_json::Value,
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
                variables.insert(key.clone(), parse_variable_from_json(value)?);
            }
            Ok(variables)
        }
    }
}

pub fn serialize_literal_value_to_json<S: OutputScalar>(
    value: &crate::executor::LiteralValue<S>,
) -> Result<serde_json::Value, String> {
    match value {
        crate::executor::LiteralValue::Scalar(scalar) => {
            S::to_json_value(scalar)
        }
        crate::executor::LiteralValue::Object(object_value) => {
            serialize_values_to_json(object_value)
        }
    }
}

pub fn serialize_array_value_to_json<
    S: crate::executor::Scalar + JSONSerializableScalar,
>(
    values: &[crate::executor::Value<S>],
) -> Result<serde_json::Value, String> {
    let mut array = Vec::<serde_json::Value>::new();
    for value in values {
        array.push(serialize_value_to_json(value)?);
    }
    return Ok(serde_json::Value::Array(array));
}

pub fn serialize_value_to_json<
    S: crate::executor::Scalar + JSONSerializableScalar,
>(
    value: &crate::executor::Value<S>,
) -> Result<serde_json::Value, String> {
    match value {
        crate::executor::Value::Null => Ok(serde_json::Value::Null),
        crate::executor::Value::NonNullable(non_nullable) => match non_nullable
        {
            crate::executor::NonNullableValue::Literal(literal) => {
                serialize_literal_value_to_json(literal)
            }
            crate::executor::NonNullableValue::Array(array) => {
                serialize_array_value_to_json(array)
            }
        },
    }
}

pub fn serialize_values_to_json<
    S: crate::executor::Scalar + JSONSerializableScalar,
>(
    values: &crate::executor::Values<S>,
) -> Result<serde_json::Value, String> {
    let mut map = serde_json::Map::new();
    for (key, value) in values {
        map.insert(key.clone(), serialize_value_to_json(value)?);
    }
    return Ok(serde_json::Value::Object(map));
}
