use crate::parsers::schema::shared;

pub trait Scalar: Sized + std::fmt::Debug {
    fn get_str(self: &Self) -> Option<&str>;
    fn from_string(str: &str) -> Result<Self, String>;
    fn from_literal(literal: &shared::ast::Literal) -> Result<Self, String>;
}
