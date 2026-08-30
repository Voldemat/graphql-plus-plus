use super::errors;
use crate::parsers::{
    file,
    schema::{server, shared},
};

fn parse_argument_value_from_literal_node<
    'client_buffer,
    'server_buffer: 'client_buffer,
    ClientStringType: shared::ast::AsStr<'client_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: server::type_registry::TypeRegistry<'server_buffer, ServerStringType>,
>(
    value: &file::shared::ast::LiteralNode<'client_buffer>,
    arg_type: &'server_buffer shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::NonCallableFieldSpec<
            shared::ast::runtime::InputTypeSpec<ServerStringType>,
            ServerStringType,
        >,
        ServerStringType,
    >,
    registry: &T,
) -> Result<
    shared::ast::runtime::ArgumentValue<ClientStringType>,
    errors::Error<'client_buffer, ClientStringType>,
> {
    return match value {
        file::shared::ast::LiteralNode::Null(location) => {
            if arg_type.nullable {
                Ok(shared::ast::runtime::ArgumentValue::Literal(
                    shared::ast::runtime::Literal::Null,
                ))
            } else {
                Err(server::type_registry::Error::DefaultValueValidationError(
                    shared::default_value::Error::UnexpectedNullOnNonNullField(
                        location.clone(),
                    ),
                )
                .into())
            }
        }
        file::shared::ast::LiteralNode::Int(i) => {
            Ok(shared::ast::runtime::ArgumentValue::Literal(i.value.into()))
        }
        file::shared::ast::LiteralNode::Float(f) => {
            Ok(shared::ast::runtime::ArgumentValue::Literal(f.value.into()))
        }
        file::shared::ast::LiteralNode::Boolean(b) => {
            Ok(shared::ast::runtime::ArgumentValue::Literal(b.value.into()))
        }
        file::shared::ast::LiteralNode::String(s) => {
            Ok(shared::ast::runtime::ArgumentValue::Literal(
                shared::ast::runtime::Literal::String(
                    ClientStringType::from_str(s.value),
                ),
            ))
        }
        file::shared::ast::LiteralNode::EnumValue(e) => {
            let shared::ast::runtime::NonCallableFieldSpec::Literal(s) =
                &arg_type.spec
            else {
                return Err(errors::Error::UnexpectedArgumentValue {
                    value: value.clone(),
                    arg_type: arg_type.clone_with_string_type(
                        |s| s.clone_with_string_type(shared::ast::runtime::InputTypeSpec::clone_with_string_type)
                    ),
                });
            };
            let shared::ast::runtime::InputTypeSpec::<ServerStringType>::Enum(
                enum_type,
            ) = &s.r#type
            else {
                return Err(errors::Error::UnexpectedArgumentValue {
                    value: value.clone(),
                    arg_type: arg_type.clone_with_string_type(
                        |s| s.clone_with_string_type(shared::ast::runtime::InputTypeSpec::clone_with_string_type)
                    ),
                });
            };
            if !registry
                .get_enum(enum_type.to_str())
                .unwrap()
                .values
                .iter()
                .any(|v| v == e.value)
            {
                return Err(errors::Error::InvalidEnumValue {
                    value: e.clone(),
                    enum_type: ClientStringType::from_str(enum_type.to_str()),
                });
            };
            return Ok(shared::ast::runtime::Literal::EnumValue(
                ClientStringType::from_str(e.value),
            )
            .into());
        }
    };
}

pub fn parse_argument_value<
    'client_buffer,
    'server_buffer: 'client_buffer,
    ClientStringType: shared::ast::AsStr<'client_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: server::type_registry::TypeRegistry<'server_buffer, ServerStringType>,
>(
    value: &file::shared::ast::ArgumentValue<'client_buffer>,
    arg_type: &'server_buffer shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::NonCallableFieldSpec<
            shared::ast::runtime::InputTypeSpec<ServerStringType>,
            ServerStringType,
        >,
        ServerStringType,
    >,
    registry: &T,
) -> Result<
    shared::ast::runtime::ArgumentValue<ClientStringType>,
    errors::Error<'client_buffer, ClientStringType>,
> {
    match value {
        file::shared::ast::ArgumentValue::NameNode(name) => {
            Ok(shared::ast::runtime::ArgumentValue::Ref(
                ClientStringType::from_str(name.name),
            ))
        }
        file::shared::ast::ArgumentValue::LiteralNode(literal) => {
            parse_argument_value_from_literal_node(&literal, arg_type, registry)
        }
    }
}
