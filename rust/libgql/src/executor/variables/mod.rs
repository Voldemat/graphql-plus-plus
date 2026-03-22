pub mod array;

use std::collections::HashMap;

use indexmap::IndexMap;

use crate::parsers::schema::shared;

use super::ast::{LiteralValue, NonNullableValue, Value, Values};
use super::registry::TypeRegistry;
use super::scalar::Scalar;

pub type ResolvedVariable = Box<dyn std::any::Any>;
pub type ResolvedVariables = HashMap<String, ResolvedVariable>;

fn resolve_type_spec<S: Scalar, R: TypeRegistry<S>>(
    registry: &R,
    spec: &shared::ast::InputTypeSpec,
    variable: LiteralValue<S>,
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
        ) => scalar
            .try_to_string()
            .map(|s| R::parse_enum(registry, enum_type, s))
            .flatten(),
        (shared::ast::InputTypeSpec::Enum(enum_type), other) => Err(format!(
            "Received invalid type for enum({}): {:?}",
            enum_type.name, other
        )),
        (
            shared::ast::InputTypeSpec::InputType(input_type),
            LiteralValue::Object(object),
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

fn resolve_literal<S: Scalar, R: TypeRegistry<S>>(
    registry: &R,
    spec: &shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>,
    var: LiteralValue<S>,
) -> Result<ResolvedVariable, String> {
    return resolve_type_spec(registry, &spec.r#type, var);
}

fn resolve_operation_parameter<S: Scalar, R: TypeRegistry<S>>(
    registry: &R,
    param: &shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    variable: Value<S>,
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
                        LiteralValue::Scalar(S::from_literal(
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
                Ok(Some(array::resolve_array(registry, spec, array)?))
            } else {
                Err(format!(
                    "Expected array for parameter: {}, received: {:?}",
                    param.name, nonnullable_variable
                ))
            }
        }
        shared::ast::NonCallableFieldSpec::Literal(spec) => {
            if let NonNullableValue::Literal(l) = nonnullable_variable {
                Ok(Some(resolve_literal(registry, spec, l)?))
            } else {
                Err(format!(
                    "Expected literal variable for parameter: {}, received: {:?}",
                    param.name, nonnullable_variable
                ))
            }
        }
    }
}

pub fn resolve_operation_parameters<S: Scalar, R: TypeRegistry<S>>(
    registry: &R,
    op_parameters: &IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
    mut variables: Values<S>,
) -> Result<ResolvedVariables, String> {
    let mut vars = ResolvedVariables::new();
    for param in op_parameters.values() {
        if let Some(variable) = variables.remove(&param.name[1..]) {
            if let Some(resolved_variable) =
                resolve_operation_parameter(registry, param, variable)?
            {
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
