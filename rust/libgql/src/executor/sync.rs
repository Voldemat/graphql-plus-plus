use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    executor::{
        LiteralValue, NonNullableValue, ResolvedVariables, Scalar,
        TypeRegistry, Value, Values,
        ast::{ResolverIntrospectionValue, ResolverRoot},
    },
    parsers::schema::{client, server, shared},
};

pub type ResolverFuture<S> =
    std::pin::Pin<Box<dyn Future<Output = Result<ResolverRoot<S>, String>>>>;
pub type SyncResolver<S, C> =
    Box<dyn Fn(&ResolverRoot<S>, &C, &ResolvedVariables) -> ResolverFuture<S>>;
pub type SyncResolversMap<S, C> = HashMap<(String, String), SyncResolver<S, C>>;

pub fn execute_fragment_on_value<'a, 'b, C, S: Scalar, T: TypeRegistry<S>>(
    context: &'a C,
    resolvers: &'a SyncResolversMap<S, C>,
    type_registry: &'a T,
    value: &'a ResolverIntrospectionValue<'b, S>,
    spec: &'a client::ast::FragmentSpec,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<
    Box<dyn Future<Output = Result<Vec<(String, Value<S>)>, String>> + 'a>,
> {
    Box::pin(async move { Ok(Vec::new()) })
}

fn execute_fragment<'a, C, S: Scalar, T: TypeRegistry<S>>(
    context: &'a C,
    resolvers: &'a SyncResolversMap<S, C>,
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
                    resolvers,
                    type_registry,
                    object_name,
                    &obj.r#type.borrow().fields,
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
                    resolvers,
                    type_registry,
                    &union.r#type.borrow(),
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
                    resolvers,
                    type_registry,
                    object_name,
                    &interface.r#type.borrow().fields,
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
    resolvers: &SyncResolversMap<S, C>,
    type_registry: &T,
    resolver_root: &ResolverRoot<S>,
    object_name: &str,
    object_fields: &indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    >,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Value<S>, String> {
    let resolver_key = (object_name.to_string(), field.name.clone());

    let resolver = resolvers.get(&resolver_key).ok_or_else(|| {
        format!("No resolver for {}.{}", object_name, field.name)
    })?;
    let value_future = resolver(resolver_root, context, variables);
    let value = value_future.await?;
    let mut callable_fields = Vec::new();

    if let Some(fragment) = &field.selection {
        callable_fields = execute_fragment_on_value(
            context,
            resolvers,
            type_registry,
            &value.create_introspection_value(),
            fragment,
            variables,
        )
        .await?;
    }

    value.to_value(callable_fields)
}

fn execute_typename_field<S: Scalar>(
    object_name: &str,
    field: &client::ast::TypenameField,
) -> Result<(String, Value<S>), String> {
    Ok((
        field
            .alias
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or("__typename")
            .into(),
        Value::NonNullable(NonNullableValue::Literal(LiteralValue::Scalar(
            S::from_str(&object_name.to_string())?,
        ))),
    ))
}

async fn execute_union_selection_set<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    resolvers: &SyncResolversMap<S, C>,
    type_registry: &T,
    union_type: &server::ast::Union,
    object_name: &str,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    selections: &[client::ast::UnionSelection],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    futures::future::join_all(selections.iter().map(async |selection| {
        match selection {
            client::ast::UnionSelection::TypenameField(typename_field) => {
                execute_typename_field(object_name, typename_field)
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
                    resolvers,
                    type_registry,
                    object_name,
                    &union_type.items.get(object_name).unwrap().borrow().fields,
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
                execute_fragment(
                    context,
                    resolvers,
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
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, String>>()
    .map(|a| a.into_iter().flatten().collect())
}

async fn execute_field_selection<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    resolvers: &SyncResolversMap<S, C>,
    type_registry: &T,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    object_name: &str,
    object_fields: &indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    >,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    if resolver_root_existing_fields.contains(&field.alias) {
        return Ok(Vec::new());
    };
    let value = execute_field(
        context,
        resolvers,
        type_registry,
        resolver_root,
        &object_name,
        &object_fields,
        field,
        variables,
    )
    .await?;
    Ok(vec![(field.alias.clone(), value)])
}

async fn execute_object_selection_set<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    resolvers: &SyncResolversMap<S, C>,
    type_registry: &T,
    object_name: &str,
    object_fields: &indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    >,
    resolver_root: &ResolverRoot<S>,
    resolver_root_existing_fields: &HashSet<String>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Vec<(String, Value<S>)>, String> {
    futures::future::join_all(selections.iter().map(
        async |selection| -> Result<Vec<(String, Value<S>)>, String> {
            match selection {
                client::ast::ObjectSelection::TypenameField(field) => {
                    execute_typename_field(object_name, field)
                        .map(|t_field| vec![t_field])
                }

                client::ast::ObjectSelection::FieldSelection(field) => {
                    execute_field_selection(
                        context,
                        resolvers,
                        type_registry,
                        resolver_root,
                        resolver_root_existing_fields,
                        &object_name,
                        &object_fields,
                        field,
                        variables,
                    )
                    .await
                }

                client::ast::ObjectSelection::SpreadSelection(spread) => {
                    let fragment = spread.fragment.borrow();
                    execute_fragment(
                        context,
                        resolvers,
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
        },
    ))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, String>>()
    .map(|a| a.into_iter().flatten().collect())
}

pub async fn execute_sync_operation<C, S: Scalar, T: TypeRegistry<S>>(
    context: &C,
    resolvers: &SyncResolversMap<S, C>,
    type_registry: &T,
    object: &server::ast::ObjectType,
    operation: client::ast::Operation,
    variables: ResolvedVariables,
) -> Result<Values<S>, String> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err("Root operation must select an object".into());
    };

    let mut root_value: ResolverRoot<S> = Box::new(&());
    execute_object_selection_set(
        context,
        resolvers,
        type_registry,
        operation.r#type.to_object_name(),
        &object.fields,
        &mut root_value,
        &HashSet::new(),
        &fragment_spec.selections,
        &variables,
    )
    .await
    .map(|entries| Values::from_iter(entries))
}
