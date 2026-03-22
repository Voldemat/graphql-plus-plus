use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{ResolverFuture, Value, Values};
use super::queries::QueryResolversMap;
use super::registry::TypeRegistry;
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type MutationResolver<S, C> =
    Box<dyn for<'a> Fn(&'a C, &'a ResolvedVariables) -> ResolverFuture<'a, S>>;
pub type MutationResolversMap<S, C> = HashMap<&'static str, MutationResolver<S, C>>;

async fn execute_field<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<S, C>,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Value<S>, String> {
    let value = {
        let resolver =
            mutation_resolvers.get(field.name.as_str()).ok_or_else(|| {
                format!("No mutation resolver for {}", field.name)
            })?;
        resolver(context, variables).await?
    };
    super::queries::execute_potential_selection_and_serialize(
        context,
        query_resolvers,
        type_registry,
        value.to_value()?,
        field.selection.as_ref(),
        variables,
    )
    .await
}

async fn execute_fragment<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<S, C>,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    spec: &client::ast::FragmentSpec,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    match spec {
        client::ast::FragmentSpec::Object(obj) => {
            execute_object_selection_set(
                context,
                mutation_resolvers,
                query_resolvers,
                type_registry,
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

async fn execute_field_selection<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<S, C>,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    let value = execute_field(
        context,
        mutation_resolvers,
        query_resolvers,
        type_registry,
        field,
        variables,
    )
    .await?;
    Ok(vec![(field.alias.clone(), value)])
}

async fn execute_object_selection<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<S, C>,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection,
) -> Result<Vec<(String, Value<S>)>, String> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field("Mutation", field)
                .map(|t_field| vec![t_field])
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                context,
                mutation_resolvers,
                query_resolvers,
                type_registry,
                field,
                variables,
            )
            .await
        }

        client::ast::ObjectSelection::SpreadSelection(spread) => {
            let fragment = spread.fragment.borrow();
            execute_fragment(
                context,
                mutation_resolvers,
                query_resolvers,
                type_registry,
                &fragment.spec,
                variables,
            )
            .await
        }
    }
}

async fn execute_object_selection_set<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<S, C>,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Vec<(String, Value<S>)>, String> {
            execute_object_selection(
                context,
                mutation_resolvers,
                query_resolvers,
                type_registry,
                variables,
                selection,
            )
            .await
        },
    ))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, String>>()
    .map(|a| a.into_iter().flatten().collect())
}

pub async fn execute_mutation_operation<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<S, C>,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<Values<S>, String> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err("Root mutation operation must select an object".into());
    };

    execute_object_selection_set(
        context,
        mutation_resolvers,
        query_resolvers,
        type_registry,
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
