use codeform::ir;

use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::EnumDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    let last_node_index = ast_node.values.len() - 1;
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("enum"),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::byte(b'{'),
        ir::hir::builders::hard_line(),
    ])
    .extend(ir::hir::builders::wrap_in_soft_indent(
        ast_node
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                ir::hir::builders::NodesVec::from_iterator([
                    ir::hir::builders::ascii_oneline_text(value.value.name),
                ])
                .push_if(
                    index != last_node_index,
                    ir::hir::builders::hard_line(),
                )
            })
            .flatten(),
    ))
    .extend([
        ir::hir::builders::hard_line(),
        ir::hir::builders::byte(b'}'),
    ])
}
