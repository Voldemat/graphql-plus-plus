#pragma once

#include <memory>
#include <optional>
#include <variant>

#include "../../../file/client/ast.hpp"
#include "../../client_ast.hpp"
#include "../../server_ast.hpp"
#include "../../type_registry.hpp"

namespace gql::parsers::schema::nodes {
ast::SpreadSelection parseSpreadSelectionNode(
    const file::client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry);

std::optional<
    std::variant<std::shared_ptr<ast::ObjectType>, std::shared_ptr<ast::Union>>>
getTypeForUnionConditionalSelection(
    const file::client::ast::ConditionalSpreadSelectionNode &node,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry);

ast::UnionSelection parseUnionSelectionNode(
    const file::client::ast::SelectionNode &sNode,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry);
};  // namespace gql::parsers::schema::nodes
