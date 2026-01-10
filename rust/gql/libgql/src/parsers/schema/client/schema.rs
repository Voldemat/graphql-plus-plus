use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::parsers::schema::{client::ast, visitor};

#[derive(Debug, Default)]
pub struct ClientSchema {
    pub fragments: HashMap<String, Rc<RefCell<ast::Fragment>>>,
    pub operations: HashMap<String, Rc<RefCell<ast::Operation>>>,
    pub directives: HashMap<String, Rc<ast::ClientDirective>>,
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
                self.fragments.insert(f.borrow().name.clone(), f.clone());
            }
            ast::ClientSchemaNode::Operation(o) => {
                self.operations.insert(o.borrow().name.clone(), o.clone());
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
                m.objects.insert(object_type.borrow().name.clone());
            })),
            visit_object_fragment_spec_object_type: Some(Box::new(
                |fragment_spec| {
                    let name = &fragment_spec.r#type.borrow().name;
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
                m.interfaces.insert(interface.borrow().name.clone());
            })),
            visit_input_type: Some(Box::new(|input| {
                m.inputs.insert(input.borrow().name.clone());
            })),
            visit_scalar: Some(Box::new(|scalar| {
                m.scalars.insert(scalar.clone());
            })),
            visit_enum: Some(Box::new(|e| {
                m.enums.insert(e.borrow().name.clone());
            })),
            visit_union: Some(Box::new(|union| {
                m.unions.insert(union.borrow().name.clone());
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
