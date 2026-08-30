use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<'s>(
    _config: &super::config::Config,
    ast_node: &ast::SpreadSelectionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("..."),
        ir::hir::builders::ascii_oneline_text(ast_node.fragment_name.name),
    ])
}
