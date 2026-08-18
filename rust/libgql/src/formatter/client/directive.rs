use codeform::ir;

use crate::{
    formatter::shared::input_field::DelimeterMode, parsers::file::client::ast,
};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::DirectiveDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            [
                ir::hir::builders::unicode_text(
                    documentation.location.get_source_slice(),
                    config.shared.indent_width,
                    |c| {
                        unicode_width::UnicodeWidthChar::width(c)
                            .unwrap_or_default()
                    },
                ),
                ir::hir::builders::hard_line(),
            ]
        })
        .extend(ir::hir::builders::wrap_in_group(
            ir::hir::builders::unanonymous_default_flat_group(),
            ir::hir::builders::NodesVec::from_iterator([
                ir::hir::builders::ascii_oneline_text("directive"),
                ir::hir::builders::space(),
                ir::hir::builders::byte(b'@'),
                ir::hir::builders::ascii_oneline_text(ast_node.name.name),
            ])
            .extend_if(
                ast_node.arguments.len() != 0,
                ir::hir::builders::NodesVec::from_iterator([
                    ir::hir::builders::byte(b'('),
                    ir::hir::builders::soft_line(),
                ])
                .extend(crate::formatter::shared::input_field::format_nodes(
                    config.shared,
                    &ast_node.arguments,
                    DelimeterMode::CommaAndSoftLineOrSpace,
                ))
                .extend([
                    ir::hir::builders::soft_line(),
                    ir::hir::builders::byte(b')'),
                ]),
            )
            .extend([
                ir::hir::builders::space(),
                ir::hir::builders::ascii_oneline_text("on"),
                ir::hir::builders::space(),
            ])
            .extend(ir::hir::builders::wrap_in_group(
                ir::hir::builders::unanonymous_default_flat_group(),
                ast_node
                    .targets
                    .iter()
                    .enumerate()
                    .map(|(index, target)| {
                        ir::hir::builders::NodesVec::from_node(
                            ir::hir::builders::ascii_oneline_text(
                                target.location.get_source_slice(),
                            ),
                        )
                        .extend_if(
                            index != ast_node.targets.len() - 1,
                            [
                                ir::hir::builders::space(),
                                ir::hir::builders::byte(b'|'),
                                ir::hir::builders::soft_line_or_space(),
                            ],
                        )
                    })
                    .flatten(),
            )),
        ))
}
