use crate::parsers::file::server::ast;

pub fn format_nodes<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TServerConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    server_config: &TServerConfig,
    ast_nodes: &[ast::ASTNode<'s>],
) -> codeform::ir::hir::builders::NodesVec<'s> {
    ast_nodes
        .iter()
        .map(|node| {
            super::node::format_node(shared_config, server_config, node)
                .push(codeform::ir::hir::builders::empty_line())
        })
        .flatten()
        .collect()
}
