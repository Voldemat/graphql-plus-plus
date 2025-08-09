#include "./argument.hpp"

#include <algorithm>
#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "utils.hpp"

namespace gql::parsers::schema::nodes {
ast::ArgumentLiteralValue parseArgumentValue(
    const file::shared::ast::ArgumentValue &value,
    const std::shared_ptr<ast::FieldDefinition<ast::InputFieldSpec>> &type) {
    return std::visit<ast::ArgumentLiteralValue>(
        utils::overloaded{
            [&type](const file::shared::ast::NameNode &node) -> ast::ArgumentLiteralValue {
                throw file::shared::ParserError(node.location.startToken, "",
                                                node.location.source);
            },
            [&type](const file::shared::ast::LiteralNode &node) {
                return std::visit<ast::ArgumentLiteralValue>(
                    utils::overloaded{
                        [](const file::shared::ast::LiteralBooleanNode &n) {
                            return n.value;
                        },
                        [](const file::shared::ast::LiteralIntNode &n) {
                            return n.value;
                        },
                        [](const file::shared::ast::LiteralFloatNode &n) {
                            return n.value;
                        },
                        [](const file::shared::ast::LiteralStringNode &n) {
                            return n.value;
                        },
                        [&type](
                            const file::shared::ast::LiteralEnumValueNode &n) {
                            if (!std::holds_alternative<
                                    ast::LiteralFieldSpec<ast::InputTypeSpec>>(
                                    type->spec)) {
                                throw file::shared::ParserError(
                                    n.location.startToken, "Invalid type",
                                    n.location.source);
                            };
                            const auto &typeSpec = std::get<
                                ast::LiteralFieldSpec<ast::InputTypeSpec>>(
                                type->spec);
                            if (!std::holds_alternative<
                                    std::shared_ptr<ast::Enum>>(
                                    typeSpec.type)) {
                                throw file::shared::ParserError(
                                    n.location.startToken, "Invalid type",
                                    n.location.source);
                            };
                            const auto &gqlEnum =
                                std::get<std::shared_ptr<ast::Enum>>(
                                    typeSpec.type);
                            if (std::find(gqlEnum->values.begin(),
                                          gqlEnum->values.end(),
                                          n.value) == gqlEnum->values.end()) {
                                throw file::shared::ParserError(
                                    n.location.startToken, "Unknown enum value",
                                    n.location.source);
                            };
                            return (ast::ArgumentEnumValue){
                                .value = n.value,
                            };
                        } },
                    node);
            } },
        value);
};

std::map<std::string, ast::FieldSelectionArgument> parseArguments(
    const std::vector<file::shared::ast::Argument> &arguments,
    const std::shared_ptr<ast::ServerDirective> &directive,
    const TypeRegistry &registry) {
    return arguments |
           std::views::transform(
               [&registry, &directive](const file::shared::ast::Argument &arg) {
                   if (directive->arguments.contains(arg.name.name)) {
                       throw file::shared::ParserError(
                           arg.name.location.startToken, "Unknown argument",
                           arg.name.location.source);
                   };
                   const auto &type = directive->arguments[arg.name.name];
                   return std::make_pair(
                       arg.name.name,
                       (ast::FieldSelectionArgument){
                           .name = arg.name.name,
                           .value = parseArgumentValue(arg.value, type),
                           .type = type,
                       });
               }) |
           std::ranges::to<std::map>();
};
};  // namespace parsers::schema::nodes
