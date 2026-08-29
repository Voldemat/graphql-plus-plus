use crate::{
    lexer,
    parsers::{file, schema::shared},
};

pub type ArgType = shared::ast::runtime::FieldDefinition<
    shared::ast::runtime::NonCallableFieldSpec<
        shared::ast::runtime::InputTypeSpec,
    >,
>;

#[derive(Debug)]
pub enum Error<'buffer> {
    TypeRegistryError(super::type_registry::Error<'buffer>),
    UnexpectedArgumentValue {
        value: file::shared::ast::LiteralNode<'buffer>,
        arg_type: ArgType,
    },
    InvalidEnumValue {
        value: file::shared::ast::LiteralEnumValueNode<'buffer>,
        enum_type: String,
    },
    UnknownServerDirective(file::shared::ast::NameNode<'buffer>),
    UnknownInterface(file::shared::ast::NameNode<'buffer>),
    UnknownObject(file::shared::ast::NameNode<'buffer>),
}

impl<'buffer> From<super::type_registry::Error<'buffer>> for Error<'buffer> {
    fn from(value: super::type_registry::Error<'buffer>) -> Self {
        return Self::TypeRegistryError(value);
    }
}

impl<'buffer> Error<'buffer> {
    pub fn get_location(
        self: &Self,
    ) -> &file::shared::ast::NodeLocation<'buffer> {
        match self {
            Self::TypeRegistryError(e) => e.get_location(),
            Self::UnexpectedArgumentValue { value, arg_type: _ } => {
                value.get_location()
            }
            Self::InvalidEnumValue {
                value,
                enum_type: _,
            } => &value.location,
            Self::UnknownServerDirective(node) => &node.location,
            Self::UnknownInterface(node) => &node.location,
            Self::UnknownObject(node) => &node.location,
        }
    }
}

impl<'buffer> std::fmt::Display for Error<'buffer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeRegistryError(e) => e.fmt(f),
            Self::UnexpectedArgumentValue { value, arg_type } => {
                f.write_fmt(format_args!(
                    "Unexpected argument value: {} for argument of type: {}",
                    value.get_location().get_source_slice(),
                    arg_type
                ))
            }
            Self::InvalidEnumValue { value, enum_type } => f.write_fmt(
                format_args!("Invalid enum value for {}", enum_type),
            ),
            Self::UnknownServerDirective(node) => f.write_fmt(format_args!(
                "Unknown server directive: {}",
                node.name
            )),
            Self::UnknownInterface(node) => {
                f.write_fmt(format_args!("Unknown interface: {}", node.name))
            }
            Self::UnknownObject(node) => {
                f.write_fmt(format_args!("Unknown object: {}", node.name))
            }
        }
    }
}
