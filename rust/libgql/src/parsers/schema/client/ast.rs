use crate::parsers::{file, schema::shared};

#[derive(Debug, Clone)]
pub struct SpreadSelection<S = String> {
    pub fragment: S,
}

#[derive(Debug, Clone)]
pub struct TypenameField<S = String> {
    pub alias: Option<S>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum UnionSelection<S = String> {
    TypenameField(TypenameField<S>),
    SpreadSelection(SpreadSelection<S>),
    ObjectConditionalSpreadSelection(ObjectConditionalSpreadSelection<S>),
    UnionConditionalSpreadSelection(UnionConditionalSpreadSelection<S>),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ObjectSelection<S = String> {
    TypenameField(TypenameField<S>),
    SpreadSelection(SpreadSelection<S>),
    FieldSelection(FieldSelection<S>),
}

#[derive(Debug, Clone)]
pub struct UnionFragmentSpec<S = String> {
    pub r#type: S,
    pub selections: Vec<UnionSelection<S>>,
}

#[derive(Debug, Clone)]
pub struct ObjectFragmentSpec<S = String> {
    pub r#type: S,
    pub selections: Vec<ObjectSelection<S>>,
}

#[derive(Debug, Clone)]
pub struct InterfaceFragmentSpec<S = String> {
    pub r#type: S,
    pub selections: Vec<ObjectSelection<S>>,
}

#[derive(Debug, Clone)]
pub struct ObjectConditionalSpreadSelection<S = String> {
    pub r#type: S,
    pub selections: Vec<ObjectSelection<S>>,
}

#[derive(Debug, Clone)]
pub struct UnionConditionalSpreadSelection<S = String> {
    pub r#type: S,
    pub selection: Vec<UnionSelection<S>>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum FragmentSpec<S = String> {
    Union(UnionFragmentSpec<S>),
    Object(ObjectFragmentSpec<S>),
    Interface(InterfaceFragmentSpec<S>),
}

#[derive(Debug, Clone)]
pub struct FieldSelection<S = String> {
    pub name: S,
    pub alias: S,
    pub arguments:
        indexmap::IndexMap<S, shared::ast::FieldSelectionArgument<S>>,
    pub selection: Option<FragmentSpec<S>>,
}

#[derive(Debug, Clone)]
pub struct Fragment<S = String> {
    pub name: S,
    pub spec: FragmentSpec<S>,
    pub source_text: S,
    pub hash: u64,
}

pub type OpType = file::client::ast::OpType;

#[derive(Debug, Clone)]
pub struct Operation<S = String> {
    pub r#type: OpType,
    pub name: S,
    pub parameters: indexmap::IndexMap<
        S,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec<S>, S>,
    >,
    pub fragment_spec: FragmentSpec<S>,
    pub used_fragments: Vec<S>,
    pub source_text: S,
    pub parameters_hash: u64,
    pub fragment_spec_hash: u64,
}

pub type DirectiveLocation = file::client::ast::DirectiveLocation;

#[derive(Debug, Clone)]
pub struct ClientDirective<S = String> {
    pub name: S,
    pub arguments: indexmap::IndexMap<
        S,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec<S>, S>,
    >,
    pub locations: Vec<DirectiveLocation>,
}
