use crate::parsers::file::server::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TServerConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    server_config: &TServerConfig,
    ast_node: &ast::TypeDefinitionNode<'s>,
) -> codeform::ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::TypeDefinitionNode::Enum(node) => {
            super::enum_type::format_node(shared_config, server_config, node)
        }
        ast::TypeDefinitionNode::Union(node) => {
            super::union::format_node(shared_config, server_config, node)
        }
        ast::TypeDefinitionNode::Object(node) => super::object::format_node(
            shared_config,
            server_config,
            false,
            node,
        ),
        ast::TypeDefinitionNode::Interface(node) => {
            super::interface::format_node(shared_config, server_config, node)
        }
        ast::TypeDefinitionNode::Input(node) => {
            super::input::format_node(shared_config, server_config, node)
        }
        ast::TypeDefinitionNode::Scalar(node) => {
            super::scalar::format_node(shared_config, server_config, node)
        }
        ast::TypeDefinitionNode::Directive(node) => {
            super::directive::format_node(shared_config, server_config, node)
        }
    }
}
