use codeform::ir;

use crate::parsers::file::client::ast;

pub fn format_node<
    's,
    TSharedConfig: crate::formatter::shared::config::Config,
    TClientConfig: super::config::Config,
>(
    shared_config: &TSharedConfig,
    client_config: &TClientConfig,
    ast_node: &ast::FieldSelectionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::wrap_in_group(
        ir::hir::builders::unanonymous_default_flat_group(),
        super::object_field_spec::format_node(
            shared_config,
            client_config,
            &ast_node.field,
        )
        .extend_if(
            ast_node.spec.is_some(),
            [ir::hir::builders::space(), ir::hir::builders::byte(b'{')],
        ),
    )
    .extend_if_some(ast_node.spec.as_ref(), |spec| {
        ir::hir::builders::NodesVec::from_node(ir::hir::builders::hard_line())
            .extend(ir::hir::builders::wrap_in_hard_indent(
                super::fragment_spec::format_node(
                    shared_config,
                    client_config,
                    spec,
                ),
            ))
            .push(ir::hir::builders::hard_line())
            .push(ir::hir::builders::byte(b'}'))
    })
}
