use crate::parsers::{
    file,
    schema::{
        client::{ast, directive, errors, fragment, hash, operation},
        server, shared,
    },
};

use super::type_registry::TypeRegistry;

fn fragment_spec_from_name<'buffer, T: server::type_registry::TypeRegistry>(
    registry: &T,
    name: &file::shared::ast::NameNode<'buffer>,
) -> Result<ast::FragmentSpec, errors::Error<'buffer>> {
    if registry.has_object_with_name(name.name) {
        return Ok(ast::ObjectFragmentSpec {
            r#type: name.name.to_string(),
            selections: Vec::new(),
        }
        .into());
    };
    if registry.has_union_with_name(name.name) {
        return Ok(ast::UnionFragmentSpec {
            r#type: name.name.to_string(),
            selections: Vec::new(),
        }
        .into());
    };
    if registry.has_interface_with_name(name.name) {
        return Ok(ast::ObjectFragmentSpec {
            r#type: name.name.to_string(),
            selections: Vec::new(),
        }
        .into());
    };
    return Err(errors::Error::UnknownFragmentType(name.clone()));
}

fn fragment_spec_from_optype<'buffer>(
    optype: &file::client::ast::OpType,
) -> Result<ast::FragmentSpec, errors::Error<'buffer>> {
    match optype {
        file::client::ast::OpType::Query => {
            return Ok(ast::ObjectFragmentSpec {
                r#type: "Query".to_string(),
                selections: Vec::new(),
            }
            .into());
        }
        file::client::ast::OpType::Mutation => {
            return Ok(ast::ObjectFragmentSpec {
                r#type: "Mutation".to_string(),
                selections: Vec::new(),
            }
            .into());
        }
        file::client::ast::OpType::Subscription => {
            return Ok(ast::ObjectFragmentSpec {
                r#type: "Subscription".to_string(),
                selections: Vec::new(),
            }
            .into());
        }
    }
}

pub fn parse_first_pass<'buffer, T: server::type_registry::TypeRegistry>(
    server_registry: &T,
    registry: &mut TypeRegistry,
    node: &file::client::ast::ASTNode<'buffer>,
) -> Result<(), errors::Error<'buffer>> {
    match node {
        file::client::ast::ASTNode::Fragment(fragment) => {
            if registry.fragments.contains_key(fragment.name.name) {
                return Err(errors::Error::FragmentNameCollision(
                    fragment.name.clone(),
                ));
            };
            registry.fragments.insert(
                fragment.name.name.to_string(),
                ast::Fragment {
                    name: fragment.name.name.to_string(),
                    source_text: shared::source_text::extract_from_fragment(
                        fragment,
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
                operation.name.name.to_string(),
                ast::Operation {
                    name: operation.name.name.to_string(),
                    source_text: shared::source_text::extract_from_operation(
                        operation,
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
                node.name.name.to_string(),
                directive::parse(server_registry, node)?,
            );
            Ok(())
        }
    }
}

pub fn parse_second_pass<'buffer, T: server::type_registry::TypeRegistry>(
    server_registry: &T,
    registry: &mut TypeRegistry,
    node: &file::client::ast::ASTNode<'buffer>,
) -> Result<(), errors::Error<'buffer>> {
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
