pub mod ast;
pub mod hashmap_registry;
pub mod registry;
pub mod scalar;
pub mod subscription;
pub mod sync;
pub mod variables;
pub use ast::{LiteralValue, NonNullableValue, ResolverRoot, Value, Values};
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::Registry;
pub use scalar::Scalar;
pub use variables::{ResolvedVariables, resolve_operation_parameters};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client, type_registry::TypeRegistry},
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

pub enum OperationResult<S: Scalar> {
    Immediate(Values<S>),
    Stream(subscription::SubscriptionStream<S>),
}

async fn execute_operation<C, S: Scalar, R: Registry<S>>(
    context: &mut C,
    registry: &TypeRegistry,
    sync_resolvers: &sync::SyncResolversMap<C, S>,
    subscription_resolvers: &subscription::SubscriptionResolversMap<C, S>,
    parse_registry: &R,
    operation: &client::ast::Operation,
    variables: Values<S>,
) -> Result<OperationResult<S>, String> {
    let resolved_variables = resolve_operation_parameters(
        parse_registry,
        &operation.parameters,
        variables,
    )?;
    match operation.r#type {
        client::ast::OpType::Query => Ok(OperationResult::Immediate(
            sync::execute_sync_operation(
                context,
                sync_resolvers,
                &registry.get_query_object().unwrap().borrow(),
                operation,
                &resolved_variables,
            )
            .await?,
        )),
        client::ast::OpType::Mutation => Ok(OperationResult::Immediate(
            sync::execute_sync_operation(
                context,
                sync_resolvers,
                &registry.get_mutation_object().unwrap().borrow(),
                operation,
                &resolved_variables,
            )
            .await?,
        )),
        client::ast::OpType::Subscription => Ok(OperationResult::Stream(
            subscription::execute_subscription_operation(
                context,
                subscription_resolvers,
                operation,
                &resolved_variables,
            )
            .await?,
        )),
    }
}

pub async fn execute<C, S: Scalar, R: Registry<S>>(
    context: &mut C,
    registry: &TypeRegistry,
    sync_resolvers: &sync::SyncResolversMap<C, S>,
    subscription_resolvers: &subscription::SubscriptionResolversMap<C, S>,
    parse_registry: &R,
    client_query: &str,
    variables: Values<S>,
    operation: &Option<String>,
) -> Result<OperationResult<S>, Error> {
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
    let result = execute_operation(
        context,
        &mut local_registry,
        sync_resolvers,
        subscription_resolvers,
        parse_registry,
        &operation.borrow(),
        variables,
    )
    .await
    .unwrap();
    Ok(result)
}
