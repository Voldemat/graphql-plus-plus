use crate::parsers::{
    file,
    schema::{
        server::{self, errors},
        shared,
    },
};

use super::type_registry::{HashMapTypeRegistry, TypeRegistry};

fn parse_argument_value_from_literal_node<'buffer>(
    value: &file::shared::ast::LiteralNode<'buffer>,
    arg_type: &shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::NonCallableFieldSpec<
            shared::ast::runtime::InputTypeSpec,
        >,
    >,
    registry: &HashMapTypeRegistry,
) -> Result<shared::ast::runtime::ArgumentValue, errors::Error<'buffer>> {
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
                shared::ast::runtime::Literal::String(s.value.to_string()),
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
            let shared::ast::runtime::InputTypeSpec::Enum(enum_type) =
                &s.r#type
            else {
                return Err(errors::Error::UnexpectedArgumentValue {
                    value: value.clone(),
                    arg_type: arg_type.clone_with_string_type(
                        |s| s.clone_with_string_type(shared::ast::runtime::InputTypeSpec::clone_with_string_type)
                    ),
                });
            };
            if !registry
                .get_enum(&enum_type)
                .unwrap()
                .values
                .iter()
                .any(|v| v == e.value)
            {
                return Err(errors::Error::InvalidEnumValue {
                    value: e.clone(),
                    enum_type: enum_type.to_string(),
                });
            };
            return Ok(shared::ast::runtime::Literal::EnumValue(
                e.value.to_string(),
            )
            .into());
        }
    };
}

pub fn parse_argument_value<'buffer>(
    value: &file::shared::ast::ArgumentValue<'buffer>,
    arg_type: &shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::NonCallableFieldSpec<
            shared::ast::runtime::InputTypeSpec,
        >,
    >,
    registry: &HashMapTypeRegistry,
) -> Result<shared::ast::runtime::ArgumentValue, errors::Error<'buffer>> {
    match value {
        file::shared::ast::ArgumentValue::NameNode(name) => Ok(
            shared::ast::runtime::ArgumentValue::Ref(name.name.to_string()),
        ),
        file::shared::ast::ArgumentValue::LiteralNode(literal) => {
            parse_argument_value_from_literal_node(&literal, arg_type, registry)
        }
    }
}
