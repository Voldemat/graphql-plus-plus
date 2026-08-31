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
        literal: libgql::parsers::schema::shared::ast::traits::LiteralRef<'_>,
    ) -> Result<Option<Scalar>, String> {
        match literal {
            libgql::parsers::schema::shared::ast::traits::LiteralRef::Null => Ok(None),
            libgql::parsers::schema::shared::ast::traits::LiteralRef::EnumValue(_) => Err("Unexpected enum value for scalar".to_string()),
            libgql::parsers::schema::shared::ast::traits::LiteralRef::Int(i) => Ok(Some(Scalar::Int(
                TryInto::<i32>::try_into(*i).map_err(|e| e.to_string())?,
            ))),
            libgql::parsers::schema::shared::ast::traits::LiteralRef::Float(f) => Ok(Some(Scalar::Float(*f as f32))),
            libgql::parsers::schema::shared::ast::traits::LiteralRef::String(s) => {
                Ok(Some(Scalar::String(s.to_string())))
            }
            libgql::parsers::schema::shared::ast::traits::LiteralRef::Boolean(b) => Ok(Some(Scalar::Boolean(*b))),
        }
    }
}

impl libgql::json::executor::ast::JSONParsableScalar for Scalar {
    fn from_json_scalar<'a>(
        json_scalar: libgql::json::executor::ast::JSONScalar<'a>,
    ) -> Result<Scalar, String> {
        match json_scalar {
            libgql::json::executor::ast::JSONScalar::Bool(b) => {
                Ok(Scalar::Boolean(b))
            }
            libgql::json::executor::ast::JSONScalar::String(s) => {
                Ok(Scalar::String(s.to_string()))
            }
            libgql::json::executor::ast::JSONScalar::Number(n) => {
                if let Some(u64_n) = n.as_u64() {
                    Ok(Scalar::Int(
                        TryInto::<i32>::try_into(u64_n)
                            .map_err(|e| e.to_string())?,
                    ))
                } else if let Some(i64_n) = n.as_i64() {
                    Ok(Scalar::Int(
                        TryInto::<i32>::try_into(i64_n)
                            .map_err(|e| e.to_string())?,
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
    fn to_json_value(self: &Self) -> Result<serde_json::Value, String> {
        match self {
            Self::Int(i) => Ok(serde_json::Value::Number(
                serde_json::Number::from_i128(*i as i128).ok_or(
                    "Failed to convert Scalar::Int to serde_json::Number",
                )?,
            )),
            Self::Float(f) => Ok(serde_json::Value::Number(
                serde_json::Number::from_f64(*f as f64).ok_or(
                    "Failed to convert Scalar::Float to serde_json::Number",
                )?,
            )),
            Self::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
            Self::String(s) => Ok(serde_json::Value::String(s.clone())),
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
impl libgql::executor::GQLScalar<Scalar> for uuid::Uuid {
    fn from_scalar(s: Scalar) -> Result<Self, String> {
        match s {
            Scalar::String(s) => {
                Self::parse_str(s.as_str()).map_err(|e| e.to_string())
            }
            _ => Err(format!("Invalid scalar for String {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<Scalar, String> {
        Ok(Scalar::String(self.to_string()))
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
