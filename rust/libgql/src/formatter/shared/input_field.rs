use codeform::ir;

use crate::parsers::file::shared::ast;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelimeterMode {
    CommaAndSoftLineOrSpace,
    HardLine,
}

pub fn format_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::InputFieldDefinitionNode<'s>,
    is_last_node: bool,
    delimeter_mode: DelimeterMode,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator(
        ir::hir::builders::wrap_in_group(
            ir::hir::builders::unanonymous_default_flat_group(),
            ir::hir::builders::wrap_in_soft_indent(
                ir::hir::builders::NodesVec::from_iterator([
                    ir::hir::builders::ascii_oneline_text(ast_node.name.name),
                    ir::hir::builders::byte(b':'),
                    ir::hir::builders::byte(b' '),
                ])
                .extend(super::type_node::format_node(config, &ast_node.r#type))
                .extend_if_some(
                    ast_node.default_value.as_ref(),
                    |default_value| {
                        ir::hir::builders::NodesVec::from_iterator([
                            ir::hir::builders::ascii_oneline_text(" = "),
                        ])
                        .extend(
                            super::literal::format_node(config, default_value),
                        )
                    },
                ),
            ),
        ),
    )
    .push_if(
        !is_last_node && delimeter_mode == DelimeterMode::HardLine,
        ir::hir::builders::hard_line(),
    )
    .extend_if(
        !is_last_node
            && delimeter_mode == DelimeterMode::CommaAndSoftLineOrSpace,
        [
            ir::hir::builders::byte(b','),
            ir::hir::builders::soft_line_or_space(),
        ],
    )
}

pub fn format_nodes<'s>(
    config: &crate::formatter::config::Config,
    ast_nodes: &[ast::InputFieldDefinitionNode<'s>],
    delimeter_mode: DelimeterMode,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::wrap_in_soft_indent(
        ast_nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                format_node(
                    config,
                    node,
                    index == ast_nodes.len() - 1,
                    delimeter_mode,
                )
            })
            .flatten(),
    )
}
