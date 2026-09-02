use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TServerConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    server_config: &TServerConfig,
    ast_node: &ast::InterfaceDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(shared_config, documentation)
        })
        .extend([
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
                        shared_config,
                        server_config,
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
