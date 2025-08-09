#pragma once

#include <memory>

#include "../../../file/client/ast.hpp"
#include "../../../file/shared/parser_error.hpp"
#include "../../client_ast.hpp"
#include "../../server_ast.hpp"
#include "../../type_registry.hpp"
#include "./field_selection_node.hpp"
#include "utils.hpp"

namespace gql::parsers::schema::nodes {
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
        utils::overloaded{
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
             &type](const file::client::ast::FieldSelectionNode &node)
                -> ast::ObjectSelection {
                if (isObjectFieldSpecIsTypenameField(node.field)) {
                    return (ast::TypenameField){
                        .alias =
                            file::client::ast::extractSelectionName(node.field)
                    };
                }
                return parseFieldSelectionNode(node, type, registry);
            } },
        sNode);
};
};  // namespace gql::parsers::schema::nodes
