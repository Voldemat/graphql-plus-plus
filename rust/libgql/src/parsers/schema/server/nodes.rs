use indexmap::{IndexMap, IndexSet};

use crate::parsers::{
    file,
    schema::{
        server::{ast, directive, errors, input, interface, object, union},
        shared,
    },
};

use super::type_registry::{self, HashMapTypeRegistry};

pub fn parse_server_node_first_pass<'buffer>(
    registry: &mut HashMapTypeRegistry,
    ast_node: &file::server::ast::TypeDefinitionNode<'buffer>,
) -> Result<(), type_registry::Error<'buffer>> {
    match ast_node {
        file::server::ast::TypeDefinitionNode::Enum(e) => {
            if registry.enums.contains_key(e.name.name) {
                return Err(type_registry::Error::EnumNameCollision(
                    e.name.clone(),
                ));
            };
            registry.enums.insert(
                e.name.name.to_string(),
                shared::ast::Enum {
                    name: e.name.name.to_string(),
                    values: e
                        .values
                        .iter()
                        .map(|v| v.value.name.to_string())
                        .collect(),
                },
            );
            Ok(())
        }
        file::server::ast::TypeDefinitionNode::Scalar(s) => {
            if registry.scalars.contains(s.name.name) {
                return Err(type_registry::Error::ScalarNameCollision(
                    s.name.clone(),
                ));
            }
            registry.scalars.insert(s.name.name.to_string());
            Ok(())
        }
        file::server::ast::TypeDefinitionNode::Input(i) => {
            if registry.inputs.contains_key(i.name.name) {
                return Err(type_registry::Error::InputNameCollision(
                    i.name.clone(),
                ));
            }
            registry.inputs.insert(
                i.name.name.to_string(),
                shared::ast::InputType {
                    name: i.name.name.to_string(),
                    fields: IndexMap::new(),
                },
            );
            Ok(())
        }
        file::server::ast::TypeDefinitionNode::Object(o) => {
            if registry.objects.contains_key(o.name.name) {
                return Err(type_registry::Error::ObjectNameCollision(
                    o.name.clone(),
                ));
            }
            registry.objects.insert(
                o.name.name.to_string(),
                ast::ObjectType {
                    name: o.name.name.to_string(),
                    fields: IndexMap::new(),
                    implements: IndexSet::new(),
                    directives: Vec::new(),
                },
            );
            Ok(())
        }
        file::server::ast::TypeDefinitionNode::Interface(i) => {
            if registry.interfaces.contains_key(i.name.name) {
                return Err(type_registry::Error::InterfaceNameCollision(
                    i.name.clone(),
                ));
            }
            registry.interfaces.insert(
                i.name.name.to_string(),
                ast::Interface {
                    name: i.name.name.to_string(),
                    fields: IndexMap::new(),
                    directives: Vec::new(),
                },
            );
            Ok(())
        }
        file::server::ast::TypeDefinitionNode::Union(u) => {
            if registry.unions.contains_key(u.name.name) {
                return Err(type_registry::Error::UnionNameCollision(
                    u.name.clone(),
                ));
            }
            registry.unions.insert(
                u.name.name.to_string(),
                ast::Union {
                    name: u.name.name.to_string(),
                    items: IndexSet::new(),
                },
            );
            Ok(())
        }
        file::server::ast::TypeDefinitionNode::Directive(d) => {
            if registry.directives.contains_key(d.name.name) {
                return Err(type_registry::Error::DirectiveNameCollision(
                    d.name.clone(),
                ));
            }
            registry.directives.insert(
                d.name.name.to_string(),
                shared::ast::ServerDirective {
                    name: d.name.name.to_string(),
                    arguments: Default::default(),
                    locations: Vec::new(),
                },
            );
            Ok(())
        }
    }
}

pub fn parse_server_node_second_pass<'buffer>(
    ast_node: &file::server::ast::TypeDefinitionNode<'buffer>,
    registry: &mut HashMapTypeRegistry,
) -> Result<(), errors::Error<'buffer>> {
    match ast_node {
        file::server::ast::TypeDefinitionNode::Enum(_) => Ok(()),
        file::server::ast::TypeDefinitionNode::Scalar(_) => Ok(()),
        file::server::ast::TypeDefinitionNode::Input(i) => {
            input::parse_definition(registry, i)
        }
        file::server::ast::TypeDefinitionNode::Object(o) => {
            object::parse_definition(o, registry)
        }
        file::server::ast::TypeDefinitionNode::Interface(i) => {
            interface::parse_definition(i, registry)
        }
        file::server::ast::TypeDefinitionNode::Union(u) => {
            union::parse_definition(u, registry)
        }
        file::server::ast::TypeDefinitionNode::Directive(d) => {
            directive::parse_definition(d, registry)
        }
    }
}

pub fn parse_server_extend_node<'buffer>(
    ast_node: &file::server::ast::ExtendTypeNode<'buffer>,
    registry: &mut HashMapTypeRegistry,
) -> Result<
    (
        &'buffer str,
        IndexMap<String, shared::ast::FieldDefinition<ast::ObjectFieldSpec>>,
    ),
    errors::Error<'buffer>,
> {
    if let None = registry.objects.get(ast_node.type_node.name.name) {
        return Err(errors::Error::UnknownObject(
            ast_node.type_node.name.clone(),
        ));
    };
    return Ok((
        ast_node.type_node.name.name,
        object::parse_fields(&ast_node.type_node.fields, registry)?,
    ));
}
