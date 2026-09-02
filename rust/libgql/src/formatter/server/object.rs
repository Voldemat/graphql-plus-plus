use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TServerConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    server_config: &TServerConfig,
    is_in_extend_context: bool,
    ast_node: &ast::ObjectDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(shared_config, documentation)
        })
        .extend(
            ir::hir::builders::wrap_in_group(
                ir::hir::builders::unanonymous_default_flat_group(),
                ir::hir::builders::NodesVec::from_iterator([
                    ir::hir::builders::ascii_oneline_text("type"),
                    ir::hir::builders::byte(b' '),
                    ir::hir::builders::ascii_oneline_text(ast_node.name.name),
                ])
                .push_if(
                    ast_node.interfaces.len() != 0
                        || ast_node.fields.len() != 0,
                    ir::hir::builders::byte(b' '),
                )
                .extend_if(
                    ast_node.interfaces.len() != 0 && !is_in_extend_context,
                    ir::hir::builders::NodesVec::from_iterator([
                        ir::hir::builders::ascii_oneline_text("implements"),
                        ir::hir::builders::soft_line_or_space(),
                    ])
                    .extend(ir::hir::builders::wrap_in_soft_indent(
                        ast_node
                            .interfaces
                            .iter()
                            .enumerate()
                            .map(|(index, interface_name)| {
                                ir::hir::builders::NodesVec::from_node(
                                    ir::hir::builders::ascii_oneline_text(
                                        interface_name.name,
                                    ),
                                )
                                .extend_if(
                                    index != ast_node.interfaces.len() - 1,
                                    [
                                        ir::hir::builders::byte(b' '),
                                        ir::hir::builders::byte(b'&'),
                                        ir::hir::builders::soft_line_or_space(),
                                    ],
                                )
                            })
                            .flatten(),
                    ))
                    .push_if(
                        ast_node.fields.len() != 0,
                        ir::hir::builders::soft_line_or_space(),
                    ),
                )
                .push_if(
                    ast_node.fields.len() != 0,
                    ir::hir::builders::byte(b'{'),
                ),
            )
            .push_if(ast_node.fields.len() != 0, ir::hir::builders::hard_line())
            .extend(ir::hir::builders::wrap_in_hard_indent(
                ast_node
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        super::field::format_node(
                            shared_config,
                            server_config,
                            field,
                            index == ast_node.fields.len() - 1,
                        )
                    })
                    .flatten(),
            ))
            .extend_if(
                ast_node.fields.len() != 0,
                [
                    ir::hir::builders::hard_line(),
                    ir::hir::builders::byte(b'}'),
                ],
            ),
        )
}
