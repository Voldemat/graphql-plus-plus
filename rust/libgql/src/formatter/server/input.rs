use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::InputObjectDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(config.shared, documentation)
        })
        .extend([
            ir::hir::builders::ascii_oneline_text("input"),
            ir::hir::builders::byte(b' '),
            ir::hir::builders::ascii_oneline_text(ast_node.name.name),
            ir::hir::builders::byte(b' '),
            ir::hir::builders::byte(b'{'),
            ir::hir::builders::hard_line(),
        ])
        .extend(crate::formatter::shared::input_field::format_nodes(
            config.shared,
            &ast_node.fields,
            crate::formatter::shared::input_field::DelimeterMode::HardLine,
        ))
        .extend([
            ir::hir::builders::hard_line(),
            ir::hir::builders::byte(b'}'),
        ])
}
