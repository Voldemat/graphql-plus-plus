use std::collections::HashMap;

use crate::parsers::schema::shared;

use super::scalar::Scalar;
use super::variables::Variables;

pub trait Registry<S: Scalar> {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: &S,
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_scalar_array(
        self: &Self,
        scalar_name: &str,
        values: &[&S],
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_enum(
        self: &Self,
        enum_type: &shared::ast::Enum,
        value: &str,
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_enum_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: &[&str],
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_input(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &Variables<S>,
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_input_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &[&Variables<S>],
    ) -> Result<Box<dyn std::any::Any>, String>;
}
