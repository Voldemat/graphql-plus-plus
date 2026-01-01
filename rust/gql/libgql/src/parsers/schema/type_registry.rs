use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parsers::{
    file,
    schema::{client, server, shared},
};

pub type FieldMapping = indexmap::IndexMap<
    String,
    Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
>;

pub struct TypeRegistry {
    pub server_directives:
        HashMap<String, Rc<RefCell<shared::ast::ServerDirective>>>,
    pub client_directives:
        HashMap<String, Rc<RefCell<client::ast::ClientDirective>>>,
    pub queries: FieldMapping,
    pub mutations: FieldMapping,
    pub subscriptions: FieldMapping,
    pub objects: HashMap<String, Rc<RefCell<server::ast::ObjectType>>>,
    pub inputs: HashMap<String, Rc<RefCell<shared::ast::InputType>>>,
    pub interfaces: HashMap<String, Rc<RefCell<server::ast::Interface>>>,
    pub scalars: HashMap<String, Rc<RefCell<shared::ast::Scalar>>>,
    pub enums: HashMap<String, Rc<RefCell<shared::ast::Enum>>>,
    pub unions: HashMap<String, Rc<RefCell<server::ast::Union>>>,
    pub fragments: HashMap<String, Rc<RefCell<client::ast::Fragment>>>,
    pub operations: HashMap<String, Rc<RefCell<client::ast::Operation>>>,
}

pub enum Error {
    UnknownType(file::shared::ast::NameNode),
    UnknownArgument(file::shared::ast::NameNode),
}

impl TypeRegistry {
    fn get_query_object(
        self: &Self,
    ) -> Option<&Rc<RefCell<server::ast::ObjectType>>> {
        return self.objects.get("Query");
    }

    fn get_mutation_object(
        self: &Self,
    ) -> Option<&Rc<RefCell<server::ast::ObjectType>>> {
        return self.objects.get("Mutation");
    }

    fn get_subscription_object(
        self: &Self,
    ) -> Option<&Rc<RefCell<server::ast::ObjectType>>> {
        return self.objects.get("Subscription");
    }

    pub fn get_type_for_input(
        self: &Self,
        node: &file::shared::ast::NameNode,
    ) -> Result<shared::ast::InputTypeSpec, Error> {
        let name = &node.name;
        if let Some(input) = self.inputs.get(name) {
            return Ok(input.clone().into());
        }
        if let Some(scalar) = self.scalars.get(name) {
            return Ok(scalar.clone().into());
        }
        if let Some(gqlenum) = self.enums.get(name) {
            return Ok(gqlenum.clone().into());
        }
        return Err(Error::UnknownType(node.clone()));
    }

    pub fn get_type_for_object(
        self: &Self,
        node: &file::shared::ast::NameNode,
    ) -> Result<server::ast::ObjectTypeSpec, Error> {
        let name = &node.name;
        if let Some(object) = self.objects.get(name) {
            return Ok(object.clone().into());
        }
        if let Some(interface) = self.interfaces.get(name) {
            return Ok(interface.clone().into());
        }
        if let Some(union) = self.unions.get(name) {
            return Ok(union.clone().into());
        }
        if let Some(scalar) = self.scalars.get(name) {
            return Ok(scalar.clone().into());
        }
        if let Some(gqlenum) = self.enums.get(name) {
            return Ok(gqlenum.clone().into());
        }
        return Err(Error::UnknownType(node.clone()));
    }

    fn get_mapping_for_op(
        self: &mut Self,
        optype: file::client::ast::OpType,
    ) -> &mut FieldMapping {
        match optype {
            file::client::ast::OpType::Query => &mut self.queries,
            file::client::ast::OpType::Mutation => &mut self.mutations,
            file::client::ast::OpType::Subscription => &mut self.subscriptions,
        }
    }

    fn add_op_if_not_exists(
        self: &mut Self,
        field: &Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
        optype: file::client::ast::OpType,
    ) -> Result<(), ()> {
        let mapping = self.get_mapping_for_op(optype);
        if mapping.contains_key(&field.name) {
            return Err(());
        }
        mapping.insert(field.name.clone(), field.clone());
        return Ok(());
    }

    fn add_node(self: &mut Self, schema_node: server::ast::ServerSchemaNode) {
        match schema_node {
            server::ast::ServerSchemaNode::ObjectType(object_rc) => {
                self.append_ops_if_special_object(
                    &object_rc.borrow().name,
                    &object_rc.borrow().fields,
                );
                let name = object_rc.borrow().name.clone();
                self.objects.insert(name, object_rc);
            }
            server::ast::ServerSchemaNode::Interface(interface) => {
                let name = interface.borrow().name.clone();
                self.interfaces.insert(name, interface);
            }
            server::ast::ServerSchemaNode::Union(union) => {
                let name = union.borrow().name.clone();
                self.unions.insert(name, union);
            }
            server::ast::ServerSchemaNode::InputType(input) => {
                let name = input.borrow().name.clone();
                self.inputs.insert(name, input);
            }
            server::ast::ServerSchemaNode::Enum(gqlenum) => {
                let name = gqlenum.borrow().name.clone();
                self.enums.insert(name, gqlenum);
            }
            server::ast::ServerSchemaNode::ServerDirective(directive) => {
                let name = directive.borrow().name.clone();
                self.server_directives.insert(name, directive);
            }
            server::ast::ServerSchemaNode::Scalar(scalar) => {
                let name = scalar.borrow().name.clone();
                self.scalars.insert(name, scalar);
            }
        }
    }

    fn append_ops_if_special_object(
        self: &mut Self,
        obj_name: &str,
        new_fields: &FieldMapping,
    ) {
        let Some(optype) =
            file::client::ast::OpType::from_object_name(obj_name)
        else {
            return;
        };
        for (_, field) in new_fields {
            let _ = self.add_op_if_not_exists(field, optype);
        }
    }

    fn patch_object<'a>(
        self: &mut Self,
        object_type: Rc<RefCell<server::ast::ObjectType>>,
        new_fields: &'a FieldMapping,
    ) -> Result<(), &'a str> {
        for (name, new_field) in new_fields {
            if object_type.borrow().fields.contains_key(name) {
                return Err(name);
            }
            object_type
                .borrow_mut()
                .fields
                .insert(name.clone(), new_field.clone());
        }
        self.append_ops_if_special_object(
            &object_type.borrow().name,
            new_fields,
        );
        return Ok(());
    }

    fn fragment_spec_from_op_type(
        self: &Self,
        optype: file::client::ast::OpType,
    ) -> client::ast::FragmentSpec {
        match optype {
            file::client::ast::OpType::Query => {
                client::ast::FragmentSpec::Object(
                    client::ast::ObjectFragmentSpec {
                        r#type: self.get_query_object().unwrap().clone(),
                        selections: Vec::new(),
                    },
                )
            }
            file::client::ast::OpType::Mutation => {
                client::ast::FragmentSpec::Object(
                    client::ast::ObjectFragmentSpec {
                        r#type: self.get_mutation_object().unwrap().clone(),
                        selections: Vec::new(),
                    },
                )
            }
            file::client::ast::OpType::Subscription => {
                client::ast::FragmentSpec::Object(
                    client::ast::ObjectFragmentSpec {
                        r#type: self.get_subscription_object().unwrap().clone(),
                        selections: Vec::new(),
                    },
                )
            }
        }
    }
}
