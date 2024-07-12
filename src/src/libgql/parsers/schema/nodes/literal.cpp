#include "./literal.hpp"

#include <algorithm>
#include <format>
#include <memory>
#include <variant>

#include "../../file/shared/ast.hpp"
#include "../../file/shared/parser_error.hpp"
#include "../server_ast.hpp"
#include "utils.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
ast::Literal parseLiteralNode(const shared::ast::LiteralNode &literal,
                              const ast::InputTypeSpec &spec) {
    return std::visit<ast::Literal>(
        overloaded{
            [&spec](const shared::ast::LiteralEnumValueNode &node) {
                if (!std::holds_alternative<std::shared_ptr<ast::Enum>>(spec)) {
                    throw shared::ParserError(
                        node.location.startToken,
                        "Unexpected default value identifier",
                        node.location.source);
                };
                const auto &type = std::get<std::shared_ptr<ast::Enum>>(spec);
                if (std::find(type->values.begin(), type->values.end(),
                              node.value) == type->values.end()) {
                    throw file::shared::ParserError(
                        node.location.startToken,
                        std::format("Enum {} doesn`t have value {}", type->name,
                                    node.value),
                        node.location.source);
                };
                return node.value;
            },
            [](const auto &node) -> ast::Literal { return node.value; } },
        literal);
};
};  // namespace parsers::schema::nodes
