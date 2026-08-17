use codeform::ir;

use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::ExtendTypeNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("extend"),
        ir::hir::builders::byte(b' '),
    ])
    .extend(super::object::format_node(
        config,
        true,
        &ast_node.type_node,
    ))
}
