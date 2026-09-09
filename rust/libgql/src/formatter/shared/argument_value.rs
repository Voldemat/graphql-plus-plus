use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<'s, TSharedConfig: super::config::Config>(
    shared_config: &TSharedConfig,
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
