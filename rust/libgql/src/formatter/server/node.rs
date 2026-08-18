use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::ASTNode<'s>,
) -> codeform::ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ASTNode::ExtendTypeNode(node) => {
            super::extend::format_node(config, &node)
        }
        ast::ASTNode::TypeDefinitionNode(node) => {
            super::typedef::format_node(config, node)
        }
    }
}
