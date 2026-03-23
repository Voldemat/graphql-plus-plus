use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{GraphqlError, ResolverFuture, Value, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type MutationResolver<S, C> = dyn for<'a> Fn(&'a C, &'a ResolvedVariables) -> ResolverFuture<'a, S>
    + Sync;
pub type MutationResolversMap<'a, S, C> =
    HashMap<&'a str, &'a MutationResolver<S, C>>;

async fn execute_field<'a, C, S: Scalar>(
    context: &'a C,
    mutation_resolvers: &'a MutationResolversMap<'_, S, C>,
    object_field_resolvers: &'a super::object::ObjectFieldResolversMap<
        '_,
        S,
        C,
    >,
    field: &'a client::ast::FieldSelection,
    variables: &'a ResolvedVariables,
) -> Result<Value<S>, Vec<GraphqlError>> {
    let value = {
        let resolver =
            mutation_resolvers.get(field.name.as_str()).ok_or_else(|| {
                vec![GraphqlError {
                    message: format!("No mutation resolver for {}", field.name)
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

async fn execute_fragment<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    spec: &client::ast::FragmentSpec,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    match spec {
        client::ast::FragmentSpec::Object(obj) => {
            execute_object_selection_set(
                context,
                mutation_resolvers,
                object_field_resolvers,
                &obj.selections,
                variables,
            )
            .await
        }
        client::ast::FragmentSpec::Union(_) => {
            panic!("Unexpected union fragment spec on Mutation object")
        }
        client::ast::FragmentSpec::Interface(_) => {
            panic!("Unexpected interface fragment spec on Mutation object")
        }
    }
}

async fn execute_field_selection<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    let value = execute_field(
        context,
        mutation_resolvers,
        object_field_resolvers,
        field,
        variables,
    )
    .await?;
    Ok(vec![(field.alias.clone(), value)])
}

async fn execute_object_selection<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field("Mutation", field)
                .map(|t_field| vec![t_field])
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                context,
                mutation_resolvers,
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
                mutation_resolvers,
                object_field_resolvers,
                &fragment.spec,
                variables,
            )
            .await
        }
    }
}

async fn execute_object_selection_set<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
            execute_object_selection(
                context,
                mutation_resolvers,
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

pub async fn execute_mutation_operation<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err(vec![GraphqlError {
            message: "Root mutation operation must select an object"
                .to_string()
                .into(),
            path: vec![],
        }]);
    };

    execute_object_selection_set(
        context,
        mutation_resolvers,
        object_field_resolvers,
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
