use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_named_node<'s>(
    ast_node: &ast::NamedTypeNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_node(
        ir::hir::builders::ascii_oneline_text(ast_node.name.name),
    )
    .push_if(!ast_node.nullable, ir::hir::builders::byte(b'!'))
}

pub fn format_list_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::ListTypeNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::wrap_in_group(
        ir::hir::builders::unanonymous_default_flat_group(),
        ir::hir::builders::NodesVec::from_iterator([ir::hir::builders::byte(
            b'[',
        )])
        .extend(ir::hir::builders::wrap_in_indent(
            ir::hir::tag::IndentMode::Soft,
            format_node(config, ast_node.r#type.as_ref()),
        ))
        .push(ir::hir::builders::byte(b']'))
        .push_if(!ast_node.nullable, ir::hir::builders::byte(b'!')),
    )
}

pub fn format_node<'s>(
    config: &crate::formatter::config::Config,
    ast_node: &ast::TypeNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::TypeNode::Named(named) => format_named_node(named),
        ast::TypeNode::List(list) => format_list_node(config, list),
    }
}
