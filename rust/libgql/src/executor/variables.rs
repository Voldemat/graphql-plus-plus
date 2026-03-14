use std::collections::HashMap;

use indexmap::IndexMap;

use crate::parsers::schema::shared;

use super::ast::{LiteralValue, NonNullableValue, Value, Values};
use super::registry::Registry;
use super::scalar::Scalar;

pub type ResolvedVariable = Box<dyn std::any::Any>;
pub type ResolvedVariables = HashMap<String, ResolvedVariable>;

fn resolve_type_spec<S: Scalar, R: Registry<S>>(
    registry: &R,
    spec: &shared::ast::InputTypeSpec,
    variable: &LiteralValue<S>,
) -> Result<ResolvedVariable, String> {
    match (spec, variable) {
        (
            shared::ast::InputTypeSpec::Scalar(scalar_name),
            LiteralValue::Scalar(scalar),
        ) => Ok(R::parse_scalar(registry, scalar_name, scalar)?),
        (shared::ast::InputTypeSpec::Scalar(scalar_name), other) => {
            Err(format!(
                "Received invalid type for scalar({}): {:?}",
                scalar_name, other
            ))
        }
        (
            shared::ast::InputTypeSpec::Enum(enum_type),
            LiteralValue::Scalar(scalar),
        ) => {
            if let Some(enum_value) = scalar.get_str() {
                return Ok(R::parse_enum(registry, enum_type, enum_value)?);
            } else {
                Err(format!(
                    "Enum value must be string, received: {:?}",
                    scalar
                ))
            }
        }
        (shared::ast::InputTypeSpec::Enum(enum_type), other) => Err(format!(
            "Received invalid type for enum({}): {:?}",
            enum_type.name, other
        )),
        (
            shared::ast::InputTypeSpec::InputType(input_type),
            LiteralValue::Object(_, object),
        ) => {
            return Ok(R::parse_input(registry, &input_type.borrow(), object)?);
        }
        (shared::ast::InputTypeSpec::InputType(input_type), other) => {
            Err(format!(
                "Received invalid type for input({}): {:?}",
                input_type.borrow().name,
                other
            ))
        }
    }
}

fn resolve_literal<S: Scalar, R: Registry<S>>(
    registry: &R,
    spec: &shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>,
    var: &LiteralValue<S>,
) -> Result<ResolvedVariable, String> {
    return resolve_type_spec(registry, &spec.r#type, var);
}

fn resolve_literal_array<S: Scalar, R: Registry<S>>(
    registry: &R,
    literal_type: &shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>,
    nullable: bool,
    elements: &[Value<S>],
) -> Result<Box<dyn std::any::Any>, String> {
    match &literal_type.r#type {
        shared::ast::InputTypeSpec::Enum(e) => {
            R::parse_enum_array(registry, &e, &elements.iter().map(|e| {
                if let Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(scalar),
                )) = e && let Some(s) = scalar.get_str()
                {
                    Ok(s)
                } else {
                    Err(format!(
                        "Expected string scalar for enum value, received: {:?}",
                        e
                    ))
                }
            }).collect::<Result<Vec<&str>, String>>()?)
        }
        shared::ast::InputTypeSpec::Scalar(scalar_name) => {
            R::parse_scalar_array(registry, &scalar_name, &elements.iter().map(|e| {
                if let Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(scalar),
                )) = e
                {
                    Ok(scalar)
                } else {
                    Err(format!(
                        "Expected scalar, received: {:?}",
                        e
                    ))
                }
            }).collect::<Result<Vec<_>, String>>()?)
        }
        shared::ast::InputTypeSpec::InputType(input_type) => {
            R::parse_input_array(registry, &input_type.borrow(), &elements.iter().map(|e| {
                if let Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Object(_, object),
                )) = e
                {
                    Ok(object)
                } else {
                    Err(format!(
                        "Expected object, received: {:?}",
                        e
                    ))
                }
            }).collect::<Result<Vec<_>, String>>()?)
        }
    }
}

fn resolve_array<S: Scalar, R: Registry<S>>(
    registry: &R,
    array_type: &shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>,
    elements: &[Value<S>],
) -> Result<Box<dyn std::any::Any>, String> {
    match array_type.r#type.as_ref() {
        shared::ast::NonCallableFieldSpec::Literal(literal) => {
            resolve_literal_array(
                registry,
                literal,
                array_type.nullable,
                elements,
            )
        }
        shared::ast::NonCallableFieldSpec::Array(array) => {
            let mut a = Vec::new();
            for element in elements {
                let Value::NonNullable(NonNullableValue::Array(
                    nested_elements,
                )) = element
                else {
                    return Err("Unexpected value for nested array".into());
                };
                a.push(resolve_array(registry, array, &nested_elements)?);
            }
            Ok(Box::new(a))
        }
    }
}

fn resolve_operation_parameter<S: Scalar, R: Registry<S>>(
    registry: &R,
    param: &shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    variable: &Value<S>,
) -> Result<Option<ResolvedVariable>, String> {
    let Value::NonNullable(nonnullable_variable) = variable else {
        if param.nullable {
            return Ok(None);
        } else if param.spec.has_default_value() {
            match &param.spec {
                shared::ast::NonCallableFieldSpec::Array(_) => {
                    return Ok(None);
                }
                shared::ast::NonCallableFieldSpec::Literal(spec) => {
                    return Ok(Some(resolve_literal(
                        registry,
                        spec,
                        &LiteralValue::Scalar(S::from_literal(
                            &spec
                                .default_value
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unwrap(),
                        )?),
                    )?));
                }
            }
        } else {
            return Err(format!(
                "Received null for nonnullable parameter: {}",
                param.name
            ));
        }
    };
    match &param.spec {
        shared::ast::NonCallableFieldSpec::Array(spec) => {
            if let NonNullableValue::Array(array) = nonnullable_variable {
                Ok(Some(resolve_array(registry, spec, array)?))
            } else {
                Err(format!(
                    "Expected array for parameter: {}, received: {:?}",
                    param.name, variable
                ))
            }
        }
        shared::ast::NonCallableFieldSpec::Literal(spec) => {
            if let NonNullableValue::Literal(l) = nonnullable_variable {
                Ok(Some(resolve_literal(registry, spec, l)?))
            } else {
                Err(format!(
                    "Expected literal variable for parameter: {}, received: {:?}",
                    param.name, variable
                ))
            }
        }
    }
}

pub fn resolve_operation_parameters<S: Scalar, R: Registry<S>>(
    registry: &R,
    op_parameters: &IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
    variables: &Values<S>,
) -> Result<ResolvedVariables, String> {
    let mut vars = ResolvedVariables::new();
    for param in op_parameters.values() {
        if let Some(variable) = variables.get(&param.name[1..]) {
            if let Some(resolved_variable) = resolve_operation_parameter(
                registry,
                param,
                variable,
            )? {
                vars.insert(param.name[1..].to_string(), resolved_variable);
            }
            continue;
        }
        if !param.nullable {
            return Err(format!(
                "Required operation parameter {} is missing",
                param.name
            ));
        }
    }
    Ok(vars)
}
