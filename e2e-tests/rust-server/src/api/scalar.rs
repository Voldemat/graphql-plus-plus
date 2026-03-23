#[derive(Debug)]
pub enum Scalar {
    String(String),
    Int(i32),
    Float(f32),
    Boolean(bool),
}

impl libgql::executor::Scalar for Scalar {
    fn try_to_string(self: Self) -> Result<String, String> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err("Invalid scalar type for string".to_string()),
        }
    }

    fn from_str(str: &str) -> Result<Self, String> {
        Ok(Self::String(str.to_string()))
    }

    fn from_literal(
        literal: &libgql::parsers::schema::shared::ast::Literal,
    ) -> Result<Scalar, String> {
        match literal {
            libgql::parsers::schema::shared::ast::Literal::Int(i) => Ok(Scalar::Int(
                TryInto::<i32>::try_into(*i).map_err(|e| e.to_string())?,
            )),
            libgql::parsers::schema::shared::ast::Literal::Float(f) => Ok(Scalar::Float(*f as f32)),
            libgql::parsers::schema::shared::ast::Literal::String(s) => {
                Ok(Scalar::String(s.to_string()))
            }
            libgql::parsers::schema::shared::ast::Literal::Boolean(b) => Ok(Scalar::Boolean(*b)),
        }
    }
}

impl libgql::json::executor::ast::JSONParsableScalar for Scalar {
    fn from_json_scalar<'a>(
        json_scalar: libgql::json::executor::ast::JSONScalar<'a>,
    ) -> Result<Scalar, String> {
        match json_scalar {
            libgql::json::executor::ast::JSONScalar::Bool(b) => Ok(Scalar::Boolean(b)),
            libgql::json::executor::ast::JSONScalar::String(s) => Ok(Scalar::String(s.to_string())),
            libgql::json::executor::ast::JSONScalar::Number(n) => {
                if let Some(u64_n) = n.as_u64() {
                    Ok(Scalar::Int(
                        TryInto::<i32>::try_into(u64_n).map_err(|e| e.to_string())?,
                    ))
                } else if let Some(i64_n) = n.as_i64() {
                    Ok(Scalar::Int(
                        TryInto::<i32>::try_into(i64_n).map_err(|e| e.to_string())?,
                    ))
                } else if let Some(f64_n) = n.as_f64() {
                    Ok(Scalar::Float(f64_n as f32))
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl libgql::json::executor::ast::JSONSerializableScalar for Scalar {
    fn to_json_value(self: &Self) -> Result<serde_json_path_to_error::Value, String> {
        match self {
            Self::Int(i) => Ok(serde_json_path_to_error::Value::Number(
                serde_json_path_to_error::Number::from_i128(*i as i128).ok_or(
                    "Failed to convert Scalar::Int to serde_json::Number",
                )?,
            )),
            Self::Float(f) => Ok(serde_json_path_to_error::Value::Number(
                serde_json_path_to_error::Number::from_f64(*f as f64).ok_or(
                    "Failed to convert Scalar::Float to serde_json::Number",
                )?,
            )),
            Self::Boolean(b) => Ok(serde_json_path_to_error::Value::Bool(*b)),
            Self::String(s) => Ok(serde_json_path_to_error::Value::String(s.clone())),
        }
    }
}

#[libgqlcodegen::macros::gql_scalar_resolver_value]
impl libgql::executor::GQLScalar<Scalar> for i32 {
    fn from_scalar(s: Scalar) -> Result<Self, String> {
        match s {
            Scalar::Int(i) => Ok(i),
            _ => Err(format!("Invalid scalar for i32 {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<Scalar, String> {
        Ok(Scalar::Int(*self))
    }
}

#[libgqlcodegen::macros::gql_scalar_resolver_value]
impl libgql::executor::GQLScalar<Scalar> for f32 {
    fn from_scalar(s: Scalar) -> Result<Self, String> {
        match s {
            Scalar::Float(f) => Ok(f),
            _ => Err(format!("Invalid scalar for f32 {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<Scalar, String> {
        Ok(Scalar::Float(*self))
    }
}

#[libgqlcodegen::macros::gql_scalar_resolver_value]
impl libgql::executor::GQLScalar<Scalar> for String {
    fn from_scalar(s: Scalar) -> Result<Self, String> {
        match s {
            Scalar::String(s) => Ok(s),
            _ => Err(format!("Invalid scalar for String {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<Scalar, String> {
        Ok(Scalar::String(self.clone()))
    }
}

#[libgqlcodegen::macros::gql_scalar_resolver_value]
impl libgql::executor::GQLScalar<Scalar> for bool {
    fn from_scalar(s: Scalar) -> Result<Self, String> {
        match s {
            Scalar::Boolean(b) => Ok(b),
            _ => Err(format!("Invalid scalar for bool {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<Scalar, String> {
        Ok(Scalar::Boolean(*self))
    }
}
