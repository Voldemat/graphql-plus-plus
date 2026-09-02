use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::ObjectFieldSpec<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ObjectFieldSpec::Literal(node) => {
            super::literal_field_spec::format_node(
                shared_config,
                client_config,
                node,
            )
        }
        ast::ObjectFieldSpec::Callable(node) => {
            super::callable_field_spec::format_node(
                shared_config,
                client_config,
                node,
            )
        }
    }
}
