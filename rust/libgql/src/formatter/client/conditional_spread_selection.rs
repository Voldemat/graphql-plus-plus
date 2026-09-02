use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::ConditionalSpreadSelectionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::from_iterator([
        ir::hir::builders::ascii_oneline_text("... on "),
        ir::hir::builders::ascii_oneline_text(ast_node.type_name.name),
        ir::hir::builders::ascii_oneline_text(" {"),
        ir::hir::builders::hard_line(),
    ])
    .extend(ir::hir::builders::wrap_in_hard_indent(
        super::fragment_spec::format_node(
            shared_config,
            client_config,
            &ast_node.fragment,
        ),
    ))
    .extend([
        ir::hir::builders::hard_line(),
        ir::hir::builders::ascii_oneline_text("}"),
    ])
}
