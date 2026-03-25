use crate::parsers::{
    file,
    schema::{server, shared},
};

use super::type_registry;

#[derive(Debug)]
pub enum FragmentType {
    Object(String),
    Interface(String),
    Union(String),
}

#[derive(Debug)]
pub enum FieldType {
    Object(String),
    Interface(String),
}

#[derive(Debug)]
pub enum Error<'buffer> {
    TypeRegistryError(type_registry::Error<'buffer>),
    ServerTypeRegistryError(server::type_registry::Error<'buffer>),
    UnknownFragmentType(file::shared::ast::NameNode<'buffer>),
    UnexpectedConditionalSpreadSelectionNode(
        file::client::ast::ConditionalSpreadSelectionNode<'buffer>,
    ),
    UnknownFragment(file::shared::ast::NameNode<'buffer>),
    InvalidFragmentType {
        selection_node: file::client::ast::SpreadSelectionNode<'buffer>,
        expected_type: FragmentType,
        fragment: String,
    },
    UnknownField {
        r#type: FieldType,
        field: file::shared::ast::NameNode<'buffer>,
    },
    UnexpectedCallableField {
        field_type: shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
        definition: file::client::ast::ObjectCallableFieldSpec<'buffer>,
    },
    UnexpectedFieldSelectionNodeOnUnion(
        file::client::ast::FieldSelectionNode<'buffer>,
    ),
    NoSuitableTypeForConditionalSpreadSelection {
        selection: file::client::ast::ConditionalSpreadSelectionNode<'buffer>,
        union_type: String,
    },
    UnexpectedSelectionOnLiteralField {
        spec: file::client::ast::FragmentSpec<'buffer>,
        field: shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
    },
    InvalidLiteralForInput {
        type_spec: shared::ast::InputTypeSpec,
        node: file::shared::ast::LiteralNode<'buffer>,
    },
    FragmentNameCollision(file::shared::ast::NameNode<'buffer>),
    OperationNameCollision(file::shared::ast::NameNode<'buffer>),
    DirectiveNameCollision(file::shared::ast::NameNode<'buffer>),
}

impl<'buffer> Error<'buffer> {
    pub fn get_location(
        self: &Self,
    ) -> &file::shared::ast::NodeLocation<'buffer> {
        match self {
            Self::TypeRegistryError(error) => error.get_location(),
            Self::ServerTypeRegistryError(error) => error.get_location(),
            Self::UnknownFragmentType(name_node) => &name_node.location,
            Self::FragmentNameCollision(node) => &node.location,
            Self::OperationNameCollision(node) => &node.location,
            Self::DirectiveNameCollision(node) => &node.location,
            Self::UnexpectedConditionalSpreadSelectionNode(node) => {
                &node.location
            }
            Self::UnknownFragment(name_node) => &name_node.location,
            Self::InvalidFragmentType { selection_node, .. } => {
                &selection_node.location
            }
            Self::UnknownField { field, .. } => &field.location,
            Self::UnexpectedCallableField { definition, .. } => {
                &definition.location
            }
            Self::UnexpectedFieldSelectionNodeOnUnion(node) => &node.location,
            Self::NoSuitableTypeForConditionalSpreadSelection {
                selection,
                ..
            } => &selection.location,
            Self::UnexpectedSelectionOnLiteralField { spec, .. } => {
                &spec.location
            }
            Self::InvalidLiteralForInput { node, .. } => node.get_location(),
        }
    }
}

impl<'buffer> From<type_registry::Error<'buffer>> for Error<'buffer> {
    fn from(value: type_registry::Error<'buffer>) -> Self {
        return Self::TypeRegistryError(value);
    }
}

impl<'buffer> From<server::type_registry::Error<'buffer>> for Error<'buffer> {
    fn from(value: server::type_registry::Error<'buffer>) -> Self {
        return Self::ServerTypeRegistryError(value);
    }
}
