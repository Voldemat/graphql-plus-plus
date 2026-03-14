use crate::parsers::schema::shared;

use super::ast::Values;
use super::scalar::Scalar;

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
    fn parse_scalar_optional_array(
        self: &Self,
        scalar_name: &str,
        values: &[Option<&S>],
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
    fn parse_enum_optional_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: &[Option<&str>],
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_input(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &Values<S>,
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_input_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &[&Values<S>],
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_input_optional_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &[Option<&Values<S>>],
    ) -> Result<Box<dyn std::any::Any>, String>;
}
