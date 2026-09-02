use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::ObjectCallableFieldSpec<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if(
            ast_node.selection_name.name != ast_node.name.name,
            [
                ir::hir::builders::ascii_oneline_text(
                    ast_node.selection_name.name,
                ),
                ir::hir::builders::ascii_oneline_text(": "),
            ],
        )
        .push(ir::hir::builders::ascii_oneline_text(ast_node.name.name))
        .extend_if(
            ast_node.arguments.len() != 0,
            ir::hir::builders::NodesVec::from_iterator([
                ir::hir::builders::byte(b'('),
                ir::hir::builders::soft_line(),
            ])
            .extend(ir::hir::builders::wrap_in_soft_indent(
                ast_node
                    .arguments
                    .iter()
                    .enumerate()
                    .map(|(index, argument)| {
                        super::argument::format_node(
                            shared_config,
                            client_config,
                            argument,
                            index == ast_node.arguments.len() - 1,
                        )
                    })
                    .flatten(),
            ))
            .extend([
                ir::hir::builders::soft_line(),
                ir::hir::builders::byte(b')'),
            ]),
        )
}
