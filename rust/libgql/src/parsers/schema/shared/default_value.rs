use crate::parsers::file;

use super::ast;

#[derive(Debug)]
pub enum Error<'buffer> {
    UnexpectedLiteralForInputType(file::shared::ast::LiteralNode<'buffer>),
    UnexpectedLiteralForEnumType(file::shared::ast::LiteralNode<'buffer>),
    UnexpectedLiteralForScalar {
        literal: file::shared::ast::LiteralNode<'buffer>,
        scalar: String,
    },
    UnexpectedEnumValueForScalar(
        file::shared::ast::LiteralEnumValueNode<'buffer>,
    ),
}

impl<'buffer> std::fmt::Display for Error<'buffer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedLiteralForInputType(_) =>
                f.write_str("Unexpected literal for input type, input type cannot have default value"),
            Self::UnexpectedLiteralForEnumType(_) =>
                f.write_str("Unexpected literal for enum type"),
            Self::UnexpectedLiteralForScalar { literal, scalar } => {
                f.write_fmt(format_args!("Unexpected literal {} for scalar {}", literal.get_location().get_source_slice(), scalar))
            }
            Self::UnexpectedEnumValueForScalar(_) => {
                f.write_str("Unexpected enum value for scalar")
            }
        }
    }
}

impl<'buffer> Error<'buffer> {
    pub fn get_location(
        self: &Self,
    ) -> &file::shared::ast::NodeLocation<'buffer> {
        match self {
            Self::UnexpectedLiteralForInputType(node) => node.get_location(),
            Self::UnexpectedLiteralForEnumType(node) => node.get_location(),
            Self::UnexpectedLiteralForScalar { literal, .. } => {
                literal.get_location()
            }
            Self::UnexpectedEnumValueForScalar(literal) => &literal.location,
        }
    }
}

pub fn parse_default_value<'buffer>(
    r#type: ast::traits::InputTypeSpecRef<'_>,
    literal: &file::shared::ast::LiteralNode<'buffer>,
) -> Result<ast::runtime::Literal, Error<'buffer>> {
    match r#type {
        ast::traits::InputTypeSpecRef::InputType(_) => {
            Err(Error::UnexpectedLiteralForInputType(literal.clone()))
        }
        ast::traits::InputTypeSpecRef::Enum(_) => match literal {
            file::shared::ast::LiteralNode::EnumValue(s) => {
                Ok(ast::runtime::Literal::String(s.value.to_string()))
            }
            _ => Err(Error::UnexpectedLiteralForEnumType(literal.clone())),
        },
        ast::traits::InputTypeSpecRef::Scalar(scalar) => match literal {
            file::shared::ast::LiteralNode::Int(v) => {
                if scalar == "Int" {
                    Ok(ast::runtime::Literal::Int(v.value))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::Float(v) => {
                if scalar == "Float" {
                    Ok(ast::runtime::Literal::Float(v.value))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::Boolean(v) => {
                if scalar == "Boolean" {
                    Ok(ast::runtime::Literal::Boolean(v.value))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::String(v) => {
                if scalar == "String" {
                    Ok(ast::runtime::Literal::String(v.value.to_string()))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::EnumValue(v) => {
                Err(Error::UnexpectedEnumValueForScalar(v.clone()))
            }
        },
    }
}
