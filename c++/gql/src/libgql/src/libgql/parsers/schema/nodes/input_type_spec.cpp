#include "./input_type_spec.hpp"

#include <optional>
#include <utility>
#include <variant>

#include "./literal.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {

std::pair<ast::NonCallableFieldSpec<ast::InputTypeSpec>, bool>
parseNonCallableInputTypeSpec(
    const shared::ast::TypeNode astNode,
    const std::optional<shared::ast::LiteralNode> defaultValueNode,
    const TypeRegistry &registry) {
    return std::visit<
        std::pair<ast::NonCallableFieldSpec<ast::InputTypeSpec>, bool>>(
        overloaded{
            [&registry,
             &defaultValueNode](const shared::ast::NamedTypeNode &node)
                -> std::pair<ast::LiteralFieldSpec<ast::InputTypeSpec>, bool> {
                const auto &type = registry.getTypeForInput(node.name);
                return { (ast::LiteralFieldSpec<ast::InputTypeSpec>){
                             { .defaultValue = defaultValueNode.transform(
                                   [&type](const auto &literal) {
                                       return nodes::parseLiteralNode(literal,
                                                                      type);
                                   }) },
                             .type = type },
                         node.nullable };
            },
            [&registry](const shared::ast::ListTypeNode &node)
                -> std::pair<ast::ArrayFieldSpec<ast::InputTypeSpec>, bool> {
                return { (ast::ArrayFieldSpec<ast::InputTypeSpec>){
                             .type = registry.getTypeForInput(node.type.name),
                             .nullable = node.type.nullable },
                         node.nullable };
            } },
        astNode);
};

std::pair<ast::InputFieldSpec, bool> parseInputTypeSpec(
    const server::ast::FieldDefinitionNode &astNode,
    const TypeRegistry &registry) {
    const auto &[returnType, nullable] =
        parseNonCallableInputTypeSpec(astNode.type, std::nullopt, registry);
    ast::InputFieldSpec returnTypeSpec = std::visit(
        [](auto &&arg) -> ast::InputFieldSpec { return arg; }, returnType);

    return { returnTypeSpec, nullable };
};
};  // namespace parsers::schema::nodes
