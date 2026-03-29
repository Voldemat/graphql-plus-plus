use std::collections::HashMap;

use crate::parsers::schema::client;

use super::OwningTypeRegistry;
use super::ast::{GraphqlError, ResolverError, ResolverRoot, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type SubscriptionResolverStream<'a, S> = std::pin::Pin<
    Box<
        dyn futures_core::Stream<Item = Box<ResolverRoot<S>>>
            + Send
            + Sync
            + 'a,
    >,
>;
pub type SubscriptionResolverFuture<'a, S> = std::pin::Pin<
    Box<
        dyn Future<
                Output = Result<
                    SubscriptionResolverStream<'a, S>,
                    ResolverError,
                >,
            > + Send
            + 'a,
    >,
>;
pub type SubscriptionResolver<S, C> =
    dyn for<'a> Fn(
            &'a C,
            &'a ResolvedVariables,
        ) -> SubscriptionResolverFuture<'a, S>
        + Sync
        + Send;

pub type SubscriptionResolversMap<'a, S, C> =
    HashMap<&'a str, &'a SubscriptionResolver<S, C>>;

pub type SubscriptionOperationStream<'a, S> = std::pin::Pin<
    Box<
        dyn futures::Stream<Item = Result<Values<S>, Vec<GraphqlError>>>
            + Send
            + 'a,
    >,
>;

struct SendPtr(*const ());

unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

pub async fn execute_subscription_operation<
    'args,
    C: Send + Sync,
    S: Scalar,
>(
    client_registry: std::pin::Pin<Box<OwningTypeRegistry<'args>>>,
    context: &'args C,
    resolvers: &'args SubscriptionResolversMap<'args, S, C>,
    object_field_resolvers: &'args super::object::ObjectFieldResolversMap<
        'args,
        S,
        C,
    >,
    variables: ResolvedVariables,
) -> Result<SubscriptionOperationStream<'args, S>, Vec<GraphqlError>> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &client_registry.borrow_operation().fragment_spec
    else {
        return Err(vec![GraphqlError {
            message: "Root operation must select an object".to_string().into(),
            path: vec![],
        }]);
    };

    let client::ast::ObjectSelection::FieldSelection(selection) =
        fragment_spec.selections.get(0).unwrap()
    else {
        return Err(vec![GraphqlError {
            message: "Unexpected selection for subscription".to_string().into(),
            path: vec![],
        }]);
    };

    let resolver = resolvers.get(selection.name).ok_or_else(|| {
        vec![GraphqlError {
            message: format!("No subscription resolver for {}", selection.name)
                .into(),
            path: vec![selection.alias.to_string()],
        }]
    })?;
    let vars = Box::pin(variables);

    let selection_pointer = SendPtr(
        selection as *const client::ast::FieldSelection<&str> as *const (),
    );
    let var_pointer: *const ResolvedVariables =
        vars.as_ref().get_ref() as *const _;
    let var_pointer_ref: &ResolvedVariables = unsafe { &*var_pointer };
    let mut stream = resolver(context, var_pointer_ref).await.map_err(|e| {
        vec![GraphqlError {
            message: e,
            path: vec![selection.alias.to_string()],
        }]
    })?;
    Ok(Box::pin(async_stream::stream! {
        let vars2 = vars;
        let p = selection_pointer;
        let a = p.0;
        let selection = unsafe {&*(a as *const client::ast::FieldSelection<&str>)};
        while let Some(value) = futures::StreamExt::next(&mut stream.as_mut()).await {

            let serialized_value = super::object::execute_potential_selection_and_serialize(
                client_registry.borrow_registry(),
                context,
                object_field_resolvers,
                value.to_value().map_err(|e| vec![GraphqlError {
                    message: e.into(),
                    path: vec![selection.alias.to_string()]
                }])?,
                selection.selection.as_ref(),
                &vars2.as_ref(),
            )
            .await?;
            yield
                Ok(Values::from_iter([(selection.alias.to_string(), serialized_value)]))
        }
    }))
}
