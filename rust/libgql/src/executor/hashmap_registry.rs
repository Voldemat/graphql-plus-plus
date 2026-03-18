use std::collections::HashMap;

use crate::parsers::schema::shared;

use super::ast::Values;
use super::registry::Registry;
use super::scalar::Scalar;

pub trait GQLScalar<S: Scalar>: Sized
where
    Self: 'static,
{
    fn from_scalar(s: S) -> Result<Self, String>;
    fn to_scalar(self: Self) -> Result<S, String>;

    fn to_value(self: Self) -> Result<super::ast::Value<S>, String> {
        self.to_scalar().map(|scalar| {
            super::ast::Value::NonNullable(
                super::ast::NonNullableValue::Literal(
                    super::ast::LiteralValue::Scalar(scalar),
                ),
            )
        })
    }

    fn from_literal_value(
        value: super::ast::LiteralValue<S>,
    ) -> Result<Self, String> {
        match value {
            super::ast::LiteralValue::Scalar(scalar) => {
                Self::from_scalar(scalar)
            }
            super::ast::LiteralValue::Object(_, object) => {
                Err(format!("Unexpected object value for scalar: {:?}", object))
            }
        }
    }

    fn from_non_nullable_value(
        value: super::ast::NonNullableValue<S>,
    ) -> Result<Self, String> {
        match value {
            super::ast::NonNullableValue::Literal(literal) => {
                Self::from_literal_value(literal)
            }
            super::ast::NonNullableValue::Array(array) => {
                Err(format!("Unexpected array value for scalar: {:?}", array))
            }
        }
    }

    fn from_value(value: super::ast::Value<S>) -> Result<Option<Self>, String> {
        match value {
            super::ast::Value::Null => Ok(None),
            super::ast::Value::NonNullable(non_nullable) => {
                Self::from_non_nullable_value(non_nullable).map(|v| Some(v))
            }
        }
    }

    fn from_scalar_to_any(s: S) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_scalar(s).map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
    fn from_scalar_array(scalars: Vec<S>) -> Result<Vec<Self>, String> {
        scalars
            .into_iter()
            .map(|s| Self::from_scalar(s))
            .collect::<Result<Vec<_>, String>>()
    }
    fn from_scalar_array_to_any(
        s: Vec<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_scalar_array(s)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }

    fn from_optional_scalar_array(
        scalars: Vec<Option<S>>,
    ) -> Result<Vec<Option<Self>>, String> {
        scalars
            .into_iter()
            .map(|o| o.map(|s| Self::from_scalar(s)).transpose())
            .collect::<Result<Vec<_>, String>>()
    }

    fn from_optional_scalar_array_to_any(
        s: Vec<Option<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_optional_scalar_array(s)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub trait GQLEnum<S: Scalar>: Sized
where
    Self: 'static,
{
    fn from_string(s: String) -> Result<Self, String>;
    fn to_str(self: Self) -> Result<&'static str, String>;
    fn to_value(self: Self) -> Result<super::ast::Value<S>, String> {
        self.to_str().map(S::from_str).flatten().map(|scalar| {
            super::ast::Value::NonNullable(
                super::ast::NonNullableValue::Literal(
                    super::ast::LiteralValue::Scalar(scalar),
                ),
            )
        })
    }

    fn from_literal_value(
        value: super::ast::LiteralValue<S>,
    ) -> Result<Self, String> {
        match value {
            super::ast::LiteralValue::Scalar(scalar) => scalar
                .try_to_string()
                .map_err(|_| {
                    "Unexpected non-string scalar for enum".to_string()
                })
                .map(Self::from_string)
                .flatten(),
            super::ast::LiteralValue::Object(_, object) => {
                Err(format!("Unexpected object value for enum: {:?}", object))
            }
        }
    }

    fn from_non_nullable_value(
        value: super::ast::NonNullableValue<S>,
    ) -> Result<Self, String> {
        match value {
            super::ast::NonNullableValue::Literal(literal) => {
                Self::from_literal_value(literal)
            }
            super::ast::NonNullableValue::Array(array) => {
                Err(format!("Unexpected array value for enum: {:?}", array))
            }
        }
    }

    fn from_value(value: super::ast::Value<S>) -> Result<Option<Self>, String> {
        match value {
            super::ast::Value::Null => Ok(None),
            super::ast::Value::NonNullable(non_nullable) => {
                Self::from_non_nullable_value(non_nullable).map(|v| Some(v))
            }
        }
    }

    fn from_str_to_any(s: String) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_string(s).map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
    fn from_str_array(s: Vec<String>) -> Result<Vec<Self>, String> {
        s.into_iter()
            .map(|e| Self::from_string(e))
            .collect::<Result<Vec<_>, String>>()
    }
    fn from_str_array_to_any(
        s: Vec<String>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_str_array(s)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }

    fn from_optional_str_array(
        values: Vec<Option<String>>,
    ) -> Result<Vec<Option<Self>>, String> {
        values
            .into_iter()
            .map(|o| o.map(|s| Self::from_string(s)).transpose())
            .collect::<Result<Vec<_>, String>>()
    }

    fn from_optional_str_array_to_any(
        s: Vec<Option<String>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_optional_str_array(s)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub trait GQLInput<S: Scalar>: Sized
where
    Self: 'static,
{
    fn from_variables(vars: Values<S>) -> Result<Self, String>;

    fn from_literal_value(
        value: super::ast::LiteralValue<S>,
    ) -> Result<Self, String> {
        match value {
            super::ast::LiteralValue::Object(_, object) => {
                Self::from_variables(object)
            }
            super::ast::LiteralValue::Scalar(scalar) => {
                Err(format!("Unexpected scalar value for input: {:?}", scalar))
            }
        }
    }

    fn from_non_nullable_value(
        value: super::ast::NonNullableValue<S>,
    ) -> Result<Self, String> {
        match value {
            super::ast::NonNullableValue::Literal(literal) => {
                Self::from_literal_value(literal)
            }
            super::ast::NonNullableValue::Array(array) => {
                Err(format!("Unexpected array value for input: {:?}", array))
            }
        }
    }

    fn from_value(value: super::ast::Value<S>) -> Result<Option<Self>, String> {
        match value {
            super::ast::Value::Null => Ok(None),
            super::ast::Value::NonNullable(non_nullable) => {
                Self::from_non_nullable_value(non_nullable).map(|v| Some(v))
            }
        }
    }

    fn from_variables_to_any(
        vars: Values<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_variables(vars)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
    fn from_variables_array(vars: Vec<Values<S>>) -> Result<Vec<Self>, String> {
        vars.into_iter()
            .map(|s| Self::from_variables(s))
            .collect::<Result<Vec<_>, String>>()
    }
    fn from_variables_to_any_array(
        vars: Vec<Values<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_variables_array(vars)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub struct HashMapRegistry<S: Scalar> {
    pub scalars: HashMap<
        String,
        Box<dyn Fn(S) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub scalars_array: HashMap<
        String,
        Box<dyn Fn(Vec<S>) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub scalars_optional_array: HashMap<
        String,
        Box<dyn Fn(Vec<Option<S>>) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub enum_types: HashMap<
        String,
        Box<dyn Fn(String) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub enum_types_array: HashMap<
        String,
        Box<dyn Fn(Vec<String>) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub enum_types_optional_array: HashMap<
        String,
        Box<
            dyn Fn(
                Vec<Option<String>>,
            ) -> Result<Box<dyn std::any::Any>, String>,
        >,
    >,
    pub inputs: HashMap<
        String,
        Box<dyn Fn(Values<S>) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub inputs_array: HashMap<
        String,
        Box<dyn Fn(Vec<Values<S>>) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    pub inputs_optional_array: HashMap<
        String,
        Box<
            dyn Fn(
                Vec<Option<Values<S>>>,
            ) -> Result<Box<dyn std::any::Any>, String>,
        >,
    >,
}

impl<S: Scalar> Default for HashMapRegistry<S> {
    fn default() -> Self {
        Self {
            scalars: HashMap::default(),
            scalars_array: HashMap::default(),
            scalars_optional_array: HashMap::default(),
            enum_types: HashMap::default(),
            enum_types_array: HashMap::default(),
            enum_types_optional_array: HashMap::default(),
            inputs: HashMap::default(),
            inputs_array: HashMap::default(),
            inputs_optional_array: HashMap::default(),
        }
    }
}

impl<S: Scalar + 'static> HashMapRegistry<S> {
    pub fn add_scalar<T: GQLScalar<S>>(self: &mut Self, name: &str) {
        self.scalars
            .insert(name.into(), Box::new(T::from_scalar_to_any));
        self.scalars_array
            .insert(name.into(), Box::new(T::from_scalar_array_to_any));
        self.scalars_optional_array.insert(
            name.into(),
            Box::new(T::from_optional_scalar_array_to_any),
        );
    }

    pub fn add_enum<T: GQLEnum<S>>(self: &mut Self, name: &str) {
        self.enum_types
            .insert(name.into(), Box::new(T::from_str_to_any));
        self.enum_types_array
            .insert(name.into(), Box::new(T::from_str_array_to_any));
    }

    pub fn add_input<T: GQLInput<S>>(self: &mut Self, name: &str) {
        self.inputs
            .insert(name.into(), Box::new(T::from_variables_to_any));
        self.inputs_array
            .insert(name.into(), Box::new(T::from_variables_to_any_array));
    }
}

impl<S: Scalar> Registry<S> for HashMapRegistry<S> {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: S,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars.get(scalar_name).unwrap()(value)
    }

    fn parse_scalar_array(
        self: &Self,
        scalar_name: &str,
        values: Vec<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars_array.get(scalar_name).unwrap()(values)
    }

    fn parse_scalar_optional_array(
        self: &Self,
        scalar_name: &str,
        values: Vec<Option<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars_optional_array.get(scalar_name).unwrap()(values)
    }

    fn parse_enum(
        self: &Self,
        enum_type: &shared::ast::Enum,
        value: String,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types.get(&enum_type.name).unwrap()(value)
    }

    fn parse_enum_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: Vec<String>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types_array.get(&enum_type.name).unwrap()(values)
    }

    fn parse_enum_optional_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: Vec<Option<String>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types_optional_array.get(&enum_type.name).unwrap()(values)
    }

    fn parse_input(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: Values<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs
            .get(&input_type.name)
            .ok_or(format!("Failed to find {} input parser", input_type.name))?(
            value,
        )
    }

    fn parse_input_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        values: Vec<Values<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs_array.get(&input_type.name).unwrap()(values)
    }

    fn parse_input_optional_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        values: Vec<Option<Values<S>>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs_optional_array.get(&input_type.name).unwrap()(values)
    }
}
