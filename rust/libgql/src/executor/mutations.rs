use std::collections::HashMap;

use crate::parsers::schema::client;

use super::ast::{ResolverFuture, Value, Values};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type MutationResolver<S, C> =
    dyn for<'a> Fn(&'a C, &'a ResolvedVariables) -> ResolverFuture<'a, S>;
pub type MutationResolversMap<'a, S, C> =
    HashMap<&'a str, &'a MutationResolver<S, C>>;

async fn execute_field<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
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
    super::object::execute_potential_selection_and_serialize(
        context,
        object_field_resolvers,
        value.to_value()?,
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
) -> Result<Vec<(String, Value<S>)>, String> {
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
) -> Result<Vec<(String, Value<S>)>, String> {
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
                object_field_resolvers,
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
) -> Result<Vec<(String, Value<S>)>, String> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Vec<(String, Value<S>)>, String> {
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
    .collect::<Result<Vec<_>, String>>()
    .map(|a| a.into_iter().flatten().collect())
}

pub async fn execute_mutation_operation<C, S: Scalar>(
    context: &C,
    mutation_resolvers: &MutationResolversMap<'_, S, C>,
    object_field_resolvers: &super::object::ObjectFieldResolversMap<'_, S, C>,
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
        object_field_resolvers,
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
