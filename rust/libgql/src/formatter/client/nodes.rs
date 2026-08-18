use crate::parsers::file::client::ast;

pub fn format_nodes<'s>(
    config: &super::config::Config,
    ast_nodes: &[ast::ASTNode<'s>],
) -> codeform::ir::hir::builders::NodesVec<'s> {
    ast_nodes
        .iter()
        .map(|node| {
            super::node::format_node(config, node)
                .push(codeform::ir::hir::builders::empty_line())
        })
        .flatten()
        .collect()
}
