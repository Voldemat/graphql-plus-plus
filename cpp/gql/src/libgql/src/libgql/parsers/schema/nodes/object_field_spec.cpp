#include "./object_field_spec.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "../../file/server/ast.hpp"
#include "../../file/shared/ast.hpp"
#include "../nodes/input_field_definition.hpp"
#include "../server_ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "libgql/parsers/schema/nodes/server_directive_invocation.hpp"
#include "utils.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {

std::pair<ast::NonCallableFieldSpec<ast::ObjectTypeSpec>, bool>
parseNonCallableObjectTypeSpec(
    const shared::ast::TypeNode astNode,
    const std::vector<ast::ServerDirectiveInvocation> &directives,
    const TypeRegistry &registry) {
    return std::visit<
        std::pair<ast::NonCallableFieldSpec<ast::ObjectTypeSpec>, bool>>(
        utils::overloaded{
            [&registry, &directives](const shared::ast::NamedTypeNode &node)
                -> std::pair<ast::LiteralFieldSpec<ast::ObjectTypeSpec>, bool> {
                ast::LiteralFieldSpec<ast::ObjectTypeSpec> spec = {
                    .type = registry.getTypeForObject(node.name),
                };
                spec.invocations = directives;
                return { spec, node.nullable };
            },
            [&registry, &directives](const shared::ast::ListTypeNode &node)
                -> std::pair<ast::ArrayFieldSpec<ast::ObjectTypeSpec>, bool> {
                ast::ArrayFieldSpec<ast::ObjectTypeSpec> spec = {
                    .type = registry.getTypeForObject(node.type.name),
                    .nullable = node.type.nullable
                };
                spec.invocations = directives;
                return { spec, node.nullable };
            } },
        astNode);
};

std::pair<ast::ObjectFieldSpec, bool> parseObjectFieldSpec(
    const file::server::ast::FieldDefinitionNode &astNode,
    const TypeRegistry &registry) {
    const auto &directives =
        parseServerDirectiveInvocations(astNode.directives, registry);
    const auto &[returnType, nullable] =
        parseNonCallableObjectTypeSpec(astNode.type, directives, registry);
    ast::ObjectFieldSpec returnTypeSpec = std::visit(
        [](auto &&arg) -> ast::ObjectFieldSpec { return arg; }, returnType);
    if (astNode.arguments.empty()) return { returnTypeSpec, nullable };
    const auto &callableSpec = (ast::CallableFieldSpec){
        .returnType = returnType,
        .arguments =
            astNode.arguments |
            std::views::transform(
                [&registry](
                    const file::shared::ast::InputValueDefinitionNode &node)
                    -> std::pair<std::string,
                                 std::shared_ptr<ast::FieldDefinition<
                                     ast::InputFieldSpec>>> {
                    return { node.name.name,
                             std::make_shared<
                                 ast::FieldDefinition<ast::InputFieldSpec>>(
                                 parseInputFieldDefinition(node, registry)) };
                }) |
            std::ranges::to<std::map>()
    };
    return { callableSpec, nullable };
};

ast::ObjectTypeSpec getReturnTypeFromNonCallableFieldSpec(
    const ast::NonCallableFieldSpec<ast::ObjectTypeSpec> &fSpec) {
    return std::visit<ast::ObjectTypeSpec>(
        utils::overloaded{
            [](const auto &node) -> ast::ObjectTypeSpec { return node.type; } },
        fSpec);
};

ast::ObjectTypeSpec getReturnTypeFromObjectFieldSpec(
    const ast::ObjectFieldSpec &spec) {
    return std::visit<ast::ObjectTypeSpec>(
        utils::overloaded{
            [](const ast::LiteralFieldSpec<ast::ObjectTypeSpec> &node) {
                return node.type;
            },
            [](const ast::CallableFieldSpec &node) {
                return getReturnTypeFromNonCallableFieldSpec(node.returnType);
            },
            [](const ast::ArrayFieldSpec<ast::ObjectTypeSpec> &node) {
                return node.type;
            } },
        spec);
};
};  // namespace gql::parsers::schema::nodes
