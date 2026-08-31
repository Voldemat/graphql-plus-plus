use codeform::ir;

use crate::{formatter::shared, parsers::file::server::ast};

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::UnionDefinitionNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::NodesVec::empty()
        .extend_if_some(ast_node.documentation.as_ref(), |documentation| {
            shared::documentation::format_node(config.shared, documentation)
        })
        .extend(ir::hir::builders::wrap_in_group(
            ir::hir::builders::unanonymous_default_flat_group(),
            ir::hir::builders::NodesVec::from_iterator([
                ir::hir::builders::ascii_oneline_text("union"),
                ir::hir::builders::byte(b' '),
                ir::hir::builders::ascii_oneline_text(ast_node.name.name),
                ir::hir::builders::byte(b' '),
                ir::hir::builders::byte(b'='),
                ir::hir::builders::soft_line_or_space(),
            ])
            .extend(ir::hir::builders::wrap_in_soft_indent(
                ast_node
                    .values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        ir::hir::builders::NodesVec::from_node(
                            ir::hir::builders::ascii_oneline_text(value.name),
                        )
                        .extend_if(
                            index != ast_node.values.len() - 1,
                            [
                                ir::hir::builders::byte(b' '),
                                ir::hir::builders::byte(b'|'),
                                ir::hir::builders::soft_line_or_space(),
                            ],
                        )
                    })
                    .flatten(),
            )),
        ))
}
