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

#[derive(Debug)]
pub enum Error {
    Lexer(Vec<lexer::types::Error>),
    FileParser(String),
    OperationIsNotDefined(String),
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
    'args,
    C: Send + Sync,
    S: Scalar,
    T: ParseRegistry<S>,
>(
    client_registry: std::pin::Pin<Box<OwningTypeRegistry<'args>>>,
    context: &'args C,
    resolvers: &'args Resolvers<'args, S, C>,
    parse_registry: &'args T,
    variables: Values<S>,
) -> Result<OperationResult<'args, S>, Vec<GraphqlError>> {
    let resolved_variables = resolve_operation_parameters(
        parse_registry,
        &client_registry.borrow_operation().parameters,
        variables,
    )
    .map_err(|e| {
        vec![GraphqlError {
            message: e.into(),
            path: vec![],
        }]
    })?;
    match client_registry.borrow_operation().r#type {
        client::ast::OpType::Query => Ok(OperationResult::Immediate(
            queries::execute_query_operation(
                client_registry,
                context,
                &resolvers.queries,
                &resolvers.object_fields,
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
                resolved_variables,
            )
            .await?,
        )),
    }
}

pub struct OwningTypeRegistry<'server_buffer> {
    query: String,
    registry: std::mem::MaybeUninit<
        client::type_registry::TypeRegistry<&'server_buffer str>,
    >,
    operation: Option<client::ast::Operation<&'server_buffer str>>,
    _pin: std::marker::PhantomPinned,
}

impl<'server_buffer> OwningTypeRegistry<'server_buffer> {
    pub fn new(
        query: String,
        registry_builder: impl FnOnce(
            &'server_buffer String,
        ) -> Result<
            client::type_registry::TypeRegistry<&'server_buffer str>,
            Error,
        >,
        operation_builder: impl FnOnce(
            &mut client::type_registry::TypeRegistry<&'server_buffer str>,
        ) -> Result<
            client::ast::Operation<&'server_buffer str>,
            Error,
        >,
    ) -> Result<std::pin::Pin<Box<OwningTypeRegistry<'server_buffer>>>, Error>
    {
        let mut registry = Box::pin(OwningTypeRegistry::<'server_buffer> {
            query,
            registry: std::mem::MaybeUninit::uninit(),
            operation: None,
            _pin: Default::default(),
        });
        let ptr: *const String = &registry.query as *const String;
        unsafe {
            let r = std::pin::Pin::get_unchecked_mut(registry.as_mut());
            r.registry.write(registry_builder(&*ptr)?);
            r.operation =
                Some(operation_builder(r.registry.assume_init_mut())?);
        }
        Ok(registry)
    }

    pub fn borrow_registry<'this>(
        self: &'this Self,
    ) -> &'this client::type_registry::TypeRegistry<&'this str>
    where
        'this: 'server_buffer,
    {
        unsafe { self.registry.assume_init_ref() }
    }

    pub fn borrow_operation<'this>(
        self: &'this Self,
    ) -> &'this client::ast::Operation<&'this str>
    where
        'this: 'server_buffer,
    {
        self.operation.as_ref().unwrap()
    }

    pub fn take_operation<'this>(
        self: &'this mut Self,
    ) -> client::ast::Operation<&'this str>
    where
        'this: 'server_buffer,
    {
        self.operation.take().unwrap()
    }
}

pub async fn execute<
    'args,
    C: Send + Sync,
    S: Scalar,
    T: ParseRegistry<S>,
    ServerStringType: schema::shared::ast::AsStr<'args> + 'args,
    ServerTypeRegistry: server::type_registry::TypeRegistry<'args, ServerStringType>,
>(
    context: &'args C,
    server_registry: &'args ServerTypeRegistry,
    resolvers: &'args Resolvers<'args, S, C>,
    parse_registry: &'args T,
    client_query: String,
    variables: Values<S>,
    operation_name: Option<String>,
) -> Result<OperationResult<'args, S>, Error> {
    let client_registry = OwningTypeRegistry::<'args>::new(
        client_query,
        |query| {
            let mut c_registry = client::type_registry::TypeRegistry::new();
            let tokens = lexer::utils::parse_buffer_into_tokens(query)
                .map_err(|errors| Error::Lexer(errors))?;
            let source_file =
                std::sync::Arc::new(file::shared::ast::SourceFile {
                    filepath: "<request>".into(),
                    buffer: &query,
                });
            let file_nodes = file::client::Parser::new(
                file::tokens_sources::VecTokensSource::new(
                    tokens,
                    source_file.clone(),
                ),
            )
            .parse_ast_nodes()
            .map_err(|error| Error::FileParser(format!("{:?}", error)))?;
            client::parse_client_schema(
                server_registry,
                &mut c_registry,
                &file_nodes,
            )
            .unwrap();
            Ok(c_registry)
        },
        |c_registry| {
            let operation_index = match operation_name {
                None => {
                    if c_registry.operations.len() == 0 {
                        return Err(Error::NoOperationsAreDefined);
                    }
                    if c_registry.operations.len() > 1 {
                        return Err(Error::OperationNameIsNotDefined);
                    }
                    Ok::<usize, Error>(0)
                }
                Some(name) => c_registry
                    .operations
                    .get_index_of(name.as_str())
                    .ok_or(Error::OperationIsNotDefined(name)),
            }?;
            Ok(c_registry
                .operations
                .swap_remove_index(operation_index)
                .unwrap()
                .1)
        },
    )?;

    execute_operation(
        client_registry,
        context,
        resolvers,
        parse_registry,
        variables,
    )
    .await
    .map_err(|errors| Error::ExecutionErrors(errors))
}
