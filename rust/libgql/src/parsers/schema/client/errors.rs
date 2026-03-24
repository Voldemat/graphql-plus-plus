use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{
    lexer,
    parsers::{
        file,
        schema::{client::ast, server, shared, type_registry},
    },
};

#[derive(Debug, derive_more::From)]
pub enum FragmentType {
    Object(Arc<RwLock<server::ast::ObjectType>>),
    Interface(Arc<RwLock<server::ast::Interface>>),
    Union(Arc<RwLock<server::ast::Union>>),
}

#[derive(Debug, derive_more::From)]
pub enum FieldType {
    Object(Arc<RwLock<server::ast::ObjectType>>),
    Interface(Arc<RwLock<server::ast::Interface>>),
}

#[derive(Debug)]
pub enum Error<'buffer> {
    TypeRegistryError(type_registry::Error<'buffer>),
    UnknownFragmentType(file::shared::ast::NameNode<'buffer>),
    UnexpectedConditionalSpreadSelectionNode(
        file::client::ast::ConditionalSpreadSelectionNode<'buffer>,
    ),
    UnknownFragment(file::shared::ast::NameNode<'buffer>),
    InvalidFragmentType {
        selection_node: file::client::ast::SpreadSelectionNode<'buffer>,
        expected_type: FragmentType,
        fragment: Arc<RwLock<ast::Fragment>>,
    },
    UnknownField {
        r#type: FieldType,
        field: file::shared::ast::NameNode<'buffer>,
    },
    UnexpectedCallableField {
        field_type:
            Arc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
        definition: file::client::ast::ObjectCallableFieldSpec<'buffer>,
    },
    UnexpectedFieldSelectionNodeOnUnion(
        file::client::ast::FieldSelectionNode<'buffer>,
    ),
    NoSuitableTypeForConditionalSpreadSelection {
        selection: file::client::ast::ConditionalSpreadSelectionNode<'buffer>,
        r#type: Arc<RwLock<server::ast::Union>>,
    },
    UnexpectedSelectionOnLiteralField {
        spec: Rc<file::client::ast::FragmentSpec<'buffer>>,
        field: Arc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    },
    InvalidLiteralForInput {
        type_spec: shared::ast::InputTypeSpec,
        node: file::shared::ast::LiteralNode<'buffer>,
    },
}

impl<'buffer> Error<'buffer> {
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

    pub fn get_source_file(
        self: &Self,
    ) -> &Arc<file::shared::ast::SourceFile<'buffer>> {
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

impl<'buffer> From<type_registry::Error<'buffer>> for Error<'buffer> {
    fn from(value: type_registry::Error<'buffer>) -> Self {
        return Self::TypeRegistryError(value);
    }
}
