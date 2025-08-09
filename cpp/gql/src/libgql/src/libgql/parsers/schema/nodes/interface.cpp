#include "./interface.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "../../file/server/ast.hpp"
#include "../nodes/object_field_spec.hpp"
#include "../server_ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {
std::shared_ptr<ast::Interface> parseInterface(
    const server::ast::InterfaceDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.interfaces.at(node.name.name);
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
    return obj;
};
};  // namespace gql::parsers::schema::nodes
