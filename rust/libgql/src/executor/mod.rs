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

pub use ast::{LiteralValue, NonNullableValue, Value, Values};
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::ParseRegistry;
pub use scalar::Scalar;
pub use variables::{ResolvedVariables, resolve_operation_parameters};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{self, client, server},
    },
};

use self::ast::GraphqlError;

#[derive(Debug, derive_more::From)]
pub enum Error<'buffer> {
    Lexer(Vec<lexer::types::Error>),
    FileParser(file::client::Error<'buffer>),
    OperationIsNotDefined(&'buffer str),
    OperationNameIsNotDefined,
    NoOperationsAreDefined,
    ExecutionErrors(Vec<GraphqlError>),
}

pub enum OperationResult<'a, S: Scalar> {
    Immediate(Values<S>),
    Stream(subscriptions::SubscriptionOperationStream<'a, S>),
}

pub struct Resolvers<'a, S: Scalar, C: Send + Sync> {
    pub queries: queries::QueryResolversMap<'a, S, C>,
    pub mutations: mutations::MutationResolversMap<'a, S, C>,
    pub subscriptions: subscriptions::SubscriptionResolversMap<'a, S, C>,
    pub object_fields: object::ObjectFieldResolversMap<'a, S, C>,
}

async fn execute_operation<
    'buffer,
    'args,
    'operation,
    C: Send + Sync,
    S: Scalar,
    T: ParseRegistry<S>,
    StringType: schema::shared::ast::AsStr<'buffer> + 'args,
>(
    client_registry: client::type_registry::TypeRegistry<StringType>,
    context: &'args C,
    resolvers: &'args Resolvers<'args, S, C>,
    parse_registry: &'args T,
    operation: client::ast::Operation<StringType>,
    variables: Values<S>,
) -> Result<OperationResult<'args, S>, Vec<GraphqlError>> {
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
                client_registry,
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
                client_registry,
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
                client_registry,
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

pub async fn execute<
    'args,
    'buffer,
    C: Send + Sync,
    S: Scalar,
    T: ParseRegistry<S>,
    StringType: schema::shared::ast::AsStr<'buffer> + 'args,
    ServerTypeRegistry: server::type_registry::TypeRegistry<'buffer, StringType>,
>(
    context: &'args C,
    server_registry: &'args ServerTypeRegistry,
    resolvers: &'args Resolvers<'args, S, C>,
    parse_registry: &'args T,
    client_query: &'buffer str,
    variables: Values<S>,
    operation: Option<&'buffer str>,
) -> Result<OperationResult<'args, S>, Error<'buffer>> {
    let tokens = lexer::utils::parse_buffer_into_tokens(client_query)?;
    let source_file = std::sync::Arc::new(file::shared::ast::SourceFile {
        filepath: "<request>".into(),
        buffer: client_query,
    });
    let file_nodes = file::client::Parser::new(
        file::tokens_sources::VecTokensSource::new(tokens, source_file.clone()),
    )
    .parse_ast_nodes()?;
    let mut client_registry =
        client::type_registry::TypeRegistry::<StringType>::new();
    client::parse_client_schema(
        server_registry,
        &mut client_registry,
        &file_nodes,
    )
    .unwrap();

    let operation_index = match operation {
        None => {
            if client_registry.operations.len() == 0 {
                return Err(Error::NoOperationsAreDefined);
            }
            if client_registry.operations.len() > 1 {
                return Err(Error::OperationNameIsNotDefined);
            }
            Ok::<usize, Error<'buffer>>(0)
        }
        Some(name) => client_registry
            .operations
            .get_index_of(name)
            .ok_or(Error::OperationIsNotDefined(name)),
    }?;
    let operation = client_registry
        .operations
        .swap_remove_index(operation_index)
        .unwrap()
        .1;
    execute_operation(
        client_registry,
        context,
        resolvers,
        parse_registry,
        operation,
        variables,
    )
    .await
    .map_err(|errors| Error::ExecutionErrors(errors))
}
