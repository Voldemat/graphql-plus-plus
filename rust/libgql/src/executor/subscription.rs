use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{Value, Values};
use super::scalar::Scalar;

use super::ast::ResolverRoot;
use super::variables::ResolvedVariables;

pub type SubscriptionResolverStream<S> =
    std::pin::Pin<Box<dyn futures_core::Stream<Item = Value<S>>>>;
pub type SubscriptionResolverFuture<S> = std::pin::Pin<
    Box<dyn Future<Output = Result<SubscriptionResolverStream<S>, String>>>,
>;
pub type SubscriptionResolver<C, S> = Box<
    dyn Fn(
        &ResolverRoot<S>,
        &mut C,
        &ResolvedVariables,
    ) -> SubscriptionResolverFuture<S>,
>;

pub type SubscriptionResolversMap<C, S> =
    HashMap<String, SubscriptionResolver<C, S>>;

pub struct SubscriptionStream<S: Scalar> {
    field_name: String,
    stream: SubscriptionResolverStream<S>,
}

impl<S: Scalar> futures_core::Stream for SubscriptionStream<S> {
    type Item = Values<S>;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(context).map(|o| {
            o.map(|v| Values::from_iter([(self.field_name.clone(), v)]))
        })
    }
}

pub async fn execute_subscription_operation<C, S: Scalar>(
    context: &mut C,
    resolvers: &SubscriptionResolversMap<C, S>,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<SubscriptionStream<S>, String> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err("Root operation must select an object".into());
    };

    let client::ast::ObjectSelection::FieldSelection(selection) =
        &fragment_spec.selections[0]
    else {
        return Err("Unexpected selection for subscription".into());
    };

    let resolver = resolvers.get(&selection.name).ok_or_else(|| {
        format!("No subscription resolver for {}", selection.name)
    })?;

    let parent = Values::new();
    let value = resolver(&parent, context, variables).await?;
    return Ok(SubscriptionStream {
        stream: value,
        field_name: selection.alias.clone(),
    });
}
