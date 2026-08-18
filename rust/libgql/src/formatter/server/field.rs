use codeform::ir;

use crate::parsers::file::server::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::FieldDefinitionNode<'s>,
    is_last_node: bool,
) -> ir::hir::builders::NodesVec<'s> {
    ir::hir::builders::wrap_in_group(
        ir::hir::builders::unanonymous_default_flat_group(),
        ir::hir::builders::NodesVec::from_node(
            ir::hir::builders::ascii_oneline_text(ast_node.name.name)
        )
        .extend_if(ast_node.arguments.len() != 0,
            ir::hir::builders::NodesVec::from_iterator([
                ir::hir::builders::byte(b'('),ir::hir::builders::soft_line(),
            ])
                .extend(
                    crate::formatter::shared::input_field::format_nodes(
                        config.shared,
                        &ast_node.arguments,
                        crate::formatter::shared::input_field::DelimeterMode::CommaAndSoftLineOrSpace
                    )
                )
                .extend([
                    ir::hir::builders::soft_line(),
                    ir::hir::builders::byte(b')')
                ])
        )
        .extend([
            ir::hir::builders::byte(b':'),
            ir::hir::builders::byte(b' ')
        ])
        .extend(crate::formatter::shared::type_node::format_node(
            config.shared,
            &ast_node.r#type,
        ))
    )
    .push_if(!is_last_node, ir::hir::builders::soft_line_or_space())
}
