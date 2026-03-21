use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::parsers::schema::client;

use super::NonNullableValue;
use super::ast::{
    NonNullableResolverIntrospectionValue, ResolverFuture, ResolverRoot, Value,
    Values,
};
use super::registry::TypeRegistry;
use super::scalar::Scalar;
use super::variables::ResolvedVariables;

pub type QueryResolver<S, C> = Box<
    dyn for<'a> Fn(
        &'a ResolverRoot<S>,
        &'a C,
        &'a ResolvedVariables,
    ) -> ResolverFuture<'a, S>,
>;
pub type QueryResolversMap<S, C> =
    HashMap<(String, String), QueryResolver<S, C>>;

pub fn execute_potential_selection_and_serialize<
    'a,
    'b,
    C,
    S: Scalar,
    T: TypeRegistry<S>,
>(
    context: &'a C,
    query_resolvers: &'a QueryResolversMap<S, C>,
    type_registry: &'a T,
    resolver_root: &'a ResolverRoot<S>,
    selection: &'a Option<Rc<client::ast::FragmentSpec>>,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<Box<dyn Future<Output = Result<Value<S>, String>> + 'a>> {
    Box::pin(async move {
        let Some(non_nullable) = resolver_root.create_introspection_value()
        else {
            return resolver_root.to_value(Vec::new());
        };
        match non_nullable {
            NonNullableResolverIntrospectionValue::Literal(
                object_name,
                literal,
            ) => {
                let callable_fields = if let Some(spec) = selection {
                    execute_fragment(
                        context,
                        query_resolvers,
                        type_registry,
                        literal,
                        &literal.get_existing_fields(),
                        &object_name,
                        spec,
                        variables,
                    )
                    .await?
                } else {
                    Vec::new()
                };
                return resolver_root.to_value(callable_fields);
            }
            NonNullableResolverIntrospectionValue::Array(array) => {
                Ok(Value::NonNullable(NonNullableValue::Array(
                    futures::future::join_all(array.iter().map(
                        async |optional_element| -> Result<Value<S>, String> {
                            execute_potential_selection_and_serialize(
                                context,
                                query_resolvers,
                                type_registry,
                                *optional_element,
                                selection,
                                variables,
                            )
                            .await
                        },
                    ))
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, String>>()?,
                )))
            }
        }
    })
}

fn execute_fragment<'a: 'b, 'b, C, S: Scalar, T: TypeRegistry<S>>(
    context: &'a C,
    query_resolvers: &'a QueryResolversMap<S, C>,
    type_registry: &'a T,
    resolver_root: &'a ResolverRoot<S>,
    resolver_root_existing_fields: &'a HashSet<String>,
    object_name: &'a str,
    spec: &'a client::ast::FragmentSpec,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<
    Box<dyn Future<Output = Result<Vec<(String, Value<S>)>, String>> + 'a>,
> {
    Box::pin(async move {
        match spec {
            client::ast::FragmentSpec::Object(obj) => {
                execute_object_selection_set(
                    context,
                    query_resolvers,
                    type_registry,
                    object_name,
                    resolver_root,
                    resolver_root_existing_fields,
                    &obj.selections,
                    variables,
                )
                .await
            }

            client::ast::FragmentSpec::Union(union) => {
                execute_union_selection_set(
                    context,
                    query_resolvers,
                    type_registry,
                    object_name,
                    resolver_root,
                    resolver_root_existing_fields,
                    &union.selections,
                    variables,
                )
                .await
            }
            client::ast::FragmentSpec::Interface(interface) => {
                execute_object_selection_set(
                    context,
                    query_resolvers,
                    type_registry,
                    object_name,
                    resolver_root,
                    resolver_root_existing_fields,
                    &interface.selections,
                    variables,
                )
                .await
            }
        }
    })
}

async fn execute_field<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    resolver_root: &ResolverRoot<S>,
    object_name: &str,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Value<S>, String> {
    let resolver_key = (object_name.to_string(), field.name.clone());

    let value = {
        let resolver = query_resolvers.get(&resolver_key).ok_or_else(|| {
            format!("No resolver for {}.{}", object_name, field.name)
        })?;
        resolver(resolver_root, context, variables).await?
    };
    execute_potential_selection_and_serialize(
        context,
        query_resolvers,
        type_registry,
        value.as_ref(),
        &field.selection,
        variables,
    )
    .await
}

async fn execute_union_selection_set<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    selections: &[client::ast::UnionSelection],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    futures::future::join_all(selections.iter().map(async |selection| {
        match selection {
            client::ast::UnionSelection::TypenameField(typename_field) => {
                super::shared::execute_typename_field(
                    object_name,
                    typename_field,
                )
                .map(|t_field| vec![t_field])
            }
            client::ast::UnionSelection::ObjectConditionalSpreadSelection(
                spread,
            ) => {
                let spread_object_name = &spread.selection.r#type.borrow().name;
                if spread_object_name != object_name {
                    return Ok(Vec::new());
                };
                execute_object_selection_set(
                    context,
                    query_resolvers,
                    type_registry,
                    object_name,
                    resolver_root,
                    resolver_root_existing_fields,
                    &spread.selection.selections,
                    variables,
                )
                .await
            }
            client::ast::UnionSelection::UnionConditionalSpreadSelection(_) => {
                panic!("Unexpected UnionConditionalSpreadSelection on union")
            }

            client::ast::UnionSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                let result = execute_fragment(
                    context,
                    query_resolvers,
                    type_registry,
                    resolver_root,
                    resolver_root_existing_fields,
                    object_name,
                    &fragment.spec,
                    variables,
                )
                .await;
                println!("fragment: {:?}", fragment);
                return result;
            }
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, String>>()
    .map(|a| a.into_iter().flatten().collect())
}

async fn execute_field_selection<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    object_name: &str,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    if resolver_root_existing_fields.contains(&field.alias) {
        return Ok(Vec::new());
    };
    let value = execute_field(
        context,
        query_resolvers,
        type_registry,
        resolver_root,
        &object_name,
        field,
        variables,
    )
    .await?;
    Ok(vec![(field.alias.clone(), value)])
}

async fn execute_object_selection<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection,
) -> Result<Vec<(String, Value<S>)>, String> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field(object_name, field)
                .map(|t_field| vec![t_field])
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                context,
                query_resolvers,
                type_registry,
                resolver_root,
                resolver_root_existing_fields,
                &object_name,
                field,
                variables,
            )
            .await
        }

        client::ast::ObjectSelection::SpreadSelection(spread) => {
            let fragment = spread.fragment.borrow();
            execute_fragment(
                context,
                query_resolvers,
                type_registry,
                resolver_root,
                resolver_root_existing_fields,
                object_name,
                &fragment.spec,
                variables,
            )
            .await
        }
    }
}

async fn execute_object_selection_set<'a, C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Vec<(String, Value<S>)>, String> {
            execute_object_selection(
                context,
                query_resolvers,
                type_registry,
                object_name,
                resolver_root,
                resolver_root_existing_fields,
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

pub async fn execute_query_operation<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    query_resolvers: &QueryResolversMap<S, C>,
    type_registry: &T,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<Values<S>, String> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err("Root query operation must select an object".into());
    };

    let mut root_value: Box<ResolverRoot<S>> = Box::new(&());
    execute_object_selection_set(
        context,
        query_resolvers,
        type_registry,
        operation.r#type.to_object_name(),
        root_value.as_mut(),
        &HashSet::new(),
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
