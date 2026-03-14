use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{
        client::{ast, directive, errors, fragment, hash, operation},
        server, shared,
        type_registry::TypeRegistry,
    },
};

fn fragment_spec_from_name(
    registry: &TypeRegistry,
    name: &file::shared::ast::NameNode,
) -> Result<ast::FragmentSpec, errors::Error> {
    if let Some(object) = registry.objects.get(&name.name) {
        return Ok(ast::ObjectFragmentSpec::<server::ast::ObjectType> {
            r#type: object.clone(),
            selections: Vec::new(),
        }
        .into());
    };
    if let Some(union) = registry.unions.get(&name.name) {
        return Ok(ast::UnionFragmentSpec {
            r#type: union.clone(),
            selections: Vec::new(),
        }
        .into());
    };
    if let Some(interface) = registry.interfaces.get(&name.name) {
        return Ok(ast::ObjectFragmentSpec::<server::ast::Interface> {
            r#type: interface.clone(),
            selections: Vec::new(),
        }
        .into());
    };
    return Err(errors::Error::UnknownFragmentType(name.clone()));
}

fn fragment_spec_from_optype(
    registry: &TypeRegistry,
    optype: &file::client::ast::OpType,
) -> Result<ast::FragmentSpec, errors::Error> {
    match optype {
        file::client::ast::OpType::Query => {
            return Ok(ast::ObjectFragmentSpec::<server::ast::ObjectType> {
                r#type: registry.get_query_object().unwrap().clone(),
                selections: Vec::new(),
            }
            .into());
        }
        file::client::ast::OpType::Mutation => {
            return Ok(ast::ObjectFragmentSpec::<server::ast::ObjectType> {
                r#type: registry.get_mutation_object().unwrap().clone(),
                selections: Vec::new(),
            }
            .into());
        }
        file::client::ast::OpType::Subscription => {
            return Ok(ast::ObjectFragmentSpec::<server::ast::ObjectType> {
                r#type: registry.get_subscription_object().unwrap().clone(),
                selections: Vec::new(),
            }
            .into());
        }
    }
}

pub fn parse_first_pass(
    registry: &TypeRegistry,
    node: &file::client::ast::ASTNode,
) -> Result<ast::ClientSchemaNode, errors::Error> {
    match node {
        file::client::ast::ASTNode::Fragment(fragment) => {
            Ok(Rc::new(RefCell::new(ast::Fragment {
                name: fragment.name.name.clone(),
                source_text: shared::source_text::extract_from_fragment(
                    fragment,
                ),
                spec: fragment_spec_from_name(registry, &fragment.type_name)?,
                hash: 0,
            }))
            .into())
        }
        file::client::ast::ASTNode::Operation(operation) => {
            let parameters = shared::input::parse_field_definitions(
                &operation.parameters,
                registry,
            )?;
            Ok(Rc::new(RefCell::new(ast::Operation {
                name: operation.name.name.clone(),
                source_text: shared::source_text::extract_from_operation(
                    operation,
                ),
                r#type: operation.r#type,
                parameters_hash: hash::get_operation_parameters_hash(
                    &parameters,
                ),
                parameters,
                fragment_spec: fragment_spec_from_optype(
                    registry,
                    &operation.r#type,
                )?,
                fragment_spec_hash: 0,
                used_fragments: Vec::new(),
            }))
            .into())
        }
        file::client::ast::ASTNode::Directive(node) => {
            Ok(directive::parse(registry, node)?.into())
        }
    }
}

pub fn parse_second_pass(
    registry: &mut TypeRegistry,
    node: &file::client::ast::ASTNode,
) -> Result<(), errors::Error> {
    match node {
        file::client::ast::ASTNode::Fragment(fragment) => {
            fragment::parse(registry, fragment)
        }
        file::client::ast::ASTNode::Operation(operation) => {
            operation::parse(registry, operation)
        }
        file::client::ast::ASTNode::Directive(_) => return Ok(()),
    }
}
