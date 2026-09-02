use crate::parsers::file::server::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TServerConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    server_config: &TServerConfig,
    ast_node: &ast::ASTNode<'s>,
) -> codeform::ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ASTNode::ExtendTypeNode(node) => {
            super::extend::format_node(shared_config, server_config, node)
        }
        ast::ASTNode::TypeDefinitionNode(node) => {
            super::typedef::format_node(shared_config, server_config, node)
        }
    }
}
