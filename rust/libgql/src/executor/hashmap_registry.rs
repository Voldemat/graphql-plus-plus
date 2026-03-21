use std::collections::HashMap;

use crate::parsers::schema::shared;

use super::ast::{LiteralValue, NonNullableValue, Value, Values};
use super::registry::TypeRegistry;
use super::scalar::Scalar;

pub trait GQLScalar<S: Scalar>: Sized
where
    Self: 'static,
{
    fn from_scalar(s: S) -> Result<Self, String>;
    fn to_scalar(self: &Self) -> Result<S, String>;

    fn to_value(self: &Self) -> Result<Value<S>, String> {
        self.to_non_nullable_value().map(|v| Value::NonNullable(v))
    }

    fn to_non_nullable_value(
        self: &Self,
    ) -> Result<NonNullableValue<S>, String> {
        self.to_literal_value()
            .map(|v| NonNullableValue::Literal(v))
    }

    fn to_literal_value(self: &Self) -> Result<LiteralValue<S>, String> {
        self.to_scalar().map(|scalar| LiteralValue::Scalar(scalar))
    }

    fn from_literal_value(value: LiteralValue<S>) -> Result<Self, String> {
        match value {
            LiteralValue::Scalar(scalar) => Self::from_scalar(scalar),
            LiteralValue::Object(_, object) => {
                Err(format!("Unexpected object value for scalar: {:?}", object))
            }
        }
    }

    fn from_non_nullable_value(
        value: NonNullableValue<S>,
    ) -> Result<Self, String> {
        match value {
            NonNullableValue::Literal(literal) => {
                Self::from_literal_value(literal)
            }
            NonNullableValue::Array(array) => {
                Err(format!("Unexpected array value for scalar: {:?}", array))
            }
        }
    }

    fn from_value(value: Value<S>) -> Result<Option<Self>, String> {
        match value {
            Value::Null => Ok(None),
            Value::NonNullable(non_nullable) => {
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
    fn to_str(self: &Self) -> Result<&'static str, String>;

    fn to_value(self: &Self) -> Result<Value<S>, String> {
        self.to_non_nullable_value().map(|v| Value::NonNullable(v))
    }

    fn to_non_nullable_value(
        self: &Self,
    ) -> Result<NonNullableValue<S>, String> {
        self.to_literal_value()
            .map(|v| NonNullableValue::Literal(v))
    }

    fn to_literal_value(self: &Self) -> Result<LiteralValue<S>, String> {
        self.to_str()
            .map(S::from_str)
            .flatten()
            .map(|scalar| LiteralValue::Scalar(scalar))
    }

    fn from_literal_value(value: LiteralValue<S>) -> Result<Self, String> {
        match value {
            LiteralValue::Scalar(scalar) => scalar
                .try_to_string()
                .map_err(|_| {
                    "Unexpected non-string scalar for enum".to_string()
                })
                .map(Self::from_string)
                .flatten(),
            LiteralValue::Object(_, object) => {
                Err(format!("Unexpected object value for enum: {:?}", object))
            }
        }
    }

    fn from_non_nullable_value(
        value: NonNullableValue<S>,
    ) -> Result<Self, String> {
        match value {
            NonNullableValue::Literal(literal) => {
                Self::from_literal_value(literal)
            }
            NonNullableValue::Array(array) => {
                Err(format!("Unexpected array value for enum: {:?}", array))
            }
        }
    }

    fn from_value(value: Value<S>) -> Result<Option<Self>, String> {
        match value {
            Value::Null => Ok(None),
            Value::NonNullable(non_nullable) => {
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

    fn from_literal_value(value: LiteralValue<S>) -> Result<Self, String> {
        match value {
            LiteralValue::Object(_, object) => Self::from_variables(object),
            LiteralValue::Scalar(scalar) => {
                Err(format!("Unexpected scalar value for input: {:?}", scalar))
            }
        }
    }

    fn from_non_nullable_value(
        value: NonNullableValue<S>,
    ) -> Result<Self, String> {
        match value {
            NonNullableValue::Literal(literal) => {
                Self::from_literal_value(literal)
            }
            NonNullableValue::Array(array) => {
                Err(format!("Unexpected array value for input: {:?}", array))
            }
        }
    }

    fn from_value(value: Value<S>) -> Result<Option<Self>, String> {
        match value {
            Value::Null => Ok(None),
            Value::NonNullable(non_nullable) => {
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

    fn from_variables_array_to_any(
        vars: Vec<Values<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_variables_array(vars)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }

    fn from_variables_optional_array(
        vars: Vec<Option<Values<S>>>,
    ) -> Result<Vec<Option<Self>>, String> {
        vars.into_iter()
            .map(|o| o.map(|s| Self::from_variables(s)).transpose())
            .collect::<Result<Vec<_>, String>>()
    }

    fn from_variables_optional_array_to_any(
        vars: Vec<Option<Values<S>>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        Self::from_variables_optional_array(vars)
            .map(|v| -> Box<dyn std::any::Any> { Box::new(v) })
    }
}

pub struct ScalarHooks<S: Scalar> {
    literal: Box<dyn Fn(S) -> Result<Box<dyn std::any::Any>, String>>,
    array: Box<dyn Fn(Vec<S>) -> Result<Box<dyn std::any::Any>, String>>,
    optional_array:
        Box<dyn Fn(Vec<Option<S>>) -> Result<Box<dyn std::any::Any>, String>>,
}

pub struct EnumHooks {
    literal: Box<dyn Fn(String) -> Result<Box<dyn std::any::Any>, String>>,
    array: Box<dyn Fn(Vec<String>) -> Result<Box<dyn std::any::Any>, String>>,
    optional_array: Box<
        dyn Fn(Vec<Option<String>>) -> Result<Box<dyn std::any::Any>, String>,
    >,
}

pub struct InputHooks<S: Scalar> {
    literal: Box<dyn Fn(Values<S>) -> Result<Box<dyn std::any::Any>, String>>,
    array:
        Box<dyn Fn(Vec<Values<S>>) -> Result<Box<dyn std::any::Any>, String>>,
    optional_array: Box<
        dyn Fn(
            Vec<Option<Values<S>>>,
        ) -> Result<Box<dyn std::any::Any>, String>,
    >,
}

pub struct HashMapRegistry<S: Scalar> {
    pub scalars: HashMap<String, ScalarHooks<S>>,
    pub enum_types: HashMap<String, EnumHooks>,
    pub inputs: HashMap<String, InputHooks<S>>,
}

impl<S: Scalar> Default for HashMapRegistry<S> {
    fn default() -> Self {
        Self {
            scalars: HashMap::default(),
            enum_types: HashMap::default(),
            inputs: HashMap::default(),
        }
    }
}

impl<S: Scalar + 'static> HashMapRegistry<S> {
    pub fn add_scalar<T: GQLScalar<S>>(self: &mut Self, name: &str) {
        self.scalars.insert(
            name.into(),
            ScalarHooks {
                literal: Box::new(T::from_scalar_to_any),
                array: Box::new(T::from_scalar_array_to_any),
                optional_array: Box::new(T::from_optional_scalar_array_to_any),
            },
        );
    }

    pub fn add_enum<T: GQLEnum<S>>(self: &mut Self, name: &str) {
        self.enum_types.insert(
            name.into(),
            EnumHooks {
                literal: Box::new(T::from_str_to_any),
                array: Box::new(T::from_str_array_to_any),
                optional_array: Box::new(T::from_optional_str_array_to_any),
            },
        );
    }

    pub fn add_input<T: GQLInput<S>>(self: &mut Self, name: &str) {
        self.inputs.insert(
            name.into(),
            InputHooks {
                literal: Box::new(T::from_variables_to_any),
                array: Box::new(T::from_variables_array_to_any),
                optional_array: Box::new(
                    T::from_variables_optional_array_to_any,
                ),
            },
        );
    }
}

impl<S: Scalar> TypeRegistry<S> for HashMapRegistry<S> {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: S,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars[scalar_name].literal.as_ref()(value)
    }

    fn parse_scalar_array(
        self: &Self,
        scalar_name: &str,
        values: Vec<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars[scalar_name].array.as_ref()(values)
    }

    fn parse_scalar_optional_array(
        self: &Self,
        scalar_name: &str,
        values: Vec<Option<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars[scalar_name].optional_array.as_ref()(values)
    }

    fn parse_enum(
        self: &Self,
        enum_type: &shared::ast::Enum,
        value: String,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types[&enum_type.name].literal.as_ref()(value)
    }

    fn parse_enum_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: Vec<String>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types[&enum_type.name].array.as_ref()(values)
    }

    fn parse_enum_optional_array(
        self: &Self,
        enum_type: &shared::ast::Enum,
        values: Vec<Option<String>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types[&enum_type.name].optional_array.as_ref()(values)
    }

    fn parse_input(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: Values<S>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs[&input_type.name].literal.as_ref()(value)
    }

    fn parse_input_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        values: Vec<Values<S>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs[&input_type.name].array.as_ref()(values)
    }

    fn parse_input_optional_array(
        self: &Self,
        input_type: &shared::ast::InputType,
        values: Vec<Option<Values<S>>>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs[&input_type.name].optional_array.as_ref()(values)
    }
}
