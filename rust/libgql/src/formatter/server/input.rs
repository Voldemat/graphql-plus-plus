use codeform::ir;

use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::InputObjectDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("input"),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::byte(b'{'),
        ir::hir::builders::hard_line(),
    ])
    .extend(crate::formatter::shared::input_field::format_nodes(
        config,
        &ast_node.fields,
        crate::formatter::shared::input_field::DelimeterMode::HardLine,
    ))
    .extend([
        ir::hir::builders::hard_line(),
        ir::hir::builders::byte(b'}'),
    ])
}
