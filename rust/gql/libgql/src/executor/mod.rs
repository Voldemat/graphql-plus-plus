use std::collections::HashMap;

use indexmap::IndexMap;

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client, server, shared, type_registry::TypeRegistry},
    },
};

pub trait Scalar: Sized + std::fmt::Debug {
    fn from_string(s: &str) -> Result<Self, String>;
    fn from_u64(n: u64) -> Result<Self, String>;
    fn from_i64(n: i64) -> Result<Self, String>;
    fn from_f64(n: f64) -> Result<Self, String>;
    fn from_bool(b: bool) -> Result<Self, String>;
    fn get_enum_value(self: &Self) -> Option<&str>;
}

#[derive(Debug)]
pub enum Variable<S: Scalar> {
    Null,
    NonNullable(NonNullableVariable<S>),
}

#[derive(Debug)]
pub enum NonNullableVariable<S: Scalar> {
    Array(Vec<Variable<S>>),
    Literal(LiteralVariable<S>),
}

#[derive(Debug)]
pub enum LiteralVariable<S: Scalar> {
    Object(Variables<S>),
    Scalar(S),
}

pub type Variables<S> = HashMap<String, Variable<S>>;

pub type ResolvedVariable = Box<dyn std::any::Any>;
pub type ResolvedVariables = HashMap<String, ResolvedVariable>;

#[derive(Debug, derive_more::From)]
pub enum Error {
    Lexer(Vec<lexer::Error>),
    FileParser(file::client::Error),
    OperationIsNotDefined(String),
    OperationNameIsNotDefined,
    NoOperationsAreDefined,
}

type ResolverRoot = ();

type Resolver<C> = dyn Fn(&ResolverRoot, &C, &ResolvedVariables);
type ResolversMap<C> = HashMap<(String, String), Resolver<C>>;

pub trait Registry<S: Scalar> {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: &S,
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_enum(
        self: &Self,
        enum_type: &shared::ast::Enum,
        value: &str,
    ) -> Result<Box<dyn std::any::Any>, String>;
    fn parse_input(
        self: &Self,
        input_type: &shared::ast::InputType,
        value: &Variables<S>,
    ) -> Result<Box<dyn std::any::Any>, String>;
}

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
            if let Some(enum_value) = scalar.get_enum_value() {
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
    let mut v = Vec::<Box<dyn std::any::Any>>::new();
    match &array_type.r#type {
        shared::ast::InputTypeSpec::Enum(e) => {
            for element in elements {
                if let Variable::NonNullable(NonNullableVariable::Literal(
                    LiteralVariable::Scalar(scalar),
                )) = element
                    && let Some(s) = scalar.get_enum_value()
                {
                    v.push(R::parse_enum(registry, &e, s)?);
                } else {
                    return Err(format!(
                        "Expected string scalar for enum value, received: {:?}",
                        element
                    ));
                };
            }
        }
        shared::ast::InputTypeSpec::Scalar(scalar_name) => {
            for element in elements {
                if let Variable::NonNullable(NonNullableVariable::Literal(
                    LiteralVariable::Scalar(scalar),
                )) = element
                {
                    v.push(R::parse_scalar(registry, &scalar_name, scalar)?);
                } else {
                    return Err(format!(
                        "Expected scalar, received: {:?}",
                        element
                    ));
                };
            }
        }
        shared::ast::InputTypeSpec::InputType(input_type) => {
            for element in elements {
                if let Variable::NonNullable(NonNullableVariable::Literal(
                    LiteralVariable::Object(object),
                )) = element
                {
                    v.push(R::parse_input(
                        registry,
                        &input_type.borrow(),
                        object,
                    )?);
                } else {
                    return Err(format!(
                        "Expected object, received: {:?}",
                        element
                    ));
                };
            }
        }
    }
    Ok(Box::new(v))
}

fn resolve_operation_parameter<S: Scalar, R: Registry<S>>(
    registry: &R,
    param: &shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    variable: &Variable<S>,
) -> Result<Option<ResolvedVariable>, String> {
    let Variable::NonNullable(nonnullable_variable) = variable else {
        if param.nullable {
            return Ok(None);
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
                return Ok(Some(resolve_array(registry, spec, array)?));
            } else {
                Err(format!(
                    "Expected array for parameter: {}, received: {:?}",
                    param.name, variable
                ))
            }
        }
        shared::ast::NonCallableFieldSpec::Literal(spec) => {
            if let NonNullableVariable::Literal(l) = nonnullable_variable {
                return Ok(Some(resolve_literal(registry, spec, l)?));
            } else {
                Err(format!(
                    "Expected literal variable for parameter: {}, received: {:?}",
                    param.name, variable
                ))
            }
        }
    }
}

fn resolve_operation_parameters<S: Scalar, R: Registry<S>>(
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
        } else {
            return Err(format!(
                "Required operation parameter {} is missing",
                param.name
            ));
        }
    }
    Ok(vars)
}

fn execute_sync_operation(
    registry: &TypeRegistry,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<(), String> {
    Ok(())
}

fn execute_subscription_operation(
    registry: &TypeRegistry,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<(), String> {
    Ok(())
}

fn execute_operation<S: Scalar, R: Registry<S>>(
    registry: &TypeRegistry,
    parse_registry: &R,
    operation: &client::ast::Operation,
    variables: &Variables<S>,
) -> Result<(), String> {
    let resolved_variables = resolve_operation_parameters(
        parse_registry,
        &operation.parameters,
        variables,
    )?;
    match operation.r#type {
        client::ast::OpType::Query => execute_sync_operation(
            registry,
            &registry.get_query_object().unwrap().borrow(),
            operation,
            &resolved_variables,
        ),
        client::ast::OpType::Mutation => execute_sync_operation(
            registry,
            &registry.get_mutation_object().unwrap().borrow(),
            operation,
            &resolved_variables,
        ),
        client::ast::OpType::Subscription => execute_subscription_operation(
            registry,
            &registry.get_subscription_object().unwrap().borrow(),
            operation,
            &resolved_variables,
        ),
    }
}

pub fn execute<S: Scalar, R: Registry<S>>(
    registry: &TypeRegistry,
    parse_registry: &R,
    client_query: &str,
    variables: &Variables<S>,
    operation: &Option<String>,
) -> Result<(), Error> {
    let tokens = lexer::utils::parse_buffer_into_tokens(client_query)?;
    let source_file = std::rc::Rc::new(file::shared::ast::SourceFile {
        filepath: "<request>".into(),
        buffer: client_query.into(),
    });
    let file_nodes = file::client::Parser::new(
        file::tokens_sources::VecTokensSource::new(tokens, source_file.clone()),
    )
    .parse_ast_nodes()?;
    let mut local_registry = registry.clone();
    let client_schema =
        client::parse_client_schema(&mut local_registry, &file_nodes).unwrap();

    let operation_name = operation.as_ref().map_or_else(
        || {
            if client_schema.operations.len() == 0 {
                return Err(Error::NoOperationsAreDefined);
            }
            if client_schema.operations.len() > 1 {
                return Err(Error::OperationNameIsNotDefined);
            }
            Ok(client_schema.operations.first().unwrap().0)
        },
        Result::Ok,
    )?;
    let operation = client_schema
        .operations
        .get(operation_name)
        .ok_or(Error::OperationIsNotDefined(operation_name.clone()))?;
    execute_operation(
        &mut local_registry,
        parse_registry,
        &operation.borrow(),
        variables,
    )
    .unwrap();
    Ok(())
}
