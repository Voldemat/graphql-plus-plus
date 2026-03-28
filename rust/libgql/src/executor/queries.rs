use crate::parsers::schema::{self, client};
use std::collections::HashMap;

use super::ast::{GraphqlError, ResolverFuture, Value, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type QueryResolver<S, C> = dyn for<'a> Fn(&'a C, &'a ResolvedVariables) -> ResolverFuture<'a, S>
    + Sync
    + Send;
pub type QueryResolversMap<'a, S, C> =
    HashMap<&'a str, &'a QueryResolver<S, C>>;

async fn execute_fragment<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    spec: &client::ast::FragmentSpec<StringType>,
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    match spec {
        client::ast::FragmentSpec::Object(obj) => {
            execute_object_selection_set(
                client_registry,
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
}

async fn execute_field<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection<StringType>,
    variables: &ResolvedVariables,
) -> Result<Value<S>, Vec<GraphqlError>> {
    let value = {
        let resolver =
            query_resolvers.get(field.name.to_str()).ok_or_else(|| {
                vec![GraphqlError {
                    message: format!(
                        "No query resolver for {}",
                        field.name.to_str()
                    )
                    .into(),
                    path: vec![field.alias.to_str().to_string()],
                }]
            })?;
        resolver(context, variables).await.map_err(|e| {
            vec![GraphqlError {
                message: e,
                path: vec![field.alias.to_str().to_string()],
            }]
        })?
    };
    super::object::execute_potential_selection_and_serialize(
        client_registry,
        context,
        object_field_resolvers,
        value.to_value().map_err(|e| {
            vec![GraphqlError {
                message: e.into(),
                path: vec![field.alias.to_str().to_string()],
            }]
        })?,
        field.selection.as_ref(),
        variables,
    )
    .await
}

async fn execute_field_selection<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection<StringType>,
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let value = execute_field(
        client_registry,
        context,
        query_resolvers,
        object_field_resolvers,
        field,
        variables,
    )
    .await?;
    Ok(Values::from_iter([(
        field.alias.to_str().to_string(),
        value,
    )]))
}

async fn execute_object_selection<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection<StringType>,
) -> Result<Values<S>, Vec<GraphqlError>> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field("Query", field)
                .map(|t_field| Values::from_iter([t_field]))
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                client_registry,
                context,
                query_resolvers,
                object_field_resolvers,
                field,
                variables,
            )
            .await
        }

        client::ast::ObjectSelection::SpreadSelection(spread) => {
            let fragment =
                client_registry.fragments.get(&spread.fragment).unwrap();
            execute_fragment(
                client_registry,
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

async fn execute_object_selection_set<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    selections: &[client::ast::ObjectSelection<StringType>],
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Values<S>, Vec<GraphqlError>> {
            execute_object_selection(
                client_registry,
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

pub async fn execute_query_operation<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: client::type_registry::TypeRegistry<StringType>,
    context: &C,
    query_resolvers: &QueryResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    operation: client::ast::Operation<StringType>,
    variables: ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        operation.fragment_spec
    else {
        return Err(vec![GraphqlError {
            message: "Root query operation must select an object"
                .to_string()
                .into(),
            path: vec![],
        }]);
    };

    execute_object_selection_set(
        &client_registry,
        context,
        query_resolvers,
        object_field_resolvers,
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
