#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"
#include "./object_field_spec.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {

std::shared_ptr<ast::ObjectType> parseObject(
    const file::server::ast::ObjectDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.objects.at(node.name.name);
    obj->fields =
        node.fields |
        std::views::transform(
            [&registry](const server::ast::FieldDefinitionNode &defNode)
                -> std::pair<std::string, std::shared_ptr<ast::FieldDefinition<
                                              ast::ObjectFieldSpec>>> {
                const auto &[typeSpec, nullable] =
                    parseObjectFieldSpec(defNode, registry);

                return { defNode.name.name,
                         std::make_shared<
                             ast::FieldDefinition<ast::ObjectFieldSpec>>(
                             defNode.name.name, typeSpec, nullable) };
            }) |
        std::ranges::to<std::map>();
    obj->implements =
        node.interfaces |
        std::views::transform(
            [&registry](const shared::ast::NameNode &node)
                -> std::pair<std::string, std::shared_ptr<ast::Interface>> {
                return { node.name, registry.interfaces.at(node.name) };
            }) |
        std::ranges::to<std::map>();
    return obj;
};
};  // namespace parsers::schema::nodes
