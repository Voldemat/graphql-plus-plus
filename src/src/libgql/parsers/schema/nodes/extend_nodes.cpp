#include "./extend_nodes.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>
#include <vector>

#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/schema/nodes/extend_object_type.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
std::vector<std::pair<
    std::shared_ptr<ast::ObjectType>,
    std::map<std::string,
             std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>>>
parseServerExtendNodes(const std::vector<server::ast::FileNodes> &astArray,
                       const TypeRegistry &registry) {
    return astArray | std::views::transform([](const auto &ast) {
               return ast.extensions;
           }) |
           std::views::join |
           std::views::transform([&registry](const auto &node) {
               return nodes::parseExtendObjectType(node, registry);
           }) |
           std::ranges::to<std::vector>();
};
};  // namespace parsers::schema::nodes
