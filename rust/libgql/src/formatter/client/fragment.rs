use codeform::ir;

use crate::{formatter::shared, parsers::file::client::ast};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::FragmentDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(config.shared, documentation)
        })
        .extend(ir::hir::builders::wrap_in_group(
            ir::hir::builders::unanonymous_default_flat_group(),
            ir::hir::builders::wrap_in_group(
                ir::hir::builders::unanonymous_default_flat_group(),
                ir::hir::builders::NodesVec::from_iterator([
                    ir::hir::builders::ascii_oneline_text("fragment "),
                    ir::hir::builders::ascii_oneline_text(ast_node.name.name),
                    ir::hir::builders::soft_line_or_space(),
                ])
                .extend(ir::hir::builders::wrap_in_soft_indent([
                    ir::hir::builders::ascii_oneline_text("on "),
                    ir::hir::builders::ascii_oneline_text(
                        ast_node.type_name.name,
                    ),
                    ir::hir::builders::soft_line_or_space(),
                ]))
                .push(ir::hir::builders::byte(b'{')),
            )
            .push(ir::hir::builders::hard_line())
            .extend(ir::hir::builders::wrap_in_hard_indent(
                super::fragment_spec::format_node(config, &ast_node.spec),
            ))
            .extend([
                ir::hir::builders::hard_line(),
                ir::hir::builders::byte(b'}'),
            ]),
        ))
}
