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
    UnexpectedNullOnNonNullField(file::shared::ast::NodeLocation<'buffer>),
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
            Self::UnexpectedNullOnNonNullField(_) => {
                f.write_str("Unexpected null on non-null field")
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
            Self::UnexpectedNullOnNonNullField(location) => location,
        }
    }
}

pub fn parse_default_value<'buffer, S: ast::AsStr<'buffer>>(
    r#type: ast::traits::InputTypeSpecRef<'_>,
    nullable: bool,
    literal: &file::shared::ast::LiteralNode<'buffer>,
) -> Result<Option<ast::runtime::Literal<S>>, Error<'buffer>> {
    match r#type {
        ast::traits::InputTypeSpecRef::InputType(_) => match literal {
            file::shared::ast::LiteralNode::Null(location) => {
                if nullable {
                    Ok(None)
                } else {
                    Err(Error::UnexpectedNullOnNonNullField(location.clone()))
                }
            }
            _ => Err(Error::UnexpectedLiteralForInputType(literal.clone())),
        },
        ast::traits::InputTypeSpecRef::Enum(_) => match literal {
            file::shared::ast::LiteralNode::Null(location) => {
                if nullable {
                    Ok(None)
                } else {
                    Err(Error::UnexpectedNullOnNonNullField(location.clone()))
                }
            }
            file::shared::ast::LiteralNode::EnumValue(s) => {
                Ok(Some(ast::runtime::Literal::String(S::from_str(s.value))))
            }
            _ => Err(Error::UnexpectedLiteralForEnumType(literal.clone())),
        },
        ast::traits::InputTypeSpecRef::Scalar(scalar) => match literal {
            file::shared::ast::LiteralNode::Null(location) => {
                if nullable {
                    Ok(None)
                } else {
                    Err(Error::UnexpectedNullOnNonNullField(location.clone()))
                }
            }
            file::shared::ast::LiteralNode::Int(v) => {
                if scalar == "Int" || scalar == "Float" {
                    Ok(Some(ast::runtime::Literal::Int(v.value)))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::Float(v) => {
                if scalar == "Float" {
                    Ok(Some(ast::runtime::Literal::Float(v.value)))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::Boolean(v) => {
                if scalar == "Boolean" {
                    Ok(Some(ast::runtime::Literal::Boolean(v.value)))
                } else {
                    Err(Error::UnexpectedLiteralForScalar {
                        literal: literal.clone(),
                        scalar: scalar.to_string(),
                    })
                }
            }
            file::shared::ast::LiteralNode::String(v) => {
                if scalar == "String" {
                    Ok(Some(ast::runtime::Literal::String(S::from_str(
                        v.value,
                    ))))
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
