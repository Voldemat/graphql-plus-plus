use crate::parsers::{
    file,
    schema::{
        client::{ast, directive, errors, fragment, hash, operation},
        server, shared,
    },
};

use super::type_registry::TypeRegistry;

fn fragment_spec_from_name<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    registry: &T,
    name: &file::shared::ast::NameNode<'buffer>,
) -> Result<ast::FragmentSpec<S>, errors::Error<'buffer, S>> {
    if registry.has_object_with_name(name.name) {
        return Ok(ast::ObjectFragmentSpec {
            r#type: S::from_str(name.name),
            selections: Vec::new(),
        }
        .into());
    };
    if registry.has_union_with_name(name.name) {
        return Ok(ast::UnionFragmentSpec {
            r#type: S::from_str(name.name),
            selections: Vec::new(),
        }
        .into());
    };
    if registry.has_interface_with_name(name.name) {
        return Ok(ast::ObjectFragmentSpec {
            r#type: S::from_str(name.name),
            selections: Vec::new(),
        }
        .into());
    };
    return Err(errors::Error::UnknownFragmentType(name.clone()));
}

fn fragment_spec_from_optype<'buffer, S: shared::ast::AsStr<'buffer>>(
    optype: &file::client::ast::OpType,
) -> Result<ast::FragmentSpec<S>, errors::Error<'buffer, S>> {
    match optype {
        file::client::ast::OpType::Query => {
            return Ok(ast::ObjectFragmentSpec {
                r#type: S::from_str("Query"),
                selections: Vec::new(),
            }
            .into());
        }
        file::client::ast::OpType::Mutation => {
            return Ok(ast::ObjectFragmentSpec {
                r#type: S::from_str("Mutation"),
                selections: Vec::new(),
            }
            .into());
        }
        file::client::ast::OpType::Subscription => {
            return Ok(ast::ObjectFragmentSpec {
                r#type: S::from_str("Subscription"),
                selections: Vec::new(),
            }
            .into());
        }
    }
}

pub fn parse_first_pass<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &mut TypeRegistry<S>,
    node: &file::client::ast::ASTNode<'buffer>,
) -> Result<(), errors::Error<'buffer, S>> {
    match node {
        file::client::ast::ASTNode::Fragment(fragment) => {
            if registry.fragments.contains_key(fragment.name.name) {
                return Err(errors::Error::FragmentNameCollision(
                    fragment.name.clone(),
                ));
            };
            registry.fragments.insert(
                S::from_str(fragment.name.name),
                ast::Fragment {
                    name: S::from_str(fragment.name.name),
                    source_text: S::from_str(
                        shared::source_text::extract_from_fragment(fragment),
                    ),
                    spec: fragment_spec_from_name(
                        server_registry,
                        &fragment.type_name,
                    )?,
                    hash: 0,
                },
            );
            Ok(())
        }
        file::client::ast::ASTNode::Operation(operation) => {
            if registry.operations.contains_key(operation.name.name) {
                return Err(errors::Error::OperationNameCollision(
                    operation.name.clone(),
                ));
            };
            let parameters = server::input::parse_field_definitions(
                server_registry,
                &operation.parameters,
            )?;
            registry.operations.insert(
                S::from_str(operation.name.name),
                ast::Operation {
                    name: S::from_str(operation.name.name),
                    source_text: S::from_str(
                        shared::source_text::extract_from_operation(operation),
                    ),
                    r#type: operation.r#type,
                    parameters_hash: hash::get_operation_parameters_hash(
                        &parameters,
                    ),
                    parameters,
                    fragment_spec: fragment_spec_from_optype(
                        &operation.r#type,
                    )?,
                    fragment_spec_hash: 0,
                    used_fragments: Vec::new(),
                },
            );
            Ok(())
        }
        file::client::ast::ASTNode::Directive(node) => {
            if registry.directives.contains_key(node.name.name) {
                return Err(errors::Error::DirectiveNameCollision(
                    node.name.clone(),
                ));
            }
            registry.directives.insert(
                S::from_str(node.name.name),
                directive::parse(server_registry, node)?,
            );
            Ok(())
        }
    }
}

pub fn parse_second_pass<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &mut TypeRegistry<S>,
    node: &file::client::ast::ASTNode<'buffer>,
) -> Result<(), errors::Error<'buffer, S>> {
    match node {
        file::client::ast::ASTNode::Fragment(fragment) => {
            fragment::parse(server_registry, registry, fragment)
        }
        file::client::ast::ASTNode::Operation(operation) => {
            operation::parse(server_registry, registry, operation)
        }
        file::client::ast::ASTNode::Directive(_) => return Ok(()),
    }
}
