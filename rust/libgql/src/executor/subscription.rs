use std::collections::HashMap;

use crate::executor::{TypeRegistry, Values};
use crate::parsers::schema::client;

use super::scalar::Scalar;

use super::ast::ResolverRoot;
use super::variables::ResolvedVariables;

pub type SubscriptionResolverStream<S> =
    std::pin::Pin<Box<dyn futures_core::Stream<Item = ResolverRoot<S>>>>;
pub type SubscriptionResolverFuture<S> = std::pin::Pin<
    Box<dyn Future<Output = Result<SubscriptionResolverStream<S>, String>>>,
>;
pub type SubscriptionResolver<S, C> = Box<
    dyn Fn(
        &ResolverRoot<S>,
        &C,
        &ResolvedVariables,
    ) -> SubscriptionResolverFuture<S>,
>;

pub type SubscriptionResolversMap<S, C> =
    HashMap<String, SubscriptionResolver<S, C>>;

pub async fn execute_subscription_operation<
    'args,
    'variables,
    'operation,
    C,
    S: Scalar,
    T: TypeRegistry<S>,
>(
    context: &'args C,
    sync_resolvers: &'args super::sync::SyncResolversMap<S, C>,
    resolvers: &'args SubscriptionResolversMap<S, C>,
    type_registry: &'args T,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<
    std::pin::Pin<
        Box<
            impl futures::Stream<Item = Result<Values<S>, String>>
            + use<'args, C, S, T>,
        >,
    >,
    String,
> {
    let client::ast::FragmentSpec::Object(mut fragment_spec) =
        operation.fragment_spec
    else {
        return Err("Root operation must select an object".into());
    };

    let client::ast::ObjectSelection::FieldSelection(selection) =
        fragment_spec.selections.swap_remove(0)
    else {
        return Err("Unexpected selection for subscription".into());
    };

    let resolver = resolvers.get(&selection.name).ok_or_else(|| {
        format!("No subscription resolver for {}", selection.name)
    })?;

    let resolver_root: ResolverRoot<S> = Box::new(&());

    let mut stream = resolver(&resolver_root, context, &variables).await?;
    Ok(Box::pin(async_stream::stream! {
        while let Some(value) = futures::StreamExt::next(&mut stream.as_mut()).await {
            let mut callable_fields = Vec::new();
            if let Some(fragment_spec) = &selection.selection {
                callable_fields = super::sync::execute_fragment_on_value(
                    context,
                    sync_resolvers,
                    type_registry,
                    &value.create_introspection_value(),
                    &fragment_spec,
                    &variables,
                ).await?;
            };
            yield value.to_value(callable_fields).map(|v| {
                Values::from_iter([(selection.alias.clone(), v)])
            })
        }
    }))
}
