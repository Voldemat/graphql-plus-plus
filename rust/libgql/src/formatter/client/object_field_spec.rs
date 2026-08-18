use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::ObjectFieldSpec<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::ObjectFieldSpec::Literal(node) => {
            super::literal_field_spec::format_node(config, node)
        }
        ast::ObjectFieldSpec::Callable(node) => {
            super::callable_field_spec::format_node(config, node)
        }
    }
}
