use std::collections::HashSet;

use crate::parsers::schema::{client::ast, server};

use super::{type_registry::TypeRegistry, visitor};

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

impl ServerUsesMap {
    pub fn new(
        server_registry: &server::type_registry::HashMapTypeRegistry,
        client_registry: &TypeRegistry,
    ) -> ServerUsesMap {
        let mut m = ServerUsesMap::default();
        {
            let mut hooks = visitor::ASTVisitorHooks {
                visit_object_type: Some(Box::new(|object_type| {
                    m.objects.insert(object_type.name.clone());
                })),
                visit_object_fragment_spec: Some(Box::new(|fragment_spec| {
                    let name = &fragment_spec.r#type;
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
                })),
                visit_interface: Some(Box::new(|interface| {
                    m.interfaces.insert(interface.name.clone());
                })),
                visit_input_type: Some(Box::new(|input| {
                    m.inputs.insert(input.name.clone());
                })),
                visit_scalar: Some(Box::new(|scalar| {
                    m.scalars.insert(scalar.clone());
                })),
                visit_enum: Some(Box::new(|e| {
                    m.enums.insert(e.name.clone());
                })),
                visit_union: Some(Box::new(|union| {
                    m.unions.insert(union.name.clone());
                })),
                ..visitor::ASTVisitorHooks::default()
            };
            visitor::visit_client_schema(
                server_registry,
                client_registry,
                &mut hooks,
            );
        }
        m
    }
}
