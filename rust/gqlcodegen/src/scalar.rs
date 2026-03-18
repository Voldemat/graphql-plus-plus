#[derive(Debug)]
pub enum ExampleScalar {
    String(String),
    Int(i32),
    Int64(i64),
    Float(f32),
    Boolean(bool),
}

impl libgql::executor::Scalar for ExampleScalar {
    fn try_to_string(self: Self) -> Result<String, String> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err("Invalid scalar for string".to_string()),
        }
    }

    fn from_str(str: &str) -> Result<Self, String> {
        Ok(Self::String(str.to_string()))
    }

    fn from_literal(
        literal: &libgql::parsers::schema::shared::ast::Literal,
    ) -> Result<ExampleScalar, String> {
        match literal {
            libgql::parsers::schema::shared::ast::Literal::Int(i) => {
                Ok(ExampleScalar::Int(
                    TryInto::<i32>::try_into(*i).map_err(|e| e.to_string())?,
                ))
            }
            libgql::parsers::schema::shared::ast::Literal::Float(f) => {
                Ok(ExampleScalar::Float(*f as f32))
            }
            libgql::parsers::schema::shared::ast::Literal::String(s) => {
                Ok(ExampleScalar::String(s.to_string()))
            }
            libgql::parsers::schema::shared::ast::Literal::Boolean(b) => {
                Ok(ExampleScalar::Boolean(*b))
            }
        }
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for i32 {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Int(i) => Ok(i),
            _ => Err(format!("Invalid scalar for i32 {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Int(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for f32 {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Float(f) => Ok(f),
            _ => Err(format!("Invalid scalar for f32 {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Float(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for String {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(s) => Ok(s),
            _ => Err(format!("Invalid scalar for String {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for i64 {
    fn from_scalar(scalar: ExampleScalar) -> Result<Self, String> {
        match scalar {
            ExampleScalar::Int64(i) => Ok(i),
            _ => Err(format!("Invalid scalar for Int64 {:?}", scalar)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Int64(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar>
    for chrono::DateTime<chrono::Utc>
{
    fn from_scalar(scalar: ExampleScalar) -> Result<Self, String> {
        match scalar {
            ExampleScalar::String(s) => {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| e.to_string())
                    .map(|v| v.with_timezone(&chrono::Utc))
            }
            _ => Err(format!("Invalid scalar for Datetime {:?}", scalar)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self.to_rfc3339()))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for uuid::Uuid {
    fn from_scalar(scalar: ExampleScalar) -> Result<Self, String> {
        match scalar {
            ExampleScalar::String(s) => {
                uuid::Uuid::parse_str(&s).map_err(|e| e.to_string())
            }
            _ => Err(format!("Invalid scalar for UUID {:?}", scalar)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self.to_string()))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for bool {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Boolean(b) => Ok(b),
            _ => Err(format!("Invalid scalar for bool {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Boolean(self))
    }
}
