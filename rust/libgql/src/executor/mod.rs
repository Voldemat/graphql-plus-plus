pub mod ast;
pub mod hashmap_registry;
pub mod mutations;
pub mod object;
pub mod queries;
pub mod registry;
pub mod scalar;
pub mod shared;
pub mod subscriptions;
pub mod variables;
use std::sync::{Arc, RwLock};

pub use ast::{LiteralValue, NonNullableValue, Value, Values};
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::ParseRegistry;
pub use scalar::Scalar;
pub use variables::{ResolvedVariables, resolve_operation_parameters};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client, type_registry},
    },
};

use self::ast::GraphqlError;

#[derive(Debug, derive_more::From)]
pub enum Error<'buffer> {
    Lexer(Vec<lexer::types::Error>),
    FileParser(file::client::Error<'buffer>),
    OperationIsNotDefined(String),
    OperationNameIsNotDefined,
    NoOperationsAreDefined,
    ExecutionErrors(Vec<GraphqlError>),
}

pub enum OperationResult<
    S: Scalar,
    TStream: futures::Stream<Item = Result<Values<S>, Vec<GraphqlError>>>,
> {
    Immediate(Values<S>),
    Stream(std::pin::Pin<Box<TStream>>),
}

pub struct Resolvers<'a, S: Scalar, C> {
    pub queries: queries::QueryResolversMap<'a, S, C>,
    pub mutations: mutations::MutationResolversMap<'a, S, C>,
    pub subscriptions: subscriptions::SubscriptionResolversMap<'a, S, C>,
    pub object_fields: object::ObjectFieldResolversMap<'a, S, C>,
}

async fn execute_operation<
    'args,
    'operation,
    C,
    S: Scalar,
    T: ParseRegistry<S>,
>(
    context: &'args C,
    resolvers: &'args Resolvers<'args, S, C>,
    parse_registry: &'args T,
    operation: client::ast::Operation,
    variables: Values<S>,
) -> Result<
    OperationResult<
        S,
        impl futures::Stream<Item = Result<Values<S>, Vec<GraphqlError>>>
        + use<'args, C, S, T>,
    >,
    Vec<GraphqlError>,
> {
    let resolved_variables = resolve_operation_parameters(
        parse_registry,
        &operation.parameters,
        variables,
    )
    .map_err(|e| {
        vec![GraphqlError {
            message: e.into(),
            path: vec![],
        }]
    })?;
    match operation.r#type {
        client::ast::OpType::Query => Ok(OperationResult::Immediate(
            queries::execute_query_operation(
                context,
                &resolvers.queries,
                &resolvers.object_fields,
                operation,
                resolved_variables,
            )
            .await?,
        )),
        client::ast::OpType::Mutation => Ok(OperationResult::Immediate(
            mutations::execute_mutation_operation(
                context,
                &resolvers.mutations,
                &resolvers.object_fields,
                operation,
                resolved_variables,
            )
            .await?,
        )),
        client::ast::OpType::Subscription => Ok(OperationResult::Stream(
            subscriptions::execute_subscription_operation(
                context,
                &resolvers.subscriptions,
                &resolvers.object_fields,
                operation,
                resolved_variables,
            )
            .await?,
        )),
    }
}

pub async fn execute<'args, 'buffer, C, S: Scalar, T: ParseRegistry<S>>(
    context: &'args C,
    registry: &'args type_registry::TypeRegistry,
    resolvers: &'args Resolvers<'args, S, C>,
    parse_registry: &'args T,
    client_query: &'buffer str,
    variables: Values<S>,
    operation: Option<String>,
) -> Result<
    OperationResult<
        S,
        impl futures::Stream<Item = Result<Values<S>, Vec<GraphqlError>>>
        + use<'args, C, S, T>,
    >,
    Error<'buffer>,
> {
    let tokens = lexer::utils::parse_buffer_into_tokens(client_query)?;
    let source_file = std::sync::Arc::new(file::shared::ast::SourceFile {
        filepath: "<request>".into(),
        buffer: client_query,
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
    local_registry.operations.remove(&operation_name);
    let operation_rc = client_schema
        .operations
        .swap_remove(&operation_name)
        .ok_or(Error::OperationIsNotDefined(operation_name))?;
    let operation =
        Arc::<RwLock<client::ast::Operation>>::try_unwrap(operation_rc)
            .unwrap()
            .into_inner()
            .unwrap();
    execute_operation(context, resolvers, parse_registry, operation, variables)
        .await
        .map_err(|errors| Error::ExecutionErrors(errors))
}
