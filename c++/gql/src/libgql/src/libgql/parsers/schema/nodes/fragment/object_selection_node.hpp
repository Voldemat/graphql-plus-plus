#pragma once

#include <memory>

#include "../../../file/client/ast.hpp"
#include "../../../file/shared/parser_error.hpp"
#include "../../client_ast.hpp"
#include "libgql/parsers/schema/nodes/fragment/field_selection_node.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

namespace parsers::schema::nodes {
ast::SpreadSelection parseSpreadSelectionNode(
    const file::client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ast::Interface> &type, const TypeRegistry &registry);

ast::SpreadSelection parseSpreadSelectionNode(
    const file::client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ast::ObjectType> &type, const TypeRegistry &registry);

template <typename T>
ast::ObjectSelection parseObjectSelectionNode(
    const file::client::ast::SelectionNode &sNode,
    const std::shared_ptr<T> &type, const TypeRegistry &registry) {
    return std::visit<ast::ObjectSelection>(
        overloaded{
            [&registry,
             &type](const file::client::ast::SpreadSelectionNode &node) {
                return nodes::parseSpreadSelectionNode(node, type, registry);
            },
            [](const file::client::ast::ConditionalSpreadSelectionNode &node)
                -> ast::ObjectSelection {
                throw file::shared::ParserError(
                    node.location.startToken,
                    "Conditional spread selection is "
                    "disallowed on object fragments",
                    node.location.source);
            },
            [&registry,
             &type](const file::client::ast::FieldSelectionNode &node) {
                return parseFieldSelectionNode(node, type, registry);
            } },
        sNode);
};
};  // namespace parsers::schema::nodes
