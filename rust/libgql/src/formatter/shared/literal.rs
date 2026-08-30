use codeform::ir;

use crate::parsers::file::shared::ast;

pub fn format_node<'s>(
    config: &super::config::Config,
    ast_node: &ast::LiteralNode<'s>,
) -> ir::hir::builders::NodesVec<'s> {
    match ast_node {
        ast::LiteralNode::Null(location) => {
            ir::hir::builders::NodesVec::from_node(
                ir::hir::builders::ascii_oneline_text(
                    location.get_source_slice(),
                ),
            )
        }
        ast::LiteralNode::Int(value) => ir::hir::builders::NodesVec::from_node(
            ir::hir::builders::ascii_oneline_text(
                value.location.get_source_slice(),
            ),
        ),
        ast::LiteralNode::Float(value) => {
            ir::hir::builders::NodesVec::from_node(
                ir::hir::builders::ascii_oneline_text(
                    value.location.get_source_slice(),
                ),
            )
        }
        ast::LiteralNode::Boolean(value) => {
            ir::hir::builders::NodesVec::from_node(
                ir::hir::builders::ascii_oneline_text(
                    value.location.get_source_slice(),
                ),
            )
        }
        ast::LiteralNode::EnumValue(value) => {
            ir::hir::builders::NodesVec::from_node(
                ir::hir::builders::ascii_oneline_text(value.value),
            )
        }
        ast::LiteralNode::String(value) => {
            ir::hir::builders::NodesVec::from_iterator([
                ir::hir::builders::byte(b'"'),
                ir::hir::builders::unicode_text(
                    value.value,
                    config.indent_width,
                    |c| {
                        unicode_width::UnicodeWidthChar::width(c)
                            .unwrap_or_default()
                    },
                ),
                ir::hir::builders::byte(b'"'),
            ])
        }
    }
}
