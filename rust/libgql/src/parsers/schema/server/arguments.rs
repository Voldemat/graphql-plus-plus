use crate::parsers::{
    file,
    schema::{
        server::{self, errors},
        shared,
    },
};

use super::type_registry::TypeRegistry;

fn parse_argument_value_from_literal_node<'buffer>(
    value: &file::shared::ast::LiteralNode<'buffer>,
    arg_type: &errors::ArgType,
    registry: &TypeRegistry,
) -> Result<shared::ast::ArgumentValue, errors::Error<'buffer>> {
    return match value {
        file::shared::ast::LiteralNode::Int(i) => {
            Ok(shared::ast::ArgumentValue::Literal(i.value.into()))
        }
        file::shared::ast::LiteralNode::Float(f) => {
            Ok(shared::ast::ArgumentValue::Literal(f.value.into()))
        }
        file::shared::ast::LiteralNode::Boolean(b) => {
            Ok(shared::ast::ArgumentValue::Literal(b.value.into()))
        }
        file::shared::ast::LiteralNode::String(s) => {
            Ok(shared::ast::ArgumentValue::Literal(
                shared::ast::ArgumentLiteralValue::String(s.value.clone()),
            ))
        }
        file::shared::ast::LiteralNode::EnumValue(e) => {
            let shared::ast::NonCallableFieldSpec::Literal(s) = &arg_type.spec
            else {
                return Err(errors::Error::UnexpectedArgumentValue {
                    value: value.clone(),
                    arg_type: arg_type.clone(),
                });
            };
            let shared::ast::InputTypeSpec::Enum(enum_type) = &s.r#type else {
                return Err(errors::Error::UnexpectedArgumentValue {
                    value: value.clone(),
                    arg_type: arg_type.clone(),
                });
            };
            if !registry
                .enums
                .get(enum_type)
                .unwrap()
                .values
                .contains(&e.value)
            {
                return Err(errors::Error::InvalidEnumValue {
                    value: e.clone(),
                    enum_type: enum_type.to_string(),
                });
            };
            return Ok(shared::ast::ArgumentLiteralValue::EnumValue(
                e.value.clone(),
            )
            .into());
        }
    };
}

fn parse_argument_value<'buffer>(
    value: &file::shared::ast::ArgumentValue<'buffer>,
    arg_type: &errors::ArgType,
    registry: &TypeRegistry,
) -> Result<shared::ast::ArgumentValue, errors::Error<'buffer>> {
    match value {
        file::shared::ast::ArgumentValue::NameNode(name) => {
            Ok(shared::ast::ArgumentValue::Ref(name.name.to_string()))
        }
        file::shared::ast::ArgumentValue::LiteralNode(literal) => {
            parse_argument_value_from_literal_node(&literal, arg_type, registry)
        }
    }
}

pub fn parse_arguments<'buffer>(
    arguments: &Vec<file::shared::ast::Argument<'buffer>>,
    directive: &shared::ast::ServerDirective,
    registry: &TypeRegistry,
) -> Result<
    indexmap::IndexMap<String, shared::ast::FieldSelectionArgument>,
    errors::Error<'buffer>,
> {
    let mut final_arguments =
        indexmap::IndexMap::<String, shared::ast::FieldSelectionArgument>::new(
        );
    for argument in arguments {
        let Some(arg_type) = directive.arguments.get(argument.name.name) else {
            return Err(server::type_registry::Error::UnknownArgument(
                argument.name.clone(),
            )
            .into());
        };
        final_arguments.insert(
            argument.name.name.to_string(),
            shared::ast::FieldSelectionArgument {
                name: argument.name.name.to_string(),
                value: parse_argument_value(
                    &argument.value,
                    arg_type,
                    registry,
                )?,
                r#type: arg_type.clone(),
            },
        );
    }
    return Ok(final_arguments);
}
