use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{client::ast, server, shared, type_registry},
};

#[derive(Debug, derive_more::From)]
pub enum FragmentType {
    Object(Rc<RefCell<server::ast::ObjectType>>),
    Interface(Rc<RefCell<server::ast::Interface>>),
    Union(Rc<RefCell<server::ast::Union>>),
}

#[derive(Debug, derive_more::From)]
pub enum FieldType {
    Object(Rc<RefCell<server::ast::ObjectType>>),
    Interface(Rc<RefCell<server::ast::Interface>>),
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
        fragment: Rc<RefCell<ast::Fragment>>,
    },
    UnknownField {
        r#type: FieldType,
        field: file::shared::ast::NameNode,
    },
    UnexpectedCallableField {
        field_type:
            Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
        definition: file::client::ast::ObjectCallableFieldSpec
    },
    UnexpectedFieldSelectionNodeOnUnion(file::client::ast::FieldSelectionNode),
    NoSuitableTypeForConditionalSpreadSelection {
        selection: file::client::ast::ConditionalSpreadSelectionNode,
        r#type: Rc<RefCell<server::ast::Union>>
    },
    UnexpectedSelectionOnLiteralField {
        spec: Rc<file::client::ast::FragmentSpec>,
        field: Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>
    },
    InvalidLiteralForInput {
        type_spec: shared::ast::InputTypeSpec,
        node: file::shared::ast::LiteralNode
    }
}

impl From<type_registry::Error> for Error {
    fn from(value: type_registry::Error) -> Self {
        return Self::TypeRegistryError(value);
    }
}
