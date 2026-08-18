use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::Argument<'s>,
    is_last_node: bool,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
        ir::hir::builders::ascii_oneline_text(": "),
    ])
    .push(super::argument_value::format_node(config, &ast_node.value))
    .extend_if(
        !is_last_node,
        [
            ir::hir::builders::byte(b','),
            ir::hir::builders::soft_line_or_space(),
        ],
    )
}
