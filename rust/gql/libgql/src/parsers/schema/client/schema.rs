use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parsers::schema::client::ast;

#[derive(Debug, Default)]
pub struct ClientSchema {
    pub fragments: HashMap<String, Rc<RefCell<ast::Fragment>>>,
    pub operations: HashMap<String, Rc<RefCell<ast::Operation>>>,
    pub directives: HashMap<String, Rc<ast::ClientDirective>>,
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

    fn append_schema(self: &mut Self, new_schema: Self) {
        self.fragments.extend(new_schema.fragments.into_iter());
        self.operations.extend(new_schema.operations.into_iter());
        self.directives.extend(new_schema.directives.into_iter());
    }
}
