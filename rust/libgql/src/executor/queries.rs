use crate::parsers::schema::client;
use std::collections::HashMap;

use super::ast::{GraphqlError, ResolverFuture, Value, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type QueryResolver<S, C> = dyn for<'a> Fn(&'a C, &'a ResolvedVariables) -> ResolverFuture<'a, S>
    + Sync;
pub type QueryResolversMap<'a, S, C> =
    HashMap<&'a str, &'a QueryResolver<S, C>>;

fn execute_fragment<'a, C, S: Scalar>(
    context: &'a C,
    query_resolvers: &'a QueryResolversMap<'a, S, C>,
    object_field_resolvers: &'a super::object::ObjectFieldResolversMap<S, C>,
    spec: &'a client::ast::FragmentSpec,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<
    Box<dyn Future<Output = Result<Values<S>, Vec<GraphqlError>>> + 'a>,
> {
    Box::pin(async move {
        match spec {
            client::ast::FragmentSpec::Object(obj) => {
                execute_object_selection_set(
                    context,
                    query_resolvers,
                    object_field_resolvers,
                    &obj.selections,
                    variables,
                )
                .await
            }

            client::ast::FragmentSpec::Union(_) => {
                panic!("Unexpected union fragment spec on query object")
            }
            client::ast::FragmentSpec::Interface(_) => {
                panic!("Unexpected interface fragment spec on query object")
            }
        }
    })
}

async fn execute_field<C, S: Scalar>(
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Value<S>, Vec<GraphqlError>> {
    let value = {
        let resolver =
            query_resolvers.get(field.name.as_str()).ok_or_else(|| {
                vec![GraphqlError {
                    message: format!("No query resolver for {}", field.name)
                        .into(),
                    path: vec![field.alias.clone()],
                }]
            })?;
        resolver(context, variables).await.map_err(|e| {
            vec![GraphqlError {
                message: e,
                path: vec![field.alias.clone()],
            }]
        })?
    };
    super::object::execute_potential_selection_and_serialize(
        context,
        object_field_resolvers,
        value.to_value().map_err(|e| {
            vec![GraphqlError {
                message: e.into(),
                path: vec![field.alias.clone()],
            }]
        })?,
        field.selection.as_ref(),
        variables,
    )
    .await
}

async fn execute_field_selection<C, S: Scalar>(
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let value = execute_field(
        context,
        query_resolvers,
        object_field_resolvers,
        field,
        variables,
    )
    .await?;
    Ok(Values::from_iter([(field.alias.clone(), value)]))
}

async fn execute_object_selection<C, S: Scalar>(
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection,
) -> Result<Values<S>, Vec<GraphqlError>> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field("Query", field)
                .map(|t_field| Values::from_iter([t_field]))
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                context,
                query_resolvers,
                object_field_resolvers,
                field,
                variables,
            )
            .await
        }

        client::ast::ObjectSelection::SpreadSelection(spread) => {
            let fragment = spread.fragment.read().unwrap();
            execute_fragment(
                context,
                query_resolvers,
                object_field_resolvers,
                &fragment.spec,
                variables,
            )
            .await
        }
    }
}

async fn execute_object_selection_set<'a, C, S: Scalar>(
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Values<S>, Vec<GraphqlError>> {
            execute_object_selection(
                context,
                query_resolvers,
                object_field_resolvers,
                variables,
                selection,
            )
            .await
        },
    ))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, Vec<_>>>()
    .map(|a| a.into_iter().flatten().collect())
}

pub async fn execute_query_operation<C, S: Scalar>(
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err(vec![GraphqlError {
            message: "Root query operation must select an object"
                .to_string()
                .into(),
            path: vec![],
        }]);
    };

    execute_object_selection_set(
        context,
        query_resolvers,
        object_field_resolvers,
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
