use std::collections::HashMap;
use std::sync::Arc;

use crate::parsers::schema::client;

use super::ast::{
    NonNullableResolverIntrospectionValue, ResolverFuture,
    ResolverIntrospectionValue, ResolverRoot, Value, Values,
};
use super::scalar::Scalar;
use super::variables::ResolvedVariables;
use super::{LiteralValue, NonNullableValue};

pub type ObjectFieldResolver<S, C> = dyn for<'a> Fn(
    &'a ResolverRoot<S>,
    &'a C,
    &'a ResolvedVariables,
) -> ResolverFuture<'a, S> + Sync;
pub type ObjectFieldResolversMap<'a, S, C> =
    HashMap<(&'a str, &'a str), &'a ObjectFieldResolver<S, C>>;

pub fn execute_potential_selection_and_serialize<'a, 'b, C, S: Scalar>(
    context: &'a C,
    object_field_resolvers: &'a ObjectFieldResolversMap<S, C>,
    resolver_root_introspection_value: ResolverIntrospectionValue<'a, S>,
    selection: Option<&'a Arc<client::ast::FragmentSpec>>,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<Box<dyn Future<Output = Result<Value<S>, String>> + 'a>> {
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
                    context,
                    object_field_resolvers,
                    resolver_root,
                    &fields,
                    &object_name,
                    spec.as_ref(),
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
                Ok(Value::NonNullable(NonNullableValue::Array(
                    futures::future::join_all(array.into_iter().map(
                        async |optional_element| -> Result<Value<S>, String> {
                            execute_potential_selection_and_serialize(
                                context,
                                object_field_resolvers,
                                optional_element,
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

fn execute_fragment<'a: 'b, 'b, C, S: Scalar>(
    context: &'a C,
    object_field_resolvers: &'a ObjectFieldResolversMap<S, C>,
    resolver_root: &'a ResolverRoot<S>,
    fields: &'a HashMap<&'a str, &'a ResolverRoot<S>>,
    object_name: &'a str,
    spec: &'a client::ast::FragmentSpec,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<Box<dyn Future<Output = Result<Values<S>, String>> + 'a>> {
    Box::pin(async move {
        match spec {
            client::ast::FragmentSpec::Object(obj) => {
                execute_object_selection_set(
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

async fn execute_field<C, S: Scalar>(
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    resolver_root: &ResolverRoot<S>,
    object_name: &str,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
    existing_value: Option<&ResolverRoot<S>>,
) -> Result<Value<S>, String> {
    let resolver_key = (object_name, field.name.as_str());
    let owned: Box<ResolverRoot<S>>;
    let value = if let Some(v) = existing_value {
        v
    } else {
        let resolver =
            object_field_resolvers.get(&resolver_key).ok_or_else(|| {
                format!("No resolver for {}.{}", object_name, field.name)
            })?;
        owned = resolver(resolver_root, context, variables).await?;
        owned.as_ref()
    };
    execute_potential_selection_and_serialize(
        context,
        object_field_resolvers,
        value.to_value()?,
        field.selection.as_ref(),
        variables,
    )
    .await
}

async fn execute_union_selection_set<C, S: Scalar>(
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    existing_fields: &HashMap<&str, &ResolverRoot<S>>,
    selections: &[client::ast::UnionSelection],
    variables: &ResolvedVariables,
) -> Result<Values<S>, String> {
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
                let spread_object_name = &spread.selection.r#type.read().unwrap().name;
                if spread_object_name != object_name {
                    return Ok(Values::new());
                };
                execute_object_selection_set(
                    context,
                    object_field_resolvers,
                    object_name,
                    resolver_root,
                    existing_fields,
                    &spread.selection.selections,
                    variables,
                )
                .await
            }
            client::ast::UnionSelection::UnionConditionalSpreadSelection(_) => {
                panic!("Unexpected UnionConditionalSpreadSelection on union")
            }

            client::ast::UnionSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.read().unwrap();
                let result = execute_fragment(
                    context,
                    object_field_resolvers,
                    resolver_root,
                    existing_fields,
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

async fn execute_field_selection<C, S: Scalar>(
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    resolver_root: &ResolverRoot<S>,
    existing_field_value: Option<&ResolverRoot<S>>,
    object_name: &str,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Values<S>, String> {
    let value = execute_field(
        context,
        object_field_resolvers,
        resolver_root,
        &object_name,
        field,
        variables,
        existing_field_value,
    )
    .await?;
    Ok(Values::from_iter([(field.alias.clone(), value)]))
}

async fn execute_object_selection<C, S: Scalar>(
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    existing_fields: &HashMap<&str, &ResolverRoot<S>>,
    variables: &ResolvedVariables,
    selection: &client::ast::ObjectSelection,
) -> Result<Values<S>, String> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            super::shared::execute_typename_field(object_name, field)
                .map(|t_field| Values::from_iter([t_field]))
        }

        client::ast::ObjectSelection::FieldSelection(field) => {
            execute_field_selection(
                context,
                object_field_resolvers,
                resolver_root,
                existing_fields.get(field.name.as_str()).copied(),
                &object_name,
                field,
                variables,
            )
            .await
        }

        client::ast::ObjectSelection::SpreadSelection(spread) => {
            let fragment = spread.fragment.read().unwrap();
            execute_fragment(
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

async fn execute_object_selection_set<'a, C, S: Scalar>(
    context: &C,
    object_field_resolvers: &ObjectFieldResolversMap<'_, S, C>,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    existing_fields: &HashMap<&str, &ResolverRoot<S>>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Values<S>, String> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Values<S>, String> {
            execute_object_selection(
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
    .collect::<Result<Vec<_>, String>>()
    .map(|a| a.into_iter().flatten().collect())
}
