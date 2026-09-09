use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<'s, TSharedConfig: super::config::Config>(
    shared_config: &TSharedConfig,
    ast_node: &ast::DirectiveInvocationNode<'s>,
    is_last_node: bool,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::byte(b'@'),
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
    ])
    .extend_if(
        ast_node.arguments.len() != 0,
        ir::hir::builders::NodesVec::from_iterator([
            ir::hir::builders::byte(b'('),
            ir::hir::builders::soft_line(),
        ])
        .extend(super::argument::format_nodes(
            shared_config,
            &ast_node.arguments,
        ))
        .extend([
            ir::hir::builders::soft_line(),
            ir::hir::builders::byte(b')'),
        ]),
    )
    .push_if(!is_last_node, ir::hir::builders::soft_line_or_space())
}

pub fn format_nodes<'s, TConfig: super::config::Config>(
    config: &TConfig,
    ast_nodes: &[ast::DirectiveInvocationNode<'s>],
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::wrap_in_group(
        ir::hir::builders::unanonymous_default_flat_group(),
        ast_nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                format_node(config, node, index == ast_nodes.len() - 1)
            })
            .flatten(),
    )
}
