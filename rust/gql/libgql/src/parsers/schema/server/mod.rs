pub mod arguments;
pub mod ast;
pub mod directive;
pub mod errors;
pub mod input;
pub mod interface;
pub mod object;
pub mod union;
use std::{cell::RefCell, rc::Rc};

pub use errors::Error;
use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{shared, type_registry::TypeRegistry},
};

fn parse_server_node_first_pass(
    ast_node: &file::server::ast::TypeDefinitionNode,
) -> ast::ServerSchemaNode {
    match ast_node {
        file::server::ast::TypeDefinitionNode::Enum(e) => {
            Rc::new(RefCell::new(shared::ast::Enum {
                name: e.name.name.clone(),
                values: e.values.iter().map(|v| v.value.name.clone()).collect(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Scalar(s) => {
            Rc::new(RefCell::new(shared::ast::Scalar {
                name: s.name.name.clone(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Input(i) => {
            Rc::new(RefCell::new(shared::ast::InputType {
                name: i.name.name.clone(),
                fields: IndexMap::new(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Object(o) => {
            Rc::new(RefCell::new(ast::ObjectType {
                name: o.name.name.clone(),
                fields: IndexMap::new(),
                implements: IndexMap::new(),
                directives: Vec::new()
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Interface(i) => {
            Rc::new(RefCell::new(ast::Interface {
                name: i.name.name.clone(),
                fields: IndexMap::new(),
                directives: Vec::new()
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Union(u) => {
            Rc::new(RefCell::new(ast::Union {
                name: u.name.name.clone(),
                items: IndexMap::new(),
            }))
            .into()
        }
        file::server::ast::TypeDefinitionNode::Directive(d) => {
            Rc::new(RefCell::new(shared::ast::ServerDirective {
                name: d.name.name.clone(),
                arguments: IndexMap::new(),
                locations: Vec::new(),
            }))
            .into()
        }
    }
}

fn parse_server_node_second_pass(
    ast_node: &file::server::ast::TypeDefinitionNode,
    registry: &mut TypeRegistry,
) -> Result<ast::ServerSchemaNode, Error> {
    match ast_node {
        file::server::ast::TypeDefinitionNode::Enum(e) => {
            Ok(registry.enums.get(&e.name.name).unwrap().clone().into())
        }
        file::server::ast::TypeDefinitionNode::Scalar(s) => {
            Ok(registry.enums.get(&s.name.name).unwrap().clone().into())
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
