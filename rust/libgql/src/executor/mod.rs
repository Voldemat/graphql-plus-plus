pub mod ast;
pub mod hashmap_registry;
pub mod registry;
pub mod scalar;
pub mod subscription;
pub mod sync;
pub mod variables;
use std::{cell::RefCell, rc::Rc};

pub use ast::{LiteralValue, NonNullableValue, Value, Values};
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::TypeRegistry;
pub use scalar::Scalar;
pub use variables::{ResolvedVariables, resolve_operation_parameters};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client, type_registry},
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

pub enum OperationResult<
    S: Scalar,
    TStream: futures::Stream<Item = Result<Values<S>, String>>,
> {
    Immediate(Values<S>),
    Stream(std::pin::Pin<Box<TStream>>),
}

async fn execute_operation<
    'args,
    'operation,
    C,
    S: Scalar,
    T: TypeRegistry<S>,
>(
    context: &'args C,
    registry: type_registry::TypeRegistry,
    sync_resolvers: &'args sync::SyncResolversMap<S, C>,
    subscription_resolvers: &'args subscription::SubscriptionResolversMap<S, C>,
    type_registry: &'args T,
    operation: client::ast::Operation,
    variables: Values<S>,
) -> Result<
    OperationResult<
        S,
        impl futures::Stream<Item = Result<Values<S>, String>> + use<'args, C, S, T>,
    >,
    String,
> {
    let resolved_variables = resolve_operation_parameters(
        type_registry,
        &operation.parameters,
        variables,
    )?;
    match operation.r#type {
        client::ast::OpType::Query => Ok(OperationResult::Immediate(
            sync::execute_sync_operation(
                context,
                sync_resolvers,
                type_registry,
                &registry.get_query_object().unwrap().borrow(),
                operation,
                resolved_variables,
            )
            .await?,
        )),
        client::ast::OpType::Mutation => Ok(OperationResult::Immediate(
            sync::execute_sync_operation(
                context,
                sync_resolvers,
                type_registry,
                &registry.get_mutation_object().unwrap().borrow(),
                operation,
                resolved_variables,
            )
            .await?,
        )),
        client::ast::OpType::Subscription => Ok(OperationResult::Stream(
            subscription::execute_subscription_operation(
                context,
                sync_resolvers,
                subscription_resolvers,
                type_registry,
                operation,
                resolved_variables,
            )
            .await?,
        )),
    }
}

pub async fn execute<'args, 'client_query, C, S: Scalar, T: TypeRegistry<S>>(
    context: &'args C,
    registry: &'args type_registry::TypeRegistry,
    sync_resolvers: &'args sync::SyncResolversMap<S, C>,
    subscription_resolvers: &'args subscription::SubscriptionResolversMap<S, C>,
    parse_registry: &'args T,
    client_query: &'client_query str,
    variables: Values<S>,
    operation: Option<String>,
) -> Result<
    OperationResult<
        S,
        impl futures::Stream<Item = Result<Values<S>, String>> + use<'args, C, S, T>,
    >,
    Error,
> {
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
    let mut client_schema =
        client::parse_client_schema(&mut local_registry, &file_nodes).unwrap();

    let operation_name = operation.map_or_else(
        || {
            if client_schema.operations.len() == 0 {
                return Err(Error::NoOperationsAreDefined);
            }
            if client_schema.operations.len() > 1 {
                return Err(Error::OperationNameIsNotDefined);
            }
            Ok(client_schema.operations.first().unwrap().0.to_string())
        },
        Result::Ok,
    )?;
    let operation_rc = client_schema
        .operations
        .swap_remove(&operation_name)
        .ok_or(Error::OperationIsNotDefined(operation_name))?;
    let operation =
        Rc::<RefCell<client::ast::Operation>>::try_unwrap(operation_rc)
            .unwrap()
            .into_inner();
    let result = execute_operation(
        context,
        local_registry,
        sync_resolvers,
        subscription_resolvers,
        parse_registry,
        operation,
        variables,
    )
    .await
    .unwrap();
    Ok(result)
}
