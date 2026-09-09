use codeform::ir;

use crate::parsers::file::shared::ast;

use super::input_field::DelimeterMode;

pub fn format_node<'s, TSharedConfig: super::config::Config>(
    shared_config: &TSharedConfig,
    ast_node: &ast::Argument<'s>,
    is_last_node: bool,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
        ir::hir::builders::ascii_oneline_text(": "),
    ])
    .extend(super::argument_value::format_node(
        shared_config,
        &ast_node.value,
    ))
    .extend_if(
        !is_last_node,
        [
            ir::hir::builders::byte(b','),
            ir::hir::builders::soft_line_or_space(),
        ],
    )
}

pub fn format_nodes<'s, TConfig: super::config::Config>(
    config: &TConfig,
    ast_nodes: &[ast::Argument<'s>],
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::wrap_in_soft_indent(
        ast_nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                format_node(config, node, index == ast_nodes.len() - 1)
            })
            .flatten(),
    )
}
