use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::SelectionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::SelectionNode::FieldSelectionNode(node) => {
            super::field_selection::format_node(config, node)
        }
        ast::SelectionNode::SpreadSelectionNode(node) => {
            super::spread_selection::format_node(config, node)
        }
        ast::SelectionNode::ConditionalSpreadSelectionNode(node) => {
            super::conditional_spread_selection::format_node(config, node)
        }
    }
}
