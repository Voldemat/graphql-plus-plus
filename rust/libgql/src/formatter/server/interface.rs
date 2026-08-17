use codeform::ir;

use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::InterfaceDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("interface"),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
        ir::hir::builders::byte(b' '),
        ir::hir::builders::byte(b'{'),
        ir::hir::builders::hard_line(),
    ])
    .extend(ir::hir::builders::wrap_in_hard_indent(
        ast_node
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                super::field::format_node(
                    config,
                    field,
                    index == ast_node.fields.len() - 1,
                )
            })
            .flatten(),
    ))
    .extend([
        ir::hir::builders::hard_line(),
        ir::hir::builders::byte(b'}'),
    ])
}
