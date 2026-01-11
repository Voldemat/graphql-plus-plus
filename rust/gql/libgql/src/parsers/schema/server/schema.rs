use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::parsers::schema::{server::ast, shared};

#[derive(Debug, Default)]
pub struct Schema {
    pub objects: IndexMap<String, Rc<RefCell<ast::ObjectType>>>,
    pub inputs: IndexMap<String, Rc<RefCell<shared::ast::InputType>>>,
    pub interfaces: IndexMap<String, Rc<RefCell<ast::Interface>>>,
    pub scalars: Vec<String>,
    pub enums: IndexMap<String, Rc<shared::ast::Enum>>,
    pub unions: IndexMap<String, Rc<RefCell<ast::Union>>>,
    pub directives: IndexMap<String, Rc<RefCell<shared::ast::ServerDirective>>>,
}

impl Schema {
    pub fn from_nodes(nodes: &[ast::ServerSchemaNode]) -> Self {
        let mut schema = Schema::default();
        for node in nodes {
            schema.add_node(node);
        }
        return schema;
    }

    fn add_node(self: &mut Self, s_node: &ast::ServerSchemaNode) {
        match s_node {
            ast::ServerSchemaNode::Enum(node) => {
                self.enums.insert(node.name.clone(), node.clone());
            }
            ast::ServerSchemaNode::ObjectType(node) => {
                self.objects
                    .insert(node.borrow().name.clone(), node.clone());
            }
            ast::ServerSchemaNode::InputType(node) => {
                self.inputs.insert(node.borrow().name.clone(), node.clone());
            }
            ast::ServerSchemaNode::Union(node) => {
                self.unions.insert(node.borrow().name.clone(), node.clone());
            }
            ast::ServerSchemaNode::Scalar(node) => {
                self.scalars.push(node.clone());
            }
            ast::ServerSchemaNode::ServerDirective(node) => {
                self.directives
                    .insert(node.borrow().name.clone(), node.clone());
            }
            ast::ServerSchemaNode::Interface(node) => {
                self.interfaces
                    .insert(node.borrow().name.clone(), node.clone());
            }
        }
    }

    pub fn append_schema(self: &mut Self, mut new_schema: Self) {
        self.objects.append(&mut new_schema.objects);
        self.scalars.append(&mut new_schema.scalars);
        self.inputs.append(&mut new_schema.inputs);
        self.enums.append(&mut new_schema.enums);
        self.unions.append(&mut new_schema.unions);
        self.interfaces.append(&mut new_schema.interfaces);
        self.directives.append(&mut new_schema.directives);
    }
}
