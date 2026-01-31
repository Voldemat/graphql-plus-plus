use std::collections::HashMap;

use crate::parsers::schema::shared;

use super::registry::Registry;
use super::scalar::Scalar;
use super::variables::Variables;

pub trait GQLScalar<S: Scalar>: Sized
where
    Self: 'static,
{
    fn from_scalar(s: &S) -> Result<Self, String>;
    fn from_scalar_to_any(s: &S) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_scalar(s).map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
    fn from_scalar_array(scalars: &[&S]) -> Result<Vec<Self>, String> {
        scalars
            .iter()
            .map(|s| Self::from_scalar(s))
            .collect::<Result<Vec<_>, String>>()
    }
    fn from_scalar_array_to_any(
        s: &[&S],
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_scalar_array(s)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub trait GQLEnum: Sized
where
    Self: 'static,
{
    fn from_str(s: &str) -> Result<Self, String>;
    fn from_str_to_any(s: &str) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_str(s).map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
    fn from_str_array(s: &[&str]) -> Result<Vec<Self>, String> {
        s.iter()
            .map(|e| Self::from_str(e))
            .collect::<Result<Vec<_>, String>>()
    }
    fn from_str_array_to_any(
        s: &[&str],
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_str_array(s)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub trait GQLInput<S: Scalar>: Sized
where
    Self: 'static,
{
    fn from_variables(vars: &Variables<S>) -> Result<Self, String>;
    fn from_variables_to_any(
        vars: &Variables<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_variables(vars)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
    fn from_variables_array(
        vars: &[&Variables<S>],
    ) -> Result<Vec<Self>, String> {
        vars.iter()
            .map(|s| Self::from_variables(s))
            .collect::<Result<Vec<_>, String>>()
    }
    fn from_variables_to_any_array(
        vars: &[&Variables<S>],
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_variables_array(vars)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub struct HashMapRegistry<S: Scalar> {
    pub scalars: HashMap<
        String,
        Box<dyn Fn(&S) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub enum_types: HashMap<
        String,
        Box<dyn Fn(&str) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub inputs: HashMap<
        String,
        Box<dyn Fn(&Variables<S>) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub scalars_array: HashMap<
        String,
        Box<dyn Fn(&[&S]) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub enum_types_array: HashMap<
        String,
        Box<dyn Fn(&[&str]) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub inputs_array: HashMap<
        String,
        Box<dyn Fn(&[&Variables<S>]) -> Result<Box<dyn std::any::Any>, String>>,
    >,
}

impl<S: Scalar> Default for HashMapRegistry<S> {
    fn default() -> Self {
        Self {
            scalars: HashMap::default(),
            enum_types: HashMap::default(),
            inputs: HashMap::default(),
            scalars_array: HashMap::default(),
            enum_types_array: HashMap::default(),
            inputs_array: HashMap::default(),
        }
    }
}

impl<S: Scalar> Registry<S> for HashMapRegistry<S> {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: &S,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars.get(scalar_name).unwrap()(value)
    }

    fn parse_scalar_array(
        self: &Self,
        scalar_name: &str,
        values: &[&S],
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars_array.get(scalar_name).unwrap()(values)
    }

    fn parse_enum(
        self: &Self,
        enum_type: &shared::ast::Enum,
        value: &str,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types.get(&enum_type.name).unwrap()(value)
    }

    fn parse_enum_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: &[&str],
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types_array.get(&enum_type.name).unwrap()(values)
    }

    fn parse_input(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &Variables<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs.get(&input_type.name).unwrap()(value)
    }

    fn parse_input_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        values: &[&Variables<S>],
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs_array.get(&input_type.name).unwrap()(values)
    }
}
