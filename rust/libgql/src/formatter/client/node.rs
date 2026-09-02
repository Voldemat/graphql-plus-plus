use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::ASTNode<'s>,
) -> codeform::ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ASTNode::Fragment(node) => {
            super::fragment::format_node(shared_config, client_config, node)
        }
        ast::ASTNode::Operation(node) => {
            super::operation::format_node(shared_config, client_config, node)
        }
        ast::ASTNode::Directive(node) => {
            super::directive::format_node(shared_config, client_config, node)
        }
    }
}
