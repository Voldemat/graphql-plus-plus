pub mod hashmap_registry;
pub mod registry;
pub mod scalar;
pub mod variables;
pub use hashmap_registry::{GQLEnum, GQLInput, GQLScalar, HashMapRegistry};
pub use registry::Registry;
pub use scalar::Scalar;
use std::collections::HashMap;
pub use variables::{
    LiteralVariable, NonNullableVariable, ResolvedVariables, Variable,
    Variables, resolve_operation_parameters,
};

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

type ResolverRoot<S> = Variable<S>;

type Resolver<C, S> = Box<
    dyn Fn(
        &ResolverRoot<S>,
        &mut C,
        &ResolvedVariables,
    ) -> Result<Variable<S>, String>,
>;
type ResolversMap<C, S> = HashMap<(String, String), Resolver<C, S>>;

fn execute_fragment<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    parent: &ResolverRoot<S>,
    spec: &client::ast::FragmentSpec,
    variables: &ResolvedVariables,
) -> Result<Variables<S>, String> {
    match spec {
        client::ast::FragmentSpec::Object(obj) => execute_object_selection_set(
            context,
            registry,
            resolvers,
            &obj.r#type.borrow(),
            parent,
            &obj.selections,
            variables,
        ),

        client::ast::FragmentSpec::Union(union) => {
            // runtime type check required
            // choose matching conditional spread
            Ok(Variables::<S>::new())
        }

        client::ast::FragmentSpec::Interface(interface) => {
            // resolve runtime object type
            Ok(Variables::<S>::new())
        }
    }
}

fn execute_field<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object_type: &server::ast::ObjectType,
    parent: &ResolverRoot<S>,
    field: &client::ast::FieldSelection,
    variables: &ResolvedVariables,
) -> Result<Variable<S>, String> {
    let resolver_key = (object_type.name.clone(), field.name.clone());

    let resolver = resolvers
        .get(&resolver_key)
        .ok_or_else(|| format!("No resolver for {}", field.name))?;

    let value = resolver(parent, context, variables)?;

    if let Some(fragment) = &field.selection {
        execute_fragment(
            context, registry, resolvers, &value, fragment, variables,
        )?;
    }

    Ok(value)
}

fn execute_object_selection_set<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object_type: &server::ast::ObjectType,
    parent: &ResolverRoot<S>,
    selections: &[client::ast::ObjectSelection],
    variables: &ResolvedVariables,
) -> Result<Variables<S>, String> {
    let mut result = Variables::<S>::new();
    for selection in selections {
        match selection {
            client::ast::ObjectSelection::TypenameField(_) => {
                let typename = object_type.name.clone();
                result.insert(
                    "__typename".into(),
                    Variable::NonNullable(NonNullableVariable::Literal(
                        LiteralVariable::Scalar(S::from_string(&typename)?),
                    )),
                );
            }

            client::ast::ObjectSelection::FieldSelection(field) => {
                execute_field(
                    context,
                    registry,
                    resolvers,
                    object_type,
                    parent,
                    field,
                    variables,
                )?;
            }

            client::ast::ObjectSelection::SpreadSelection(spread) => {
                let fragment = spread.fragment.borrow();
                execute_fragment(
                    context,
                    registry,
                    resolvers,
                    parent,
                    &fragment.spec,
                    variables,
                )?;
            }
        };
    };
    Ok(result)
}

fn execute_sync_operation<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<Variables<S>, String> {
    let client::ast::FragmentSpec::Object(fragment_spec) =
        &operation.fragment_spec
    else {
        return Err("Root operation must select an object".into());
    };

    let root_value: ResolverRoot<S> = Variable::Null;
    execute_object_selection_set(
        context,
        registry,
        resolvers,
        object,
        &root_value,
        &fragment_spec.selections,
        variables,
    )
}

fn execute_subscription_operation<C, S: Scalar>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    object: &server::ast::ObjectType,
    operation: &client::ast::Operation,
    variables: &ResolvedVariables,
) -> Result<Variables<S>, String> {
    Ok(Variables::<S>::new())
}

fn execute_operation<C, S: Scalar, R: Registry<S>>(
    context: &mut C,
    registry: &TypeRegistry,
    resolvers: &ResolversMap<C, S>,
    parse_registry: &R,
    operation: &client::ast::Operation,
    variables: &Variables<S>,
) -> Result<Variables<S>, String> {
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
    variables: &Variables<S>,
    operation: &Option<String>,
) -> Result<(), Error> {
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
    execute_operation::<C, S, R>(
        context,
        &mut local_registry,
        resolvers,
        parse_registry,
        &operation.borrow(),
        variables,
    )
    .unwrap();
    Ok(())
}
