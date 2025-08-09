#pragma once

#include <memory>
#include <ranges>
#include <vector>

#include "../../../file/client/ast.hpp"
#include "../../client_ast.hpp"
#include "../../type_registry.hpp"
#include "./object_selection_node.hpp"

namespace gql::parsers::schema::nodes {
template <typename T>
ast::ObjectFragmentSpec<T> parseObjectFragmentSpec(
    const std::vector<file::client::ast::SelectionNode> &selections,
    const std::shared_ptr<T> &type, const TypeRegistry &registry) {
    const auto &selectionNodes =
        selections |
        std::views::transform([&registry, &type](const auto &sNode) {
            return nodes::parseObjectSelectionNode(sNode, type, registry);
        }) |
        std::ranges::to<std::vector>();
    return { .type = type, .selections = selectionNodes };
};
};  // namespace parsers::schema::nodes
