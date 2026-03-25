use crate::parsers::{file, schema::shared::ast};

pub fn parse_literal(node: &file::shared::ast::LiteralNode) -> ast::Literal {
    match node {
        file::shared::ast::LiteralNode::Int(i) => ast::Literal::Int(i.value),
        file::shared::ast::LiteralNode::Float(i) => {
            ast::Literal::Float(i.value)
        }
        file::shared::ast::LiteralNode::Boolean(i) => {
            ast::Literal::Boolean(i.value)
        }
        file::shared::ast::LiteralNode::String(i) => {
            ast::Literal::String(i.value.clone())
        }
        file::shared::ast::LiteralNode::EnumValue(i) => {
            ast::Literal::String(i.value.clone())
        }
    }
}
