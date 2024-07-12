#include "./union.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "../../file/server/ast.hpp"
#include "../../file/shared/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace parsers::schema::nodes {
std::shared_ptr<ast::Union> parseUnion(
    const file::server::ast::UnionDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.unions.at(node.name.name);
    obj->items =
        node.values |
        std::views::transform(
            [&registry](const file::shared::ast::NameNode &nNode)
                -> std::pair<std::string, std::shared_ptr<ast::ObjectType>> {
                return { nNode.name, registry.getObject(nNode.name) };
            }) |
        std::ranges::to<std::map>();
    return obj;
};
};  // namespace parsers::schema::nodes
