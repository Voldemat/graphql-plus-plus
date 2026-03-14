pub mod ast;
pub mod hashmap_registry;
pub mod registry;
pub mod scalar;
pub mod variables;
pub use ast::{LiteralValue, NonNullableValue, Value, Values};
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::Registry;
pub use scalar::Scalar;
use std::collections::HashMap;
pub use variables::{ResolvedVariables, resolve_operation_parameters};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client, server, type_registry::TypeRegistry},
    },
};

#[derive(Debug, derive_more::From)]
pub enum Error {
    Lexer(Vec<lexer::Error>),
    FileParser(file::client::Error),
    OperationIsNotDefined(String),
    OperationNameIsNotDefined,
    NoOperationsAreDefined,
}

pub type ResolverRoot<S> = Values<S>;

pub type Resolver<C, S> = Box<
    dyn Fn(
        &ResolverRoot<S>,
        &mut C,
        &ResolvedVariables,
    ) -> Result<Value<S>, String>,
>;
pub type ResolversMap<C, S> = HashMap<(String, String), Resolver<C, S>>;

fn execute_fragment_on_value<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    parent: &mut Value<S>,
    spec: &client::ast::FragmentSpec,
    variables: &ResolvedVariables,
) -> Result<(), String> {
    let Value::NonNullable(non_nullable) = parent else {
        return Ok(());
    };
    match non_nullable {
        NonNullableValue::Literal(literal) => match literal {
            LiteralValue::Object(object_name, object_value) => {
                execute_fragment(
                    context,
                    registry,
                    resolvers,
                    object_name,
                    object_value,
                    spec,
                    variables,
                )?;
                return Ok(());
            }
            LiteralValue::Scalar(_) => {
                panic!("Unexpected fragment selection on scalar value")
            }
        },
        NonNullableValue::Array(array) => {
            for value in array {
                execute_fragment_on_value(
                    context, registry, resolvers, value, spec, variables,
                )?;
            }
            return Ok(());
        }
    }
}

fn execute_fragment<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object_name: &str,
    parent: &mut Values<S>,
    spec: &client::ast::FragmentSpec,
    variables: &ResolvedVariables,
) -> Result<(), String> {
    match spec {
        client::ast::FragmentSpec::Object(obj) => execute_object_selection_set(
            context,
            registry,
            resolvers,
            &obj.r#type.borrow(),
            object_name,
            parent,
            &obj.selections,
            variables,
        ),

        client::ast::FragmentSpec::Union(union) => execute_union_selection_set(
            context,
            registry,
            resolvers,
            &union.r#type.borrow(),
            object_name,
            parent,
            &union.selections,
            variables,
        ),
        client::ast::FragmentSpec::Interface(interface) => {
            execute_interface_selection_set(
                context,
                registry,
                resolvers,
                &interface.r#type.borrow(),
                object_name,
                parent,
                &interface.selections,
                variables,
            )
        }
    }
}

fn execute_field<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object_name: &str,
    parent: &Values<S>,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Value<S>, String> {
    let resolver_key = (object_name.to_string(), field.name.clone());

    let resolver = resolvers.get(&resolver_key).ok_or_else(|| {
        format!("No resolver for {}.{}", object_name, field.name)
    })?;

    let mut value = resolver(parent, context, variables)?;

    if let Some(fragment) = &field.selection {
        execute_fragment_on_value(
            context, registry, resolvers, &mut value, fragment, variables,
        )?;
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

fn execute_union_selection_set<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
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
                    registry,
                    resolvers,
                    &union_type.items.get(object_name).unwrap().borrow(),
                    object_name,
                    parent,
                    &spread.selection.selections,
                    variables,
                )?;
                ()
            }
            client::ast::UnionSelection::UnionConditionalSpreadSelection(_) => {
                panic!("Unexpected UnionConditionalSpreadSelection on union")
            }

            client::ast::UnionSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    registry,
                    resolvers,
                    object_name,
                    parent,
                    &fragment.spec,
                    variables,
                )?;
                return Ok(());
            }
        };
    }
    return Ok(());
}

