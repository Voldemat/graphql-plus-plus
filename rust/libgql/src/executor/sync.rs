use std::collections::HashMap;

use crate::{
    executor::{
        LiteralValue, NonNullableValue, ResolvedVariables, Scalar, Value,
        Values, ast::ResolverRoot,
    },
    parsers::schema::{client, server},
};

pub type SyncResolver<C, S> = Box<
    dyn Fn(
        &ResolverRoot<S>,
        &mut C,
        &ResolvedVariables,
    )
        -> std::pin::Pin<Box<dyn Future<Output = Result<Value<S>, String>>>>,
>;
pub type SyncResolversMap<C, S> = HashMap<(String, String), SyncResolver<C, S>>;

fn execute_fragment_on_value<'a, C, S: Scalar>(
    context: &'a mut C,
    resolvers: &'a SyncResolversMap<C, S>,
    parent: &'a mut Value<S>,
    spec: &'a client::ast::FragmentSpec,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<Box<dyn Future<Output = Result<(), String>> + 'a>> {
    Box::pin(async move {
        let Value::NonNullable(non_nullable) = parent else {
            return Ok(());
        };
        match non_nullable {
            NonNullableValue::Literal(literal) => match literal {
                LiteralValue::Object(object_name, object_value) => {
                    execute_fragment(
                        context,
                        resolvers,
                        object_name,
                        object_value,
                        spec,
                        variables,
                    )
                    .await?;
                    return Ok(());
                }
                LiteralValue::Scalar(_) => {
                    panic!("Unexpected fragment selection on scalar value")
                }
            },
            NonNullableValue::Array(array) => {
                for value in array {
                    execute_fragment_on_value(
                        context, resolvers, value, spec, variables,
                    )
                    .await?;
                }
                return Ok(());
            }
        }
    })
}

fn execute_fragment<'a, C, S: Scalar>(
    context: &'a mut C,
    resolvers: &'a SyncResolversMap<C, S>,
    object_name: &'a str,
    parent: &'a mut Values<S>,
    spec: &'a client::ast::FragmentSpec,
    variables: &'a ResolvedVariables,
) -> std::pin::Pin<Box<dyn Future<Output = Result<(), String>> + 'a>> {
    Box::pin(async move {
        match spec {
            client::ast::FragmentSpec::Object(obj) => {
                execute_object_selection_set(
                    context,
                    resolvers,
                    &obj.r#type.borrow(),
                    object_name,
                    parent,
                    &obj.selections,
                    variables,
                )
                .await
            }

            client::ast::FragmentSpec::Union(union) => {
                execute_union_selection_set(
                    context,
                    resolvers,
                    &union.r#type.borrow(),
                    object_name,
                    parent,
                    &union.selections,
                    variables,
                )
                .await
            }
            client::ast::FragmentSpec::Interface(interface) => {
                execute_interface_selection_set(
                    context,
                    resolvers,
                    &interface.r#type.borrow(),
                    object_name,
                    parent,
                    &interface.selections,
                    variables,
                )
                .await
            }
        }
    })
}

