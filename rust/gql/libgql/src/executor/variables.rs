use std::collections::HashMap;

use indexmap::IndexMap;

use crate::parsers::schema::shared;

use super::registry::Registry;
use super::scalar::Scalar;

#[derive(Debug)]
pub enum Variable<S: Scalar> {
    Null,
    NonNullable(NonNullableVariable<S>),
}

impl<S: Scalar> Variable<S> {
    pub fn get_str(self: &Self) -> Option<&str> {
        match self {
            Variable::NonNullable(n) => n.get_str(),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum NonNullableVariable<S: Scalar> {
    Array(Vec<Variable<S>>),
    Literal(LiteralVariable<S>),
}

impl<S: Scalar> NonNullableVariable<S> {
    fn get_str(self: &Self) -> Option<&str> {
        match self {
            NonNullableVariable::Literal(l) => l.get_str(),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum LiteralVariable<S: Scalar> {
    Object(Variables<S>),
    Scalar(S),
}

impl<S: Scalar> LiteralVariable<S> {
    fn get_str(self: &Self) -> Option<&str> {
        match self {
            LiteralVariable::Scalar(s) => s.get_str(),
            _ => None,
        }
    }
}

pub type Variables<S> = HashMap<String, Variable<S>>;

pub type ResolvedVariable = Box<dyn std::any::Any>;
pub type ResolvedVariables = HashMap<String, ResolvedVariable>;

fn resolve_type_spec<S: Scalar, R: Registry<S>>(
    registry: &R,
    spec: &shared::ast::InputTypeSpec,
    variable: &LiteralVariable<S>,
) -> Result<ResolvedVariable, String> {
    match (spec, variable) {
        (
            shared::ast::InputTypeSpec::Scalar(scalar_name),
            LiteralVariable::Scalar(scalar),
        ) => Ok(R::parse_scalar(registry, scalar_name, scalar)?),
        (shared::ast::InputTypeSpec::Scalar(scalar_name), other) => {
            Err(format!(
                "Received invalid type for scalar({}): {:?}",
                scalar_name, other
            ))
        }
        (
            shared::ast::InputTypeSpec::Enum(enum_type),
            LiteralVariable::Scalar(scalar),
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
            LiteralVariable::Object(object),
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
    var: &LiteralVariable<S>,
) -> Result<ResolvedVariable, String> {
    return resolve_type_spec(registry, &spec.r#type, var);
}

fn resolve_array<S: Scalar, R: Registry<S>>(
    registry: &R,
    array_type: &shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>,
    elements: &[Variable<S>],
) -> Result<Box<dyn std::any::Any>, String> {
    match &array_type.r#type {
        shared::ast::InputTypeSpec::Enum(e) => {
            R::parse_enum_array(registry, &e, &elements.iter().map(|e| {
                if let Variable::NonNullable(NonNullableVariable::Literal(
                    LiteralVariable::Scalar(scalar),
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
                if let Variable::NonNullable(NonNullableVariable::Literal(
                    LiteralVariable::Scalar(scalar),
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
                if let Variable::NonNullable(NonNullableVariable::Literal(
                    LiteralVariable::Object(object),
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

fn literal_to_literal_variable<S: Scalar>(
    literal: &shared::ast::Literal,
) -> Result<LiteralVariable<S>, String> {
    match literal {
        shared::ast::Literal::Int(i) => {
            Ok(LiteralVariable::Scalar(S::from_i64(*i)?))
        }
        shared::ast::Literal::Float(f) => {
            Ok(LiteralVariable::Scalar(S::from_f64(*f)?))
        }
        shared::ast::Literal::String(s) => {
            Ok(LiteralVariable::Scalar(S::from_string(s)?))
        }
        shared::ast::Literal::Boolean(b) => {
            Ok(LiteralVariable::Scalar(S::from_bool(*b)?))
        }
    }
}

fn resolve_operation_parameter<S: Scalar, R: Registry<S>>(
    registry: &R,
    param: &shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    variable: &Variable<S>,
) -> Result<Option<ResolvedVariable>, String> {
    let Variable::NonNullable(nonnullable_variable) = variable else {
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
                        &literal_to_literal_variable(
                            &spec
                                .default_value
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unwrap(),
                        )?,
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
            if let NonNullableVariable::Array(array) = nonnullable_variable {
                Ok(Some(resolve_array(registry, spec, array)?))
            } else {
                Err(format!(
                    "Expected array for parameter: {}, received: {:?}",
                    param.name, variable
                ))
            }
        }
        shared::ast::NonCallableFieldSpec::Literal(spec) => {
            if let NonNullableVariable::Literal(l) = nonnullable_variable {
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
    variables: &Variables<S>,
) -> Result<ResolvedVariables, String> {
    let mut vars = ResolvedVariables::new();
    for param in op_parameters.values() {
        if let Some(variable) = variables.get(&param.name[1..]) {
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
