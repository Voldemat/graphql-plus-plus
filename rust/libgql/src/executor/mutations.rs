use std::collections::HashMap;

use crate::parsers::schema::{self, client};

use super::OwningTypeRegistry;
use super::ast::{GraphqlError, ResolverFuture, Value, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type MutationResolver<S, C> = dyn for<'a> Fn(&'a C, &'a ResolvedVariables) -> ResolverFuture<'a, S>
    + Sync
    + Send;
pub type MutationResolversMap<'a, S, C> =
    HashMap<&'a str, &'a MutationResolver<S, C>>;

async fn execute_field<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection<StringType>,
    variables: &ResolvedVariables,
) -> Result<Value<S>, Vec<GraphqlError>> {
    let value = {
        let resolver =
            mutation_resolvers.get(field.name.to_str()).ok_or_else(|| {
                vec![GraphqlError {
                    message: format!(
                        "No mutation resolver for {}",
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

async fn execute_fragment<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    spec: &client::ast::FragmentSpec<StringType>,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    match spec {
        client::ast::FragmentSpec::Object(obj) => {
            execute_object_selection_set(
                client_registry,
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

async fn execute_field_selection<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    field: &client::ast::FieldSelection<StringType>,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    Ok(vec![(
        field.alias.to_str().to_string(),
        execute_field(
            client_registry,
            context,
            mutation_resolvers,
            object_field_resolvers,
            field,
            variables,
        )
        .await?,
    )])
}

async fn execute_object_selection<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection<StringType>,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field("Mutation", field)
                .map(|t_field| vec![t_field])
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                client_registry,
                context,
                mutation_resolvers,
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
                mutation_resolvers,
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
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    selections: &[client::ast::ObjectSelection<StringType>],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Vec<(String, Value<S>)>, Vec<GraphqlError>> {
            execute_object_selection(
                &client_registry,
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

pub async fn execute_mutation_operation<'buffer, C: Send + Sync, S: Scalar>(
    client_registry: std::pin::Pin<Box<OwningTypeRegistry<'buffer>>>,
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
    variables: ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &client_registry.borrow_operation().fragment_spec
    else {
        return Err(vec![GraphqlError {
            message: "Root mutation operation must select an object"
                .to_string()
                .into(),
            path: vec![],
        }]);
    };

    execute_object_selection_set(
        client_registry.borrow_registry(),
        context,
        mutation_resolvers,
        object_field_resolvers,
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
