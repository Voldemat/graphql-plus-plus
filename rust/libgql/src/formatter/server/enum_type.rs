use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::EnumDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    let last_node_index = ast_node.values.len() - 1;
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(config.shared, documentation)
        })
        .extend([
            ir::hir::builders::ascii_oneline_text("enum"),
            ir::hir::builders::byte(b' '),
            ir::hir::builders::ascii_oneline_text(ast_node.name.name),
            ir::hir::builders::byte(b' '),
            ir::hir::builders::byte(b'{'),
            ir::hir::builders::hard_line(),
        ])
        .extend(ir::hir::builders::wrap_in_soft_indent(
            ast_node
                .values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    ir::hir::builders::NodesVec::empty()
                    .extend_if_some(value.documentation.as_ref(), |documentation| {
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
                    .extend([
                        ir::hir::builders::ascii_oneline_text(value.value.name),
                    ])
                    .push_if(
                        index != last_node_index,
                        ir::hir::builders::hard_line(),
                    )
                })
                .flatten(),
        ))
        .extend([
            ir::hir::builders::hard_line(),
            ir::hir::builders::byte(b'}'),
        ])
}
