use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{server, shared},
};

pub struct SpreadSelection {
    pub fragment: Rc<Fragment>,
}

pub struct TypenameField {
    pub alias: Option<String>,
}

#[derive(derive_more::From)]
pub enum UnionSelection {
    TypenameField(TypenameField),
    SpreadSelection(SpreadSelection),
    ObjectConditionalSpreadSelection(ObjectConditionalSpreadSelection),
    UnionConditionalSpreadSelection(UnionConditionalSpreadSelection),
}

#[derive(derive_more::From)]
pub enum ObjectSelection {
    TypenameField(TypenameField),
    SpreadSelection(SpreadSelection),
    FieldSelection(FieldSelection),
}

pub struct UnionFragmentSpec {
    pub r#type: Rc<RefCell<server::ast::Union>>,
    pub selections: Vec<UnionSelection>,
}

pub struct ObjectFragmentSpec<T> {
    pub r#type: Rc<RefCell<T>>,
    pub selections: Vec<ObjectSelection>,
}

pub struct ObjectConditionalSpreadSelection {
    pub r#type: Rc<RefCell<server::ast::ObjectType>>,
    pub selection: Rc<ObjectFragmentSpec<server::ast::ObjectType>>,
}

pub struct UnionConditionalSpreadSelection {
    pub r#type: Rc<RefCell<server::ast::Union>>,
    pub selection: Rc<UnionFragmentSpec>,
}

#[derive(derive_more::From)]
pub enum FragmentSpec {
    Union(UnionFragmentSpec),
    Object(ObjectFragmentSpec<server::ast::ObjectType>),
    Interface(ObjectFragmentSpec<server::ast::Interface>),
}

pub struct FieldSelection {
    pub name: String,
    pub alias: String,
    pub arguments:
        indexmap::IndexMap<String, shared::ast::FieldSelectionArgument>,
    pub selection: Option<Rc<FragmentSpec>>,
}

pub struct Fragment {
    pub name: String,
    pub spec: FragmentSpec,
    pub source_text: String,
    pub hash: u64,
}

pub struct Operation {
    pub r#type: file::client::ast::OpType,
    pub name: String,
    pub parameters: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
    pub fragment_spec: FragmentSpec,
    pub used_fragments: Vec<Rc<Fragment>>,
    pub source_text: String,
    pub parameters_hash: u64,
    pub fragment_spec_hash: u64,
}

pub struct ClientDirective {
    pub name: String,
    pub arguments: indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<shared::ast::InputFieldSpec>>,
    >,
    pub locations: Vec<file::client::ast::DirectiveLocation>,
}

#[derive(derive_more::From)]
pub enum ClientSchemaNode {
    Fragment(Rc<Fragment>),
    Operation(Rc<Operation>),
    ClientDirective(Rc<ClientDirective>),
}
