use crate::parsers::file::client::ast;

pub fn format_nodes<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_nodes: &[ast::ASTNode<'s>],
) -> codeform::ir::hir::builders::NodesVec<'s> {
    ast_nodes
        .iter()
        .map(|node| {
            super::node::format_node(shared_config, client_config, node)
                .push(codeform::ir::hir::builders::empty_line())
        })
        .flatten()
        .collect()
}
