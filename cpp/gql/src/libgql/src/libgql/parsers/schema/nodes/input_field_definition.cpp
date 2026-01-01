#include "./input_field_definition.hpp"

#include <optional>
#include <variant>

#include "../../file/shared/ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "./input_type_spec.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {
ast::FieldDefinition<ast::InputFieldSpec> parseInputFieldDefinition(
    const shared::ast::InputFieldDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &[returnType, nullable] =
        parseNonCallableInputTypeSpec(node.type, node.defaultValue, registry);
    const ast::InputFieldSpec &returnTypeSpec = std::visit(
        [](auto &&arg) -> ast::InputFieldSpec { return arg; }, returnType);

    return { .name = node.name.name,
             .spec = returnTypeSpec,
             .nullable = nullable };
};
};  // namespace parsers::schema::nodes
