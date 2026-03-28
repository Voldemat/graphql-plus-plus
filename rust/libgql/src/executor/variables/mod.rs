pub mod array;

use std::collections::HashMap;

use indexmap::IndexMap;

use crate::parsers::schema::shared;

use super::ast::{LiteralValue, NonNullableValue, Value, Values};
use super::registry::{ParseRegistry, ResolvedVariable};
use super::scalar::Scalar;

pub type ResolvedVariables = HashMap<String, ResolvedVariable>;

fn resolve_type_spec<
    'buffer,
    S: Scalar,
    R: ParseRegistry<S>,
    StringType: shared::ast::AsStr<'buffer>,
>(
    registry: &R,
    spec: &shared::ast::InputTypeSpec<StringType>,
    variable: LiteralValue<S>,
) -> Result<ResolvedVariable, String> {
    match (spec, variable) {
        (
            shared::ast::InputTypeSpec::Scalar(scalar_name),
            LiteralValue::Scalar(scalar),
        ) => Ok(R::parse_scalar(registry, scalar_name.to_str(), scalar)
            .map_err(|e| format!("{}: {}", scalar_name.to_str(), e))?),
        (shared::ast::InputTypeSpec::Scalar(scalar_name), other) => {
            Err(format!(
                "Received invalid type for scalar({}): {:?}",
                scalar_name.to_str(),
                other
            ))
        }
        (
            shared::ast::InputTypeSpec::Enum(enum_type),
            LiteralValue::Scalar(scalar),
        ) => scalar
            .try_to_string()
            .map(|s| R::parse_enum(registry, enum_type.to_str(), s))
            .flatten(),
        (shared::ast::InputTypeSpec::Enum(enum_type), other) => Err(format!(
            "Received invalid type for enum({}): {:?}",
            enum_type.to_str(),
            other
        )),
        (
            shared::ast::InputTypeSpec::InputType(input_type),
            LiteralValue::Object(object),
        ) => {
            return Ok(R::parse_input(registry, input_type.to_str(), object)?);
        }
        (shared::ast::InputTypeSpec::InputType(input_type), other) => {
            Err(format!(
                "Received invalid type for input({}): {:?}",
                input_type.to_str(),
                other
            ))
        }
    }
}

fn resolve_literal<
    'buffer,
    S: Scalar,
    R: ParseRegistry<S>,
    StringType: shared::ast::AsStr<'buffer>,
>(
    registry: &R,
    spec: &shared::ast::LiteralFieldSpec<
        shared::ast::InputTypeSpec<StringType>,
        StringType,
    >,
    var: LiteralValue<S>,
) -> Result<ResolvedVariable, String> {
    return resolve_type_spec(registry, &spec.r#type, var);
}

fn resolve_operation_parameter<
    'buffer,
    S: Scalar,
    R: ParseRegistry<S>,
    StringType: shared::ast::AsStr<'buffer>,
>(
    registry: &R,
    param: &shared::ast::FieldDefinition<
        shared::ast::InputFieldSpec<StringType>,
        StringType,
    >,
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
                param.name.to_str()
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
                    param.name.to_str(),
                    nonnullable_variable
                ))
            }
        }
        shared::ast::NonCallableFieldSpec::Literal(spec) => {
            if let NonNullableValue::Literal(l) = nonnullable_variable {
                Ok(Some(resolve_literal(registry, spec, l)?))
            } else {
                Err(format!(
                    "Expected literal variable for parameter: {}, received: {:?}",
                    param.name.to_str(),
                    nonnullable_variable
                ))
            }
        }
    }
}

pub fn resolve_operation_parameters<
    'buffer,
    S: Scalar,
    R: ParseRegistry<S>,
    StringType: shared::ast::AsStr<'buffer>,
>(
    registry: &R,
    op_parameters: &IndexMap<
        StringType,
        shared::ast::FieldDefinition<
            shared::ast::InputFieldSpec<StringType>,
            StringType,
        >,
    >,
    mut variables: Values<S>,
) -> Result<ResolvedVariables, String> {
    let mut vars = ResolvedVariables::new();
    for param in op_parameters.values() {
        if let Some(variable) = variables.remove(&param.name.to_str()[1..]) {
            if let Some(resolved_variable) =
                resolve_operation_parameter(registry, param, variable)?
            {
                vars.insert(
                    param.name.to_str()[1..].to_string(),
                    resolved_variable,
                );
            }
            continue;
        }
        if !param.nullable {
            return Err(format!(
                "Required operation parameter {} is missing",
                param.name.to_str()
            ));
        }
    }
    Ok(vars)
}
