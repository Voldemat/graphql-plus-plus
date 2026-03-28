use std::collections::HashMap;

use crate::parsers::schema::{self, client};

use super::ast::{
    GraphqlError, NonNullableResolverIntrospectionValue, ResolverFuture,
    ResolverIntrospectionValue, ResolverRoot, Value, Values,
};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;
use super::{LiteralValue, NonNullableValue};

pub type ObjectFieldResolver<S, C> = dyn for<'a> Fn(
        &'a ResolverRoot<S>,
        &'a C,
        &'a ResolvedVariables,
    ) -> ResolverFuture<'a, S>
    + Send
    + Sync;
pub type ObjectFieldResolversMap<'a, S, C> =
    HashMap<(&'a str, &'a str), &'a ObjectFieldResolver<S, C>>;

pub fn execute_potential_selection_and_serialize<
    'a,
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &'a client::type_registry::TypeRegistry<StringType>,
    context: &'a C,
    object_field_resolvers: &'a ObjectFieldResolversMap<S, C>,
    resolver_root_introspection_value: ResolverIntrospectionValue<'a, S>,
    selection: Option<&'a client::ast::FragmentSpec<StringType>>,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<
    Box<
        dyn Future<Output = Result<Value<S>, Vec<GraphqlError>>>
            + 'a
            + Send
            + Sync,
    >,
> {
    Box::pin(async move {
        let Some(non_nullable) = resolver_root_introspection_value else {
            return Ok(Value::Null);
        };
        match non_nullable {
            NonNullableResolverIntrospectionValue::Scalar(scalar) => {
                Ok(Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(scalar),
                )))
            }
            NonNullableResolverIntrospectionValue::Object(
                resolver_root,
                object_name,
                fields,
            ) => {
                let spec = selection.unwrap();
                execute_fragment(
                    client_registry,
                    context,
                    object_field_resolvers,
                    resolver_root,
                    &fields,
                    &object_name,
                    spec,
                    variables,
                )
                .await
                .map(|values| {
                    Value::NonNullable(NonNullableValue::Literal(
                        LiteralValue::Object(values),
                    ))
                })
            }
            NonNullableResolverIntrospectionValue::Array(array) => {
                Ok(Value::NonNullable(
                    NonNullableValue::Array(
                        futures::future::join_all(
                            array.into_iter().map(
                                async |optional_element| -> Result<
                                    Value<S>,
                                    Vec<GraphqlError>,
                                > {
                                    execute_potential_selection_and_serialize(
                                        client_registry,
                                        context,
                                        object_field_resolvers,
                                        optional_element,
                                        selection,
                                        variables,
                                    )
                                    .await
                                },
                            ),
                        )
                        .await
                        .into_iter()
                        .collect::<Result<Vec<_>, Vec<_>>>()?,
                    ),
                ))
            }
        }
    })
}

fn execute_fragment<
    'args,
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &'args client::type_registry::TypeRegistry<StringType>,
    context: &'args C,
    object_field_resolvers: &'args ObjectFieldResolversMap<S, C>,
    resolver_root: &'args ResolverRoot<S>,
    fields: &'args HashMap<&'args str, &'args ResolverRoot<S>>,
    object_name: &'args str,
    spec: &'args client::ast::FragmentSpec<StringType>,
    variables: &'args ResolvedVariables,
) -> std::pin::Pin<
    Box<
        dyn Future<Output = Result<Values<S>, Vec<GraphqlError>>>
            + 'args
            + Send
            + Sync,
    >,
> {
    Box::pin(async move {
        match spec {
            client::ast::FragmentSpec::Object(obj) => {
                execute_object_selection_set(
                    client_registry,
                    context,
                    object_field_resolvers,
                    object_name,
                    resolver_root,
                    fields,
                    &obj.selections,
                    variables,
                )
                .await
            }

            client::ast::FragmentSpec::Union(union) => {
                execute_union_selection_set(
                    client_registry,
                    context,
                    object_field_resolvers,
                    object_name,
                    resolver_root,
                    fields,
                    &union.selections,
                    variables,
                )
                .await
            }
            client::ast::FragmentSpec::Interface(interface) => {
                execute_object_selection_set(
                    client_registry,
                    context,
                    object_field_resolvers,
                    object_name,
                    resolver_root,
                    fields,
                    &interface.selections,
                    variables,
                )
                .await
            }
        }
    })
}

