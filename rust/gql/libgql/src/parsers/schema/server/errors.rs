use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{shared, type_registry},
};

pub type ArgType = shared::ast::FieldDefinition<
    shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
>;

#[derive(Debug)]
pub enum Error {
    TypeRegistryError(type_registry::Error),
    UnexpectedArgumentValue {
        value: file::shared::ast::LiteralNode,
        arg_type: ArgType,
    },
    InvalidEnumValue {
        value: file::shared::ast::LiteralEnumValueNode,
        enum_type: Rc<RefCell<shared::ast::Enum>>,
    },
    UnknownServerDirective(file::shared::ast::NameNode),
    UnknownInterface(file::shared::ast::NameNode),
    UnknownObject(file::shared::ast::NameNode)
}

impl From<type_registry::Error> for Error {
    fn from(value: type_registry::Error) -> Self {
        return Self::TypeRegistryError(value);
    }
}
