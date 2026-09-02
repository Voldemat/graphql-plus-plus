use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::ArgumentValue<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ArgumentValue::NameNode(node) => {
            ir::hir::builders::NodesVec::from_node(
                ir::hir::builders::ascii_oneline_text(node.name),
            )
        }
        ast::ArgumentValue::LiteralNode(literal_node) => {
            crate::formatter::shared::literal::format_node(
                shared_config,
                literal_node,
            )
        }
    }
}
