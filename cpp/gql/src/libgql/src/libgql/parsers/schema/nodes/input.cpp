#include "./input.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "../../file/server/ast.hpp"
#include "../../file/shared/ast.hpp"
#include "../server_ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "./input_type_spec.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {

std::shared_ptr<ast::InputType> parseInput(
    const server::ast::InputObjectDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.inputs.at(node.name.name);
    obj->fields =
        node.fields |
        std::views::transform(
            [&registry](const shared::ast::InputFieldDefinitionNode &defNode)
                -> std::pair<std::string,
                             ast::FieldDefinition<ast::InputFieldSpec>> {
                const auto &[typeSpec, nullable] =
                    nodes::parseInputTypeSpec(defNode, registry);
                return { defNode.name.name,
                         { .name = defNode.name.name,
                           .spec = typeSpec,
                           .nullable = nullable } };
            }) |
        std::ranges::to<std::map>();
    return obj;
};

};  // namespace gql::parsers::schema::nodes
