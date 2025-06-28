#include "./client_operation.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/nodes/fragment/spec.hpp"
#include "libgql/parsers/schema/nodes/input_field_definition.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
std::shared_ptr<ast::Operation> parseClientOperationDefinition(
    const client::ast::OperationDefinition &definition,
    const TypeRegistry &registry) {
    const auto &fragment = registry.fragmentSpecFromOpType(definition.type);
    const auto &parameters =
        definition.parameters | std::views::values |
        std::views::transform(
            [&registry](const auto &param)
                -> std::pair<std::string,
                             ast::FieldDefinition<ast::InputFieldSpec>> {
                return { param.name.name,
                         parseInputFieldDefinition(param, registry) };
            }) |
        std::ranges::to<std::map>();
    const auto &node =
        parseFragmentSpec(definition.fragment, fragment, registry);
    const auto &objectNode =
        std::get<ast::ObjectFragmentSpec<ast::ObjectType>>(node);
    const auto &sNode = std::get<ast::FieldSelection>(objectNode.selections[0]);

    return std::make_shared<ast::Operation>(
        definition.type, definition.name.name, parameters, node);
};
};  // namespace parsers::schema::nodes
