use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::ExtendTypeNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(config.shared, documentation)
        })
        .extend([
            ir::hir::builders::ascii_oneline_text("extend"),
            ir::hir::builders::byte(b' '),
        ])
        .extend(super::object::format_node(
            config,
            true,
            &ast_node.type_node,
        ))
}
