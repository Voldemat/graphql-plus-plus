use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{ResolverRoot, Values};
use super::registry::TypeRegistry;
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type SubscriptionResolverStream<S> =
    std::pin::Pin<Box<dyn futures_core::Stream<Item = Box<ResolverRoot<S>>>>>;
pub type SubscriptionResolverFuture<'a, S> = std::pin::Pin<
    Box<
        dyn Future<Output = Result<SubscriptionResolverStream<S>, String>> + 'a,
    >,
>;
pub type SubscriptionResolver<S, C> = Box<
    dyn for<'a> Fn(
        &'a C,
        &'a ResolvedVariables,
    ) -> SubscriptionResolverFuture<'a, S>,
>;

pub type SubscriptionResolversMap<S, C> =
    HashMap<&'static str, SubscriptionResolver<S, C>>;

pub async fn execute_subscription_operation<
    'args,
    'variables,
    'operation,
    C,
    S: Scalar,
    T: TypeRegistry<S>,
>(
    context: &'args C,
    resolvers: &'args SubscriptionResolversMap<S, C>,
    query_resolvers: &'args super::queries::QueryResolversMap<S, C>,
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

    let resolver = resolvers.get(selection.name.as_str()).ok_or_else(|| {
        format!("No subscription resolver for {}", selection.name)
    })?;
    let vars = Box::new(variables);

    let mut stream = resolver(context, vars.as_ref()).await?;
    Ok(Box::pin(async_stream::stream! {
        let vars2 = vars;
        while let Some(value) = futures::StreamExt::next(&mut stream.as_mut()).await {

            let serialized_value = super::queries::execute_potential_selection_and_serialize(
                context,
                query_resolvers,
                type_registry,
                value.to_value()?,
                selection.selection.as_ref(),
                &vars2.as_ref(),
            )
            .await?;
            yield
                Ok(Values::from_iter([(selection.alias.clone(), serialized_value)]))
        }
    }))
}