async fn execute_field<C, S: Scalar>(
    context: &mut C,
    resolvers: &SyncResolversMap<C, S>,
    object_name: &str,
    parent: &Values<S>,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Value<S>, String> {
    let resolver_key = (object_name.to_string(), field.name.clone());

    let resolver = resolvers.get(&resolver_key).ok_or_else(|| {
        format!("No resolver for {}.{}", object_name, field.name)
    })?;
    let mut value = resolver(parent, context, variables).await?;

    if let Some(fragment) = &field.selection {
        execute_fragment_on_value(
            context, resolvers, &mut value, fragment, variables,
        )
        .await?;
    }

    Ok(value)
}

fn execute_typename_field<S: Scalar>(
    object_name: &str,
    parent: &mut Values<S>,
    field: &client::ast::TypenameField,
) -> Result<(), String> {
    let typename = object_name.to_string();
    parent.insert(
        field
            .alias
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or("__typename")
            .into(),
        Value::NonNullable(NonNullableValue::Literal(LiteralValue::Scalar(
            S::from_string(&typename)?,
        ))),
    );
    Ok(())
}

async fn execute_union_selection_set<C, S: Scalar>(
    context: &mut C,
    resolvers: &SyncResolversMap<C, S>,
    union_type: &server::ast::Union,
    object_name: &str,
    parent: &mut Values<S>,
    selections: &[client::ast::UnionSelection],
    variables: &ResolvedVariables,
) -> Result<(), String> {
    for selection in selections {
        match selection {
            client::ast::UnionSelection::TypenameField(typename_field) => {
                execute_typename_field(object_name, parent, typename_field)?;
            }
            client::ast::UnionSelection::ObjectConditionalSpreadSelection(
                spread,
            ) => {
                let spread_object_name = &spread.selection.r#type.borrow().name;
                if spread_object_name != object_name {
                    return Ok(());
                }
                execute_object_selection_set(
                    context,
                    resolvers,
                    &union_type.items.get(object_name).unwrap().borrow(),
                    object_name,
                    parent,
                    &spread.selection.selections,
                    variables,
                )
                .await?;
                ()
            }
            client::ast::UnionSelection::UnionConditionalSpreadSelection(_) => {
                panic!("Unexpected UnionConditionalSpreadSelection on union")
            }

            client::ast::UnionSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    resolvers,
                    object_name,
                    parent,
                    &fragment.spec,
                    variables,
                )
                .await?;
                return Ok(());
            }
        };
    }
    return Ok(());
}

async fn execute_object_selection_set<C, S: Scalar>(
    context: &mut C,
    resolvers: &SyncResolversMap<C, S>,
    object_type: &server::ast::ObjectType,
    object_name: &str,
    parent: &mut Values<S>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<(), String> {
    for selection in selections {
        match selection {
            client::ast::ObjectSelection::TypenameField(field) => {
                execute_typename_field(object_name, parent, field)?;
            }

            client::ast::ObjectSelection::FieldSelection(field) => {
                if parent.contains_key(&field.alias) {
                    return Ok(());
                }
                let value = execute_field(
                    context,
                    resolvers,
                    &object_type.name,
                    parent,
                    field,
                    variables,
                )
                .await?;
                parent.insert(field.alias.clone(), value);
                return Ok(());
            }

            client::ast::ObjectSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    resolvers,
                    object_name,
                    parent,
                    &fragment.spec,
                    variables,
                )
                .await?;
                return Ok(());
            }
        };
    }
    return Ok(());
}

async fn execute_interface_selection_set<C, S: Scalar>(
    context: &mut C,
    resolvers: &SyncResolversMap<C, S>,
    interface_type: &server::ast::Interface,
    object_name: &str,
    parent: &mut Values<S>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<(), String> {
    for selection in selections {
        match selection {
            client::ast::ObjectSelection::TypenameField(_) => {
                parent.insert(
                    "__typename".into(),
                    Value::NonNullable(NonNullableValue::Literal(
                        LiteralValue::Scalar(S::from_string(object_name)?),
                    )),
                );
                return Ok(());
            }

            client::ast::ObjectSelection::FieldSelection(field) => {
                if parent.contains_key(&field.alias) {
                    return Ok(());
                }
                let value = execute_field(
                    context,
                    resolvers,
                    &interface_type.name,
                    parent,
                    field,
                    variables,
                )
                .await?;
                parent.insert(field.alias.clone(), value);
                return Ok(());
            }

            client::ast::ObjectSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    resolvers,
                    object_name,
                    parent,
                    &fragment.spec,
                    variables,
                )
                .await?;
                return Ok(());
            }
        };
    }
    return Ok(());
}

pub async fn execute_sync_operation<C, S: Scalar>(
    context: &mut C,
    resolvers: &SyncResolversMap<C, S>,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<Values<S>, String> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err("Root operation must select an object".into());
    };

    let mut root_value: ResolverRoot<S> = Values::<S>::new();
    execute_object_selection_set(
        context,
        resolvers,
        object,
        operation.r#type.to_object_name(),
        &mut root_value,
        &fragment_spec.selections,
        variables,
    )
    .await?;
    return Ok(root_value);
}
