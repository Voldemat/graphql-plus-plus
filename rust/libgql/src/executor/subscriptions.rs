use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{GraphqlError, ResolverRoot, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type SubscriptionResolverStream<S> =
    std::pin::Pin<Box<dyn futures_core::Stream<Item = Box<ResolverRoot<S>>>>>;
pub type SubscriptionResolverFuture<'a, S> = std::pin::Pin<
    Box<
        dyn Future<
                Output = Result<
                    SubscriptionResolverStream<S>,
                    Vec<GraphqlError>,
                >,
            > + 'a,
    >,
>;
pub type SubscriptionResolver<S, C> =
    dyn for<'a> Fn(
            &'a C,
            &'a ResolvedVariables,
        ) -> SubscriptionResolverFuture<'a, S>
        + Sync;

pub type SubscriptionResolversMap<'a, S, C> =
    HashMap<&'a str, &'a SubscriptionResolver<S, C>>;

pub async fn execute_subscription_operation<
    'args,
    'variables,
    'operation,
    C,
    S: Scalar,
>(
    context: &'args C,
    resolvers: &'args SubscriptionResolversMap<'args, S, C>,
    object_field_resolvers: &'args super::object::ObjectFieldResolversMap<
        'args,
        S,
        C,
    >,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<
    std::pin::Pin<
        Box<
            impl futures::Stream<Item = Result<Values<S>, Vec<GraphqlError>>>
            + use<'args, C, S>,
        >,
    >,
    Vec<GraphqlError>,
> {
    let client::ast::FragmentSpec::Object(mut fragment_spec) =
        operation.fragment_spec
    else {
        return Err(vec![GraphqlError {
            message: "Root operation must select an object".to_string().into(),
            path: vec![],
        }]);
    };

    let client::ast::ObjectSelection::FieldSelection(selection) =
        fragment_spec.selections.swap_remove(0)
    else {
        return Err(vec![GraphqlError {
            message: "Unexpected selection for subscription".to_string().into(),
            path: vec![],
        }]);
    };

    let resolver = resolvers.get(selection.name.as_str()).ok_or_else(|| {
        vec![GraphqlError {
            message: format!("No subscription resolver for {}", selection.name)
                .into(),
            path: vec![selection.alias.clone()],
        }]
    })?;
    let vars = Box::new(variables);

    let mut stream = resolver(context, vars.as_ref()).await?;
    Ok(Box::pin(async_stream::stream! {
        let vars2 = vars;
        while let Some(value) = futures::StreamExt::next(&mut stream.as_mut()).await {

            let serialized_value = super::object::execute_potential_selection_and_serialize(
                context,
                object_field_resolvers,
                value.to_value().map_err(|e| vec![GraphqlError {
                    message: e.into(),
                    path: vec![selection.alias.clone()]
                }])?,
                selection.selection.as_ref(),
                &vars2.as_ref(),
            )
            .await?;
            yield
                Ok(Values::from_iter([(selection.alias.clone(), serialized_value)]))
        }
    }))
}
