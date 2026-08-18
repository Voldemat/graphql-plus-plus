use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::ArgumentValue<'s>,
) -> ir::hir::node::Node<'s> {
    ir::hir::builders::ascii_oneline_text(match ast_node {
        ast::ArgumentValue::NameNode(node) => node.name,
        ast::ArgumentValue::LiteralNode(literal_node) => match literal_node {
            ast::LiteralNode::Int(node) => node.location.get_source_slice(),
            ast::LiteralNode::Float(node) => node.location.get_source_slice(),
            ast::LiteralNode::Boolean(node) => node.location.get_source_slice(),
            ast::LiteralNode::String(node) => node.location.get_source_slice(),
            ast::LiteralNode::EnumValue(node) => {
                node.location.get_source_slice()
            }
        },
    })
}
