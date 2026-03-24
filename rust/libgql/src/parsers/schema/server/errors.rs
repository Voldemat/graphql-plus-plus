use std::sync::Arc;

use crate::parsers::{
    file,
    schema::{shared, type_registry},
};

pub type ArgType = shared::ast::FieldDefinition<
    shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
>;

#[derive(Debug)]
pub enum Error<'buffer> {
    TypeRegistryError(type_registry::Error<'buffer>),
    UnexpectedArgumentValue {
        value: file::shared::ast::LiteralNode<'buffer>,
        arg_type: ArgType,
    },
    InvalidEnumValue {
        value: file::shared::ast::LiteralEnumValueNode<'buffer>,
        enum_type: Arc<shared::ast::Enum>,
    },
    UnknownServerDirective(file::shared::ast::NameNode<'buffer>),
    UnknownInterface(file::shared::ast::NameNode<'buffer>),
    UnknownObject(file::shared::ast::NameNode<'buffer>),
}

impl<'buffer> From<type_registry::Error<'buffer>> for Error<'buffer> {
    fn from(value: type_registry::Error<'buffer>) -> Self {
        return Self::TypeRegistryError(value);
    }
}
