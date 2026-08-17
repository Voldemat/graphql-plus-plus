use codeform::ir;

use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    ast_node: &ast::ScalarDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("scalar"),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
    ])
}
