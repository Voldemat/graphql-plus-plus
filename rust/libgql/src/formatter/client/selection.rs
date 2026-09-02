use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::SelectionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::SelectionNode::FieldSelectionNode(node) => {
            super::field_selection::format_node(
                shared_config,
                client_config,
                node,
            )
        }
        ast::SelectionNode::SpreadSelectionNode(node) => {
            super::spread_selection::format_node(
                shared_config,
                client_config,
                node,
            )
        }
        ast::SelectionNode::ConditionalSpreadSelectionNode(node) => {
            super::conditional_spread_selection::format_node(
                shared_config,
                client_config,
                node,
            )
        }
    }
}
