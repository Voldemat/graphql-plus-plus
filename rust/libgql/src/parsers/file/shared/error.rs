use crate::lexer::tokens::TokenLocation;

pub trait Error: std::fmt::Display + std::fmt::Debug {
    fn get_location(self: &Self) -> &TokenLocation;
}
