use crate::parsers::file::client::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::ASTNode<'s>,
) -> codeform::ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ASTNode::Fragment(node) => {
            super::fragment::format_node(config, &node)
        }
        ast::ASTNode::Operation(node) => {
            super::operation::format_node(config, node)
        }
        ast::ASTNode::Directive(node) => {
            super::directive::format_node(config, node)
        }
    }
}