async fn execute_field<
    'buffer,
    'a,
    'b,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &'a client::type_registry::TypeRegistry<StringType>,
    context: &'a C,
    object_field_resolvers: &'a ObjectFieldResolversMap<'_, S, C>,
    resolver_root: &'a ResolverRoot<S>,
    object_name: &'a str,
    field: &'b client::ast::FieldSelection<StringType>,
    variables: &'a ResolvedVariables,
    existing_value: Option<&'a ResolverRoot<S>>,
) -> Result<Value<S>, Vec<GraphqlError>> {
    let resolver_key = (object_name, field.name.to_str());
    let owned: Box<ResolverRoot<S>>;
    let value = if let Some(v) = existing_value {
        v
    } else {
        let resolver =
            object_field_resolvers.get(&resolver_key).ok_or_else(|| {
                vec![GraphqlError {
                    message: format!(
                        "No resolver for {}.{}",
                        object_name,
                        field.name.to_str()
                    )
                    .into(),
                    path: vec![field.alias.to_str().to_string()],
                }]
            })?;
        owned =
            resolver(resolver_root, context, variables)
                .await
                .map_err(|e| {
                    vec![GraphqlError {
                        message: e,
                        path: vec![field.alias.to_str().to_string()],
                    }]
                })?;
        owned.as_ref()
    };
    let introspection_value = value.to_value().map_err(|e| {
        vec![GraphqlError {
            message: e.into(),
            path: vec![field.alias.to_str().to_string()],
        }]
    })?;
    execute_potential_selection_and_serialize(
        client_registry,
        context,
        object_field_resolvers,
        introspection_value,
        field.selection.as_ref(),
        variables,
    )
    .await
    .map_err(|errors| {
        errors
            .into_iter()
            .map(|mut error| {
                error.path.insert(0, field.alias.to_str().to_string());
                error
            })
            .collect()
    })
}

async fn execute_union_selection_set<
    'buffer,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'buffer>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    existing_fields: &HashMap<&str, &ResolverRoot<S>>,
    selections: &[client::ast::UnionSelection<StringType>],
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    futures::future::join_all(selections.iter().map(async |selection| {
        match selection {
            client::ast::UnionSelection::TypenameField(typename_field) => {
                super::shared::execute_typename_field(
                    object_name,
                    typename_field,
                )
                .map(|t_field| Values::from_iter([t_field]))
            }
            client::ast::UnionSelection::ObjectConditionalSpreadSelection(
                spread,
            ) => {
                if spread.r#type.to_str() != object_name {
                    return Ok(Values::new());
                };
                execute_object_selection_set(
                    client_registry,
                    context,
                    object_field_resolvers,
                    object_name,
                    resolver_root,
                    existing_fields,
                    &spread.selections,
                    variables,
                )
                .await
            }
            client::ast::UnionSelection::UnionConditionalSpreadSelection(_) => {
                panic!("Unexpected UnionConditionalSpreadSelection on union")
            }

            client::ast::UnionSelection::SpreadSelection(spread) => {
                execute_fragment(
                    client_registry,
                    context,
                    object_field_resolvers,
                    resolver_root,
                    existing_fields,
                    object_name,
                    &client_registry
                        .fragments
                        .get(&spread.fragment)
                        .unwrap()
                        .spec,
                    variables,
                )
                .await
            }
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, Vec<_>>>()
    .map(|a| a.into_iter().flatten().collect())
}

async fn execute_field_selection<
    'string_type,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'string_type>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    resolver_root: &ResolverRoot<S>,
    existing_field_value: Option<&ResolverRoot<S>>,
    object_name: &str,
    field: &client::ast::FieldSelection<StringType>,
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    let value = execute_field(
        client_registry,
        context,
        object_field_resolvers,
        resolver_root,
        &object_name,
        field,
        variables,
        existing_field_value,
    )
    .await?;
    Ok(Values::from_iter([(
        field.alias.to_str().to_string(),
        value,
    )]))
}

async fn execute_object_selection<
    'string_type,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'string_type>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    existing_fields: &HashMap<&str, &ResolverRoot<S>>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection<StringType>,
) -> Result<Values<S>, Vec<GraphqlError>> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field(object_name, field)
                .map(|t_field| Values::from_iter([t_field]))
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                client_registry,
                context,
                object_field_resolvers,
                resolver_root,
                existing_fields.get(field.name.to_str()).copied(),
                &object_name,
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
                object_field_resolvers,
                resolver_root,
                existing_fields,
                object_name,
                &fragment.spec,
                variables,
            )
            .await
        }
    }
}

async fn execute_object_selection_set<
    'string_type,
    C: Send + Sync,
    S: Scalar,
    StringType: schema::shared::ast::AsStr<'string_type>,
>(
    client_registry: &client::type_registry::TypeRegistry<StringType>,
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    existing_fields: &HashMap<&str, &ResolverRoot<S>>,
    selections: &[client::ast::ObjectSelection<StringType>],
    variables: &ResolvedVariables,
) -> Result<Values<S>, Vec<GraphqlError>> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Values<S>, Vec<GraphqlError>> {
            execute_object_selection(
                client_registry,
                context,
                object_field_resolvers,
                object_name,
                resolver_root,
                existing_fields,
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
