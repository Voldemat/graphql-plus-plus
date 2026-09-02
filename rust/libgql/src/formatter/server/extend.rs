use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TServerConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    server_config: &TServerConfig,
    ast_node: &ast::ExtendTypeNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(shared_config, documentation)
        })
        .extend([
            ir::hir::builders::ascii_oneline_text("extend"),
            ir::hir::builders::byte(b' '),
        ])
        .extend(super::object::format_node(
            shared_config,
            server_config,
            true,
            &ast_node.type_node,
        ))
}
