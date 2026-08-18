use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::ConditionalSpreadSelectionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("... on "),
        ir::hir::builders::ascii_oneline_text(ast_node.type_name.name),
        ir::hir::builders::ascii_oneline_text(" {"),
        ir::hir::builders::hard_line(),
    ])
    .extend(ir::hir::builders::wrap_in_hard_indent(
        super::fragment_spec::format_node(config, &ast_node.fragment),
    ))
    .extend([
        ir::hir::builders::hard_line(),
        ir::hir::builders::ascii_oneline_text("}"),
    ])
}
