#include "./client_operation.hpp"

#include <cstddef>
#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/shared.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/nodes/fragment/spec.hpp"
#include "libgql/parsers/schema/nodes/input_field_definition.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {

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

    return std::make_shared<ast::Operation>(
        definition.type, definition.name.name, parameters, node,
        shared::getSourceText(definition.location.source->buffer,
                              definition.location.startToken.location,
                              definition.location.endToken.location),
        0);
};
};  // namespace gql::parsers::schema::nodes
