use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{Value, Values};
use super::scalar::Scalar;

use super::ast::ResolverRoot;
use super::variables::ResolvedVariables;

pub type SubscriptionResolver<C, S, Stream> = Box<
    dyn Fn(
        &ResolverRoot<S>,
        &mut C,
        &ResolvedVariables,
    )
        -> std::pin::Pin<Box<dyn Future<Output = Result<Stream, String>>>>,
>;

pub type SubscriptionResolversMap<C, S, Stream> =
    HashMap<String, SubscriptionResolver<C, S, Stream>>;

pub struct SubscriptionStream<
    S: Scalar,
    Stream: futures_core::Stream<Item = Value<S>>,
> {
    field_name: String,
    stream: std::pin::Pin<Box<Stream>>,
}

impl<'f, S: Scalar, Stream: futures_core::Stream<Item = Value<S>>>
    futures_core::Stream for SubscriptionStream<S, Stream>
{
    type Item = Values<S>;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        <Stream as futures_core::Stream>::poll_next(
            self.stream.as_mut(),
            context,
        )
        .map(|o| o.map(|v| Values::from_iter([(self.field_name.clone(), v)])))
    }
}

pub async fn execute_subscription_operation<
    C,
    S: Scalar,
    Stream: futures_core::Stream<Item = Value<S>>,
>(
    context: &mut C,
    resolvers: &SubscriptionResolversMap<C, S, Stream>,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<SubscriptionStream<S, Stream>, String> {
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
        stream: Box::pin(value),
        field_name: selection.alias.clone(),
    });
}
