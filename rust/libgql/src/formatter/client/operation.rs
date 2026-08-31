use codeform::ir;

use crate::{
    formatter::shared::{self, input_field::DelimeterMode},
    parsers::file::client::ast,
};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::OperationDefinitionNode<'s>,
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
                    ir::hir::builders::ascii_oneline_text(
                        ast_node.r#type.location.get_source_slice(),
                    ),
                    ir::hir::builders::byte(b' '),
                    ir::hir::builders::ascii_oneline_text(ast_node.name.name),
                ])
                .extend_if(
                    ast_node.parameters.len() != 0,
                    ir::hir::builders::NodesVec::from_iterator([
                        ir::hir::builders::byte(b'('),
                        ir::hir::builders::soft_line(),
                    ])
                    .extend(
                        crate::formatter::shared::input_field::format_nodes(
                            config.shared,
                            &ast_node.parameters,
                            DelimeterMode::CommaAndSoftLineOrSpace,
                        ),
                    )
                    .extend([
                        ir::hir::builders::soft_line(),
                        ir::hir::builders::byte(b')'),
                    ]),
                )
                .push(ir::hir::builders::ascii_oneline_text(" {")),
            )
            .push(ir::hir::builders::hard_line())
            .extend(ir::hir::builders::wrap_in_hard_indent(
                super::fragment_spec::format_node(config, &ast_node.fragment),
            ))
            .extend([
                ir::hir::builders::hard_line(),
                ir::hir::builders::byte(b'}'),
            ]),
        ))
}
