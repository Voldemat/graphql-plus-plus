use std::collections::HashSet;

use indexmap::{IndexMap, IndexSet};

use crate::parsers::{
    file,
    schema::{server, shared},
};

#[derive(Debug)]
pub struct TypeRegistry {
    pub directives: IndexMap<String, shared::ast::ServerDirective>,
    pub queries: HashSet<String>,
    pub mutations: HashSet<String>,
    pub subscriptions: HashSet<String>,
    pub objects: IndexMap<String, server::ast::ObjectType>,
    pub inputs: IndexMap<String, shared::ast::InputType>,
    pub interfaces: IndexMap<String, server::ast::Interface>,
    pub scalars: IndexSet<String>,
    pub enums: IndexMap<String, shared::ast::Enum>,
    pub unions: IndexMap<String, server::ast::Union>,
}

#[derive(Debug)]
pub enum Error<'buffer> {
    UnknownType(file::shared::ast::NameNode<'buffer>),
    UnknownArgument(file::shared::ast::NameNode<'buffer>),
    EnumNameCollision(file::shared::ast::NameNode<'buffer>),
    ObjectNameCollision(file::shared::ast::NameNode<'buffer>),
    InterfaceNameCollision(file::shared::ast::NameNode<'buffer>),
    UnionNameCollision(file::shared::ast::NameNode<'buffer>),
    ScalarNameCollision(file::shared::ast::NameNode<'buffer>),
    InputNameCollision(file::shared::ast::NameNode<'buffer>),
    DirectiveNameCollision(file::shared::ast::NameNode<'buffer>),
}

impl<'buffer> Error<'buffer> {
    pub fn get_location(
        self: &Self,
    ) -> &file::shared::ast::NodeLocation<'buffer> {
        match self {
            Self::UnknownType(name_node) => &name_node.location,
            Self::UnknownArgument(name_node) => &name_node.location,
            Self::EnumNameCollision(node) => &node.location,
            Self::ObjectNameCollision(node) => &node.location,
            Self::InterfaceNameCollision(node) => &node.location,
            Self::UnionNameCollision(node) => &node.location,
            Self::ScalarNameCollision(node) => &node.location,
            Self::InputNameCollision(node) => &node.location,
            Self::DirectiveNameCollision(node) => &node.location,
        }
    }
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            directives: Default::default(),
            queries: Default::default(),
            mutations: Default::default(),
            subscriptions: Default::default(),
            objects: Default::default(),
            inputs: Default::default(),
            interfaces: Default::default(),
            scalars: super::scalars::get_builtin_scalars(),
            enums: Default::default(),
            unions: Default::default(),
        }
    }

    pub fn get_query_object(self: &Self) -> Option<&server::ast::ObjectType> {
        return self.objects.get("Query");
    }

    pub fn get_mutation_object(
        self: &Self,
    ) -> Option<&server::ast::ObjectType> {
        return self.objects.get("Mutation");
    }

    pub fn get_subscription_object(
        self: &Self,
    ) -> Option<&server::ast::ObjectType> {
        return self.objects.get("Subscription");
    }

    pub fn get_type_for_input<'buffer>(
        self: &Self,
        node: &file::shared::ast::NameNode<'buffer>,
    ) -> Result<shared::ast::InputTypeSpec, Error<'buffer>> {
        let name = node.name.to_string();
        if self.inputs.contains_key(&name) {
            return Ok(shared::ast::InputTypeSpec::InputType(name));
        }
        if self.scalars.contains(&name) {
            return Ok(shared::ast::InputTypeSpec::Scalar(name));
        }
        if self.enums.contains_key(&name) {
            return Ok(shared::ast::InputTypeSpec::Enum(name));
        }
        return Err(Error::UnknownType(node.clone()));
    }

    pub fn get_type_for_object<'buffer>(
        self: &Self,
        node: &file::shared::ast::NameNode<'buffer>,
    ) -> Result<server::ast::ObjectTypeSpec, Error<'buffer>> {
        let name = node.name.to_string();
        if self.objects.contains_key(&name) {
            return Ok(server::ast::ObjectTypeSpec::ObjectType(name));
        }
        if self.interfaces.contains_key(&name) {
            return Ok(server::ast::ObjectTypeSpec::Interface(name));
        }
        if self.unions.contains_key(&name) {
            return Ok(server::ast::ObjectTypeSpec::Union(name));
        }
        if self.scalars.contains(&name) {
            return Ok(server::ast::ObjectTypeSpec::Scalar(name));
        }
        if self.enums.contains_key(&name) {
            return Ok(server::ast::ObjectTypeSpec::Enum(name));
        }
        return Err(Error::UnknownType(node.clone()));
    }

    fn get_hashset_for_op(
        self: &mut Self,
        optype: file::client::ast::OpType,
    ) -> &mut HashSet<String> {
        match optype {
            file::client::ast::OpType::Query => &mut self.queries,
            file::client::ast::OpType::Mutation => &mut self.mutations,
            file::client::ast::OpType::Subscription => &mut self.subscriptions,
        }
    }

    fn add_op_if_not_exists(
        self: &mut Self,
        field_name: &str,
        optype: file::client::ast::OpType,
    ) -> Result<(), ()> {
        let hashset = self.get_hashset_for_op(optype);
        if hashset.contains(field_name) {
            return Err(());
        }
        hashset.insert(field_name.to_string());
        return Ok(());
    }

    pub fn patch_object(
        self: &mut Self,
        object_name: &str,
        new_fields: IndexMap<
            String,
            shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
        >,
    ) {
        let optype_option =
            file::client::ast::OpType::from_object_name(object_name);
        for name in new_fields.keys() {
            if let Some(op_type) = optype_option {
                let _ = self.add_op_if_not_exists(name, op_type);
            }
        }
        let object = self.objects.get_mut(object_name).unwrap();
        for (name, new_field) in new_fields {
            object.fields.insert(name, new_field);
        }
    }
}
