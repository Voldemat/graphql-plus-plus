use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<'buffer, TConfig: super::config::Config>(
    config: &TConfig,
    ast_node: &ast::DocumentationNode<'buffer>,
) -> ir::hir::builders::NodesVec<'buffer> {
    if !ast_node.multiline {
        return ir::hir::builders::NodesVec::from_iter([
            ir::hir::builders::expand_parent(),
            ir::hir::builders::unicode_text(
                ast_node.location.get_source_slice(),
                config.get_indent_width(),
                |c| {
                    unicode_width::UnicodeWidthChar::width(c)
                        .unwrap_or_default()
                },
            ),
            ir::hir::builders::hard_line(),
        ]);
    }
    let mut first_line_indent = 0;
    let lines =
        ast_node
            .string
            .trim()
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if index == 0 {
                    first_line_indent =
                        line.chars().take_while(|&c| c != ' ').count();
                    line
                } else {
                    &line[first_line_indent + 1..]
                }
            });
    ir::hir::builders::NodesVec::from_iter([
        ir::hir::builders::expand_parent(),
        ir::hir::builders::ascii_oneline_text("\"\"\""),
        ir::hir::builders::hard_line(),
    ])
    .extend(
        lines
            .map(|line| {
                ir::hir::builders::NodesVec::from_iter([
                    ir::hir::builders::unicode_text(
                        line,
                        config.get_indent_width(),
                        |c| {
                            unicode_width::UnicodeWidthChar::width(c)
                                .unwrap_or_default()
                        },
                    ),
                    ir::hir::builders::hard_line(),
                ])
            })
            .flatten(),
    )
    .extend([
        ir::hir::builders::ascii_oneline_text("\"\"\""),
        ir::hir::builders::hard_line(),
    ])
}