fn execute_object_selection_set<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
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
                    registry,
                    resolvers,
                    &object_type.name,
                    parent,
                    field,
                    variables,
                )?;
                parent.insert(field.alias.clone(), value);
                return Ok(());
            }

            client::ast::ObjectSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    registry,
                    resolvers,
                    object_name,
                    parent,
                    &fragment.spec,
                    variables,
                )?;
                return Ok(());
            }
        };
    }
    return Ok(());
}

fn execute_interface_selection_set<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
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
                    registry,
                    resolvers,
                    &interface_type.name,
                    parent,
                    field,
                    variables,
                )?;
                parent.insert(field.alias.clone(), value);
                return Ok(());
            }

            client::ast::ObjectSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    registry,
                    resolvers,
                    object_name,
                    parent,
                    &fragment.spec,
                    variables,
                )?;
                return Ok(());
            }
        };
    }
    return Ok(());
}

fn execute_sync_operation<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
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
        registry,
        resolvers,
        object,
        operation.r#type.to_object_name(),
        &mut root_value,
        &fragment_spec.selections,
        variables,
    )?;
    return Ok(root_value);
}

fn execute_subscription_operation<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<Values<S>, String> {
    Ok(Values::<S>::new())
}

fn execute_operation<C, S: Scalar, R: Registry<S>>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    parse_registry: &R,
    operation: &client::ast::Operation,
    variables: &Values<S>,
) -> Result<Values<S>, String> {
    let resolved_variables = resolve_operation_parameters(
        parse_registry,
        &operation.parameters,
        variables,
    )?;
    match operation.r#type {
        client::ast::OpType::Query => execute_sync_operation::<C, S>(
            context,
            registry,
            resolvers,
            &registry.get_query_object().unwrap().borrow(),
            operation,
            &resolved_variables,
        ),
        client::ast::OpType::Mutation => execute_sync_operation::<C, S>(
            context,
            registry,
            resolvers,
            &registry.get_mutation_object().unwrap().borrow(),
            operation,
            &resolved_variables,
        ),
        client::ast::OpType::Subscription => {
            execute_subscription_operation::<C, S>(
                context,
                registry,
                resolvers,
                &registry.get_subscription_object().unwrap().borrow(),
                operation,
                &resolved_variables,
            )
        }
    }
}

pub fn execute<C, S: Scalar, R: Registry<S>>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    parse_registry: &R,
    client_query: &str,
    variables: &Values<S>,
    operation: &Option<String>,
) -> Result<Values<S>, Error> {
    let tokens = lexer::utils::parse_buffer_into_tokens(client_query)?;
    let source_file = std::rc::Rc::new(file::shared::ast::SourceFile {
        filepath: "<request>".into(),
        buffer: client_query.into(),
    });
    let file_nodes = file::client::Parser::new(
        file::tokens_sources::VecTokensSource::new(tokens, source_file.clone()),
    )
    .parse_ast_nodes()?;
    let mut local_registry = registry.clone();
    let client_schema =
        client::parse_client_schema(&mut local_registry, &file_nodes).unwrap();

    let operation_name = operation.as_ref().map_or_else(
        || {
            if client_schema.operations.len() == 0 {
                return Err(Error::NoOperationsAreDefined);
            }
            if client_schema.operations.len() > 1 {
                return Err(Error::OperationNameIsNotDefined);
            }
            Ok(client_schema.operations.first().unwrap().0)
        },
        Result::Ok,
    )?;
    let operation = client_schema
        .operations
        .get(operation_name)
        .ok_or(Error::OperationIsNotDefined(operation_name.clone()))?;
    let result = execute_operation::<C, S, R>(
        context,
        &mut local_registry,
        resolvers,
        parse_registry,
        &operation.borrow(),
        variables,
    )
    .unwrap();
    Ok(result)
}
