use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::FragmentSpec<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ast_node
        .selections
        .iter()
        .enumerate()
        .map(|(index, selection)| {
            super::selection::format_node(
                shared_config,
                client_config,
                selection,
            )
            .push_if(
                index != ast_node.selections.len() - 1,
                ir::hir::builders::hard_line(),
            )
        })
        .flatten()
        .collect()
}
