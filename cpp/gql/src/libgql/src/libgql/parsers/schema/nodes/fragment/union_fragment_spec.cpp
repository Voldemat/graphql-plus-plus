#include "./union_fragment_spec.hpp"

#include <memory>
#include <ranges>
#include <vector>

#include "./union_selection_node.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {
ast::UnionFragmentSpec parseUnionFragmentSpec(
    const std::vector<client::ast::SelectionNode> &selections,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry) {
    return { .type = type,
             .selections =
                 selections |
                 std::views::transform([&registry, &type](const auto &sNode) {
                     return nodes::parseUnionSelectionNode(sNode, type,
                                                           registry);
                 }) |
                 std::ranges::to<std::vector>() };
};
};  // namespace gql::parsers::schema::nodes
