use std::collections::HashMap;

use crate::parsers::schema::{self, client};

use super::ast::{GraphqlError, ResolverRoot, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type SubscriptionResolverStream<S> = std::pin::Pin<
    Box<dyn futures_core::Stream<Item = Box<ResolverRoot<S>>> + Send + Sync>,
>;
pub type SubscriptionResolverFuture<'a, S> = std::pin::Pin<
    Box<
        dyn Future<
                Output = Result<
                    SubscriptionResolverStream<S>,
                    Vec<GraphqlError>,
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

pub async fn execute_subscription_operation<
    'args,
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer> + 'args,
>(
    client_registry: client::type_registry::TypeRegistry<StringType>,
    context: &'args C,
    resolvers: &'args SubscriptionResolversMap<'args, S, C>,
    object_field_resolvers: &'args super::object::ObjectFieldResolversMap<
        'args,
        S,
        C,
    >,
    operation: client::ast::Operation<StringType>,
    variables: ResolvedVariables,
) -> Result<SubscriptionOperationStream<'args, S>, Vec<GraphqlError>> {
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

    let resolver = resolvers.get(selection.name.to_str()).ok_or_else(|| {
        vec![GraphqlError {
            message: format!(
                "No subscription resolver for {}",
                selection.name.to_str()
            )
            .into(),
            path: vec![selection.alias.to_str().to_string()],
        }]
    })?;
    let vars = Box::new(variables);

    let mut stream = resolver(context, vars.as_ref()).await?;
    Ok(Box::pin(async_stream::stream! {
        let vars2 = vars;
        while let Some(value) = futures::StreamExt::next(&mut stream.as_mut()).await {

            let serialized_value = super::object::execute_potential_selection_and_serialize(
                &client_registry,
                context,
                object_field_resolvers,
                value.to_value().map_err(|e| vec![GraphqlError {
                    message: e.into(),
                    path: vec![selection.alias.to_str().to_string()]
                }])?,
                selection.selection.as_ref(),
                &vars2.as_ref(),
            )
            .await?;
            yield
                Ok(Values::from_iter([(selection.alias.to_str().to_string(), serialized_value)]))
        }
    }))
}
