use crate::parsers::{file, schema::shared};

#[derive(Debug, Clone)]
pub struct SpreadSelection {
    pub fragment: String,
}

#[derive(Debug, Clone)]
pub struct TypenameField {
    pub alias: Option<String>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum UnionSelection {
    TypenameField(TypenameField),
    SpreadSelection(SpreadSelection),
    ObjectConditionalSpreadSelection(ObjectConditionalSpreadSelection),
    UnionConditionalSpreadSelection(UnionConditionalSpreadSelection),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ObjectSelection {
    TypenameField(TypenameField),
    SpreadSelection(SpreadSelection),
    FieldSelection(FieldSelection),
}

#[derive(Debug, Clone)]
pub struct UnionFragmentSpec {
    pub r#type: String,
    pub selections: Vec<UnionSelection>,
}

#[derive(Debug, Clone)]
pub struct ObjectFragmentSpec {
    pub r#type: String,
    pub selections: Vec<ObjectSelection>,
}

#[derive(Debug, Clone)]
pub struct InterfaceFragmentSpec {
    pub r#type: String,
    pub selections: Vec<ObjectSelection>,
}

#[derive(Debug, Clone)]
pub struct ObjectConditionalSpreadSelection {
    pub r#type: String,
    pub selections: Vec<ObjectSelection>,
}

#[derive(Debug, Clone)]
pub struct UnionConditionalSpreadSelection {
    pub r#type: String,
    pub selection: Vec<UnionSelection>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum FragmentSpec {
    Union(UnionFragmentSpec),
    Object(ObjectFragmentSpec),
    Interface(InterfaceFragmentSpec),
}

#[derive(Debug, Clone)]
pub struct FieldSelection {
    pub name: String,
    pub alias: String,
    pub arguments:
        indexmap::IndexMap<String, shared::ast::FieldSelectionArgument>,
    pub selection: Option<FragmentSpec>,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub name: String,
    pub spec: FragmentSpec,
    pub source_text: String,
    pub hash: u64,
}

pub type OpType = file::client::ast::OpType;

#[derive(Debug, Clone)]
pub struct Operation {
    pub r#type: OpType,
    pub name: String,
    pub parameters: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
    pub fragment_spec: FragmentSpec,
    pub used_fragments: Vec<String>,
    pub source_text: String,
    pub parameters_hash: u64,
    pub fragment_spec_hash: u64,
}

pub type DirectiveLocation = file::client::ast::DirectiveLocation;

#[derive(Debug, Clone)]
pub struct ClientDirective {
    pub name: String,
    pub arguments: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
    pub locations: Vec<DirectiveLocation>,
}
