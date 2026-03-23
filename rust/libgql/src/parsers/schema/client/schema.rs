use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use indexmap::IndexMap;

use crate::parsers::schema::{client::ast, visitor};

#[derive(Debug, Default)]
pub struct ClientSchema {
    pub fragments: IndexMap<String, Arc<RwLock<ast::Fragment>>>,
    pub operations: IndexMap<String, Arc<RwLock<ast::Operation>>>,
    pub directives: IndexMap<String, Arc<ast::ClientDirective>>,
}

#[derive(Default)]
pub struct ServerUsesMap {
    pub objects: HashSet<String>,
    pub inputs: HashSet<String>,
    pub scalars: HashSet<String>,
    pub enums: HashSet<String>,
    pub unions: HashSet<String>,
    pub interfaces: HashSet<String>,
    pub directives: HashSet<String>,
    pub queries: HashSet<String>,
    pub mutations: HashSet<String>,
    pub subscriptions: HashSet<String>,
}

impl ClientSchema {
    pub fn from_nodes(nodes: &[ast::ClientSchemaNode]) -> Self {
        let mut schema = ClientSchema::default();
        for node in nodes.iter() {
            schema.add_node(node);
        }
        return schema;
    }

    fn add_node(self: &mut Self, s_node: &ast::ClientSchemaNode) {
        match s_node {
            ast::ClientSchemaNode::Fragment(f) => {
                self.fragments
                    .insert(f.read().unwrap().name.clone(), f.clone());
            }
            ast::ClientSchemaNode::Operation(o) => {
                self.operations
                    .insert(o.read().unwrap().name.clone(), o.clone());
            }
            ast::ClientSchemaNode::ClientDirective(d) => {
                self.directives.insert(d.name.clone(), d.clone());
            }
        }
    }

    pub fn append_schema(self: &mut Self, new_schema: Self) {
        self.fragments.extend(new_schema.fragments.into_iter());
        self.operations.extend(new_schema.operations.into_iter());
        self.directives.extend(new_schema.directives.into_iter());
    }

    fn populate_server_uses_map(self: &Self, m: &mut ServerUsesMap) {
        let mut hooks = visitor::ASTVisitorHooks {
            visit_object_type: Some(Box::new(|object_type| {
                m.objects.insert(object_type.read().unwrap().name.clone());
            })),
            visit_object_fragment_spec_object_type: Some(Box::new(
                |fragment_spec| {
                    let name = &fragment_spec.r#type.read().unwrap().name;
                    let fields = fragment_spec.selections.iter().filter_map(
                        |s| match s {
                            ast::ObjectSelection::FieldSelection(f) => {
                                Some(f.name.clone())
                            }
                            _ => None,
                        },
                    );
                    if name == "Query" {
                        m.queries.extend(fields);
                    } else if name == "Mutation" {
                        m.mutations.extend(fields);
                    } else if name == "Subscription" {
                        m.subscriptions.extend(fields);
                    }
                },
            )),
            visit_interface: Some(Box::new(|interface| {
                m.interfaces.insert(interface.read().unwrap().name.clone());
            })),
            visit_input_type: Some(Box::new(|input| {
                m.inputs.insert(input.read().unwrap().name.clone());
            })),
            visit_scalar: Some(Box::new(|scalar| {
                m.scalars.insert(scalar.clone());
            })),
            visit_enum: Some(Box::new(|e| {
                m.enums.insert(e.name.clone());
            })),
            visit_union: Some(Box::new(|union| {
                m.unions.insert(union.read().unwrap().name.clone());
            })),
            ..visitor::ASTVisitorHooks::default()
        };
        visitor::visit_client_schema(&mut hooks, self);
    }

    pub fn get_server_uses_map(self: &Self) -> ServerUsesMap {
        let mut map = ServerUsesMap::default();
        self.populate_server_uses_map(&mut map);
        return map;
    }
}
