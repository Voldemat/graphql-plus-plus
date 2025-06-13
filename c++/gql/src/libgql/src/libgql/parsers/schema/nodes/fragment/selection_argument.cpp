#include "./selection_argument.hpp"

#include <algorithm>
#include <memory>
#include <string>
#include <variant>

#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "utils.hpp"

namespace parsers::schema::nodes {

bool isValidScalarType(const ast::InputTypeSpec &typeSpec,
                       const std::string &scalarTypeName) {
    if (std::holds_alternative<std::shared_ptr<ast::Scalar>>(typeSpec)) {
        const auto &scalar = std::get<std::shared_ptr<ast::Scalar>>(typeSpec);
        return scalar->name == scalarTypeName;
    };
    return false;
};

bool isValidEnumValue(const std::string &enumValue,
                      const ast::InputTypeSpec &typeSpec) {
    if (std::holds_alternative<std::shared_ptr<ast::Enum>>(typeSpec)) {
        const auto &gqlEnum = std::get<std::shared_ptr<ast::Enum>>(typeSpec);
        return std::find(gqlEnum->values.begin(), gqlEnum->values.end(),
                         enumValue) != gqlEnum->values.end();
    };
    return false;
};
ast::ArgumentLiteralValue parseArgumentLiteralValue(
    const file::shared::ast::LiteralNode &literalNode,
    const ast::InputTypeSpec &typeSpec) {
    return std::visit<ast::ArgumentLiteralValue>(
        overloaded{
            [&typeSpec](const file::shared::ast::LiteralIntNode &node)
                -> ast::ArgumentLiteralValue {
                const auto &isValid = isValidScalarType(typeSpec, "Int");
                if (!isValid)
                    throw file::shared::ParserError(
                        node.location.startToken,
                        "Invalid literal type, expected Int",
                        node.location.source);
                return node.value;
            },
            [&typeSpec](const file::shared::ast::LiteralBooleanNode &node)
                -> ast::ArgumentLiteralValue {
                const auto &isValid = isValidScalarType(typeSpec, "Boolean");
                if (!isValid)
                    throw file::shared::ParserError(
                        node.location.startToken,
                        "Invalid literal type, expected Boolean",
                        node.location.source);
                return node.value;
            },
            [&typeSpec](const file::shared::ast::LiteralStringNode &node)
                -> ast::ArgumentLiteralValue {
                const auto &isValid = isValidScalarType(typeSpec, "String");
                if (!isValid)
                    throw file::shared::ParserError(
                        node.location.startToken,
                        "Invalid literal type, expected String",
                        node.location.source);
                return node.value;
            },
            [&typeSpec](const file::shared::ast::LiteralFloatNode &node)
                -> ast::ArgumentLiteralValue {
                const auto &isValid = isValidScalarType(typeSpec, "Float");
                if (!isValid)
                    throw file::shared::ParserError(
                        node.location.startToken,
                        "Invalid literal type, expected Float",
                        node.location.source);
                return node.value;
            },
            [
             &typeSpec](const file::shared::ast::LiteralEnumValueNode &node)
                -> ast::ArgumentLiteralValue {
                const auto &isValid = isValidEnumValue(node.value, typeSpec);
                if (!isValid)
                    throw file::shared::ParserError(
                        node.location.startToken,
                        "Invalid enum value",
                        node.location.source);
                return (ast::ArgumentEnumValue){ .value = node.value };
            } },
        literalNode);
};
ast::FieldSelectionArgument parseSelectionArgument(
    const file::shared::ast::Argument &node,
    const ast::CallableFieldSpec &spec) {
    if (!spec.arguments.contains(node.name.name)) {
        throw file::shared::ParserError(
            node.name.location.startToken,
            "Argument with this name does not exists",
            node.name.location.source);
    };
    const auto &type = spec.arguments.at(node.name.name);
    const auto &typeSpec = ast::extractInputTypeSpec(type->spec);
    return { .name = node.name.name,
             .value = std::visit<ast::ArgumentValue>(
                 overloaded{
                     [](const file::shared::ast::NameNode &node) {
                         return (ast::ArgumentRefValue){ .name = node.name };
                     },
                     [&typeSpec](const file::shared::ast::LiteralNode &node) {
                         return parseArgumentLiteralValue(node, typeSpec);
                     } },
                 node.value),
             .type = type };
};
};  // namespace parsers::schema::nodes
