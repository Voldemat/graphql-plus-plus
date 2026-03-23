use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client::ast, server, shared, type_registry},
    },
};

#[derive(Debug, derive_more::From)]
pub enum FragmentType {
    Object(Arc<RefCell<server::ast::ObjectType>>),
    Interface(Arc<RefCell<server::ast::Interface>>),
    Union(Arc<RefCell<server::ast::Union>>),
}

#[derive(Debug, derive_more::From)]
pub enum FieldType {
    Object(Arc<RefCell<server::ast::ObjectType>>),
    Interface(Arc<RefCell<server::ast::Interface>>),
}

#[derive(Debug)]
pub enum Error {
    TypeRegistryError(type_registry::Error),
    UnknownFragmentType(file::shared::ast::NameNode),
    UnexpectedConditionalSpreadSelectionNode(
        file::client::ast::ConditionalSpreadSelectionNode,
    ),
    UnknownFragment(file::shared::ast::NameNode),
    InvalidFragmentType {
        selection_node: file::client::ast::SpreadSelectionNode,
        expected_type: FragmentType,
        fragment: Arc<RefCell<ast::Fragment>>,
    },
    UnknownField {
        r#type: FieldType,
        field: file::shared::ast::NameNode,
    },
    UnexpectedCallableField {
        field_type:
            Arc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
        definition: file::client::ast::ObjectCallableFieldSpec,
    },
    UnexpectedFieldSelectionNodeOnUnion(file::client::ast::FieldSelectionNode),
    NoSuitableTypeForConditionalSpreadSelection {
        selection: file::client::ast::ConditionalSpreadSelectionNode,
        r#type: Arc<RefCell<server::ast::Union>>,
    },
    UnexpectedSelectionOnLiteralField {
        spec: Rc<file::client::ast::FragmentSpec>,
        field: Arc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    },
    InvalidLiteralForInput {
        type_spec: shared::ast::InputTypeSpec,
        node: file::shared::ast::LiteralNode,
    },
}

impl Error {
    pub fn get_location(self: &Self) -> &lexer::tokens::Location {
        match self {
            Self::TypeRegistryError(error) => error.get_location(),
            Self::UnknownFragmentType(name_node) => {
                &name_node.location.start_token.location
            }
            Self::UnexpectedConditionalSpreadSelectionNode(node) => {
                &node.location.start_token.location
            }
            Self::UnknownFragment(name_node) => {
                &name_node.location.start_token.location
            }
            Self::InvalidFragmentType { selection_node, .. } => {
                &selection_node.location.start_token.location
            }
            Self::UnknownField { field, .. } => {
                &field.location.start_token.location
            }
            Self::UnexpectedCallableField { definition, .. } => {
                &definition.location.start_token.location
            }
            Self::UnexpectedFieldSelectionNodeOnUnion(node) => {
                &node.location.start_token.location
            }
            Self::NoSuitableTypeForConditionalSpreadSelection {
                selection,
                ..
            } => &selection.location.start_token.location,
            Self::UnexpectedSelectionOnLiteralField { spec, .. } => {
                &spec.location.start_token.location
            }
            Self::InvalidLiteralForInput { node, .. } => node.get_location(),
        }
    }

    pub fn get_source_file(self: &Self) -> &Arc<file::shared::ast::SourceFile> {
        match self {
            Self::TypeRegistryError(error) => error.get_source_file(),
            Self::UnknownFragmentType(name_node) => &name_node.location.source,
            Self::UnexpectedConditionalSpreadSelectionNode(node) => {
                &node.location.source
            }
            Self::UnknownFragment(name_node) => &name_node.location.source,
            Self::InvalidFragmentType { selection_node, .. } => {
                &selection_node.location.source
            }
            Self::UnknownField { field, .. } => &field.location.source,
            Self::UnexpectedCallableField { definition, .. } => {
                &definition.location.source
            }
            Self::UnexpectedFieldSelectionNodeOnUnion(node) => {
                &node.location.source
            }
            Self::NoSuitableTypeForConditionalSpreadSelection {
                selection,
                ..
            } => &selection.location.source,
            Self::UnexpectedSelectionOnLiteralField { spec, .. } => {
                &spec.location.source
            }
            Self::InvalidLiteralForInput { node, .. } => node.get_source_file(),
        }
    }
}

impl From<type_registry::Error> for Error {
    fn from(value: type_registry::Error) -> Self {
        return Self::TypeRegistryError(value);
    }
}
