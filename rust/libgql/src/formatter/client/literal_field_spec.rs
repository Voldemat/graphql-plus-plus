use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::ObjectLiteralFieldSpec<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if(
            ast_node.selection_name.name != ast_node.name.name,
            [
                ir::hir::builders::ascii_oneline_text(
                    ast_node.selection_name.name,
                ),
                ir::hir::builders::ascii_oneline_text(": "),
            ],
        )
        .push(ir::hir::builders::ascii_oneline_text(ast_node.name.name))
}
