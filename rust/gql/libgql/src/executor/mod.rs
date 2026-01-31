pub mod hashmap_registry;
pub mod registry;
pub mod scalar;
pub mod variables;
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::Registry;
pub use scalar::Scalar;
use std::collections::HashMap;
pub use variables::{
    LiteralVariable, NonNullableVariable, ResolvedVariables, Variable,
    Variables, resolve_operation_parameters,
};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client, server, type_registry::TypeRegistry},
    },
};

#[derive(Debug, derive_more::From)]
pub enum Error {
    Lexer(Vec<lexer::Error>),
    FileParser(file::client::Error),
    OperationIsNotDefined(String),
    OperationNameIsNotDefined,
    NoOperationsAreDefined,
}

type ResolverRoot = Option<Box<dyn std::any::Any>>;

type Resolver<C> = dyn Fn(&ResolverRoot, &C, &ResolvedVariables);
type ResolversMap<C> = HashMap<(String, String), Resolver<C>>;

fn execute_sync_operation(
    registry: &TypeRegistry,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<(), String> {
    println!("{:?}", variables.get("limit").unwrap().downcast_ref::<i32>());
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
