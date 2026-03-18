use crate::parsers::schema::shared;

pub trait Scalar: Sized + std::fmt::Debug {
    fn try_to_string(self: Self) -> Result<String, String>;
    fn from_str(s: &str) -> Result<Self, String>;
    fn from_literal(literal: &shared::ast::Literal) -> Result<Self, String>;
}
