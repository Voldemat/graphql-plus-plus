use std::sync::{Arc, RwLock};

use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{
        server::{ast, directive, errors, input, interface, object, union},
        shared,
        type_registry::TypeRegistry,
    },
};

pub fn parse_server_node_first_pass<'buffer>(
    ast_node: &file::server::ast::TypeDefinitionNode<'buffer>,
) -> ast::ServerSchemaNode {
    match ast_node {
        file::server::ast::TypeDefinitionNode::Enum(e) => {
            Arc::new(shared::ast::Enum {
                name: e.name.name.to_string(),
                values: e
                    .values
                    .iter()
                    .map(|v| v.value.name.to_string())
                    .collect(),
            })
            .into()
        }
        file::server::ast::TypeDefinitionNode::Scalar(s) => {
            s.name.name.to_string().into()
        }
        file::server::ast::TypeDefinitionNode::Input(i) => {
            Arc::new(RwLock::new(shared::ast::InputType {
                name: i.name.name.to_string(),
                fields: IndexMap::new(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Object(o) => {
            Arc::new(RwLock::new(ast::ObjectType {
                name: o.name.name.to_string(),
                fields: IndexMap::new(),
                implements: IndexMap::new(),
                directives: Vec::new(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Interface(i) => {
            Arc::new(RwLock::new(ast::Interface {
                name: i.name.name.to_string(),
                fields: IndexMap::new(),
                directives: Vec::new(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Union(u) => {
            Arc::new(RwLock::new(ast::Union {
                name: u.name.name.to_string(),
                items: IndexMap::new(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Directive(d) => {
            Arc::new(RwLock::new(shared::ast::ServerDirective {
                name: d.name.name.to_string(),
                arguments: IndexMap::new(),
                locations: Vec::new(),
            }))
            .into()
        }
    }
}

pub fn parse_server_node_second_pass<'buffer>(
    ast_node: &file::server::ast::TypeDefinitionNode<'buffer>,
    registry: &mut TypeRegistry,
) -> Result<ast::ServerSchemaNode, errors::Error<'buffer>> {
    match ast_node {
        file::server::ast::TypeDefinitionNode::Enum(e) => {
            Ok(registry.enums.get(e.name.name).unwrap().clone().into())
        }
        file::server::ast::TypeDefinitionNode::Scalar(s) => {
            Ok(s.name.name.to_string().into())
        }
        file::server::ast::TypeDefinitionNode::Input(i) => {
            input::parse_definition(i, registry).map(|v| v.into())
        }
        file::server::ast::TypeDefinitionNode::Object(o) => {
            object::parse_definition(o, registry).map(|v| v.into())
        }
        file::server::ast::TypeDefinitionNode::Interface(i) => {
            interface::parse_definition(i, registry).map(|v| v.into())
        }
        file::server::ast::TypeDefinitionNode::Union(u) => {
            union::parse_definition(u, registry).map(|v| v.into())
        }
        file::server::ast::TypeDefinitionNode::Directive(d) => {
            directive::parse_definition(d, registry).map(|v| v.into())
        }
    }
}

pub fn parse_server_extend_node<'buffer>(
    ast_node: &file::server::ast::ExtendTypeNode<'buffer>,
    registry: &mut TypeRegistry,
) -> Result<
    (
        Arc<RwLock<ast::ObjectType>>,
        IndexMap<
            String,
            Arc<shared::ast::FieldDefinition<ast::ObjectFieldSpec>>,
        >,
    ),
    errors::Error<'buffer>,
> {
    let Some(object) = registry.objects.get(ast_node.type_node.name.name)
    else {
        return Err(errors::Error::UnknownObject(
            ast_node.type_node.name.clone(),
        ));
    };
    return Ok((
        object.clone(),
        object::parse_fields(&ast_node.type_node.fields, registry)?,
    ));
}
