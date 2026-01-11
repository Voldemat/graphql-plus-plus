use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{server::errors, shared, type_registry},
};

fn parse_argument_value_from_literal_node(
    value: &file::shared::ast::LiteralNode,
    arg_type: &errors::ArgType,
) -> Result<shared::ast::ArgumentValue, errors::Error> {
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
            if !enum_type.values.contains(&e.value) {
                return Err(errors::Error::InvalidEnumValue {
                    value: e.clone(),
                    enum_type: enum_type.clone(),
                });
            };
            return Ok(shared::ast::ArgumentLiteralValue::EnumValue(
                e.value.clone(),
            )
            .into());
        }
    };
}

fn parse_argument_value(
    value: &file::shared::ast::ArgumentValue,
    arg_type: &errors::ArgType,
) -> Result<shared::ast::ArgumentValue, errors::Error> {
    match value {
        file::shared::ast::ArgumentValue::NameNode(name) => {
            Ok(shared::ast::ArgumentValue::Ref(name.name.clone()))
        }
        file::shared::ast::ArgumentValue::LiteralNode(literal) => {
            parse_argument_value_from_literal_node(&literal, arg_type)
        }
    }
}

pub fn parse_arguments(
    arguments: &Vec<file::shared::ast::Argument>,
    directive: &Rc<RefCell<shared::ast::ServerDirective>>,
) -> Result<
    indexmap::IndexMap<String, shared::ast::FieldSelectionArgument>,
    errors::Error,
> {
    let mut final_arguments =
        indexmap::IndexMap::<String, shared::ast::FieldSelectionArgument>::new(
        );
    let directive_br = directive.borrow();
    for argument in arguments {
        let Some(arg_type) = directive_br.arguments.get(&argument.name.name)
        else {
            return Err(type_registry::Error::UnknownArgument(
                argument.name.clone(),
            )
            .into());
        };
        final_arguments.insert(
            argument.name.name.clone(),
            shared::ast::FieldSelectionArgument {
                name: argument.name.name.clone(),
                value: parse_argument_value(&argument.value, arg_type)?,
                r#type: arg_type.clone(),
            },
        );
    }
    return Ok(final_arguments);
}
