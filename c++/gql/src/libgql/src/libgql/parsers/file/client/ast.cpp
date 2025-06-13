#include "./ast.hpp"

#include <optional>
#include <string>

namespace parsers::file::client::ast {
std::optional<OpType> opTypeFromObjectName(const std::string &value) {
    if (value == "Query") return OpType::QUERY;
    if (value == "Mutation") return OpType::MUTATION;
    if (value == "Subscription") return OpType::SUBSCRIPTION;
    return std::nullopt;
};

std::optional<OpType> opTypeFromClientOp(const std::string &lexeme) {
    if (lexeme == "mutation") return ast::OpType::MUTATION;
    if (lexeme == "query") return ast::OpType::QUERY;
    if (lexeme == "subscription") return ast::OpType::SUBSCRIPTION;
    return std::nullopt;
};

std::optional<DirectiveLocation> stringToDirectiveLocation(
    const std::string &str) {
    if (str == "QUERY")
        return DirectiveLocation::QUERY;
    else if (str == "MUTATION")
        return DirectiveLocation::MUTATION;
    else if (str == "SUBSCRIPTION")
        return DirectiveLocation::SUBSCRIPTION;
    else if (str == "FIELD")
        return DirectiveLocation::FIELD;
    else if (str == "FRAGMENT_DEFINITION")
        return DirectiveLocation::FRAGMENT_DEFINITION;
    else if (str == "FRAGMENT_SPREAD")
        return DirectiveLocation::FRAGMENT_SPREAD;
    else if (str == "INLINE_FRAGMENT")
        return DirectiveLocation::INLINE_FRAGMENT;
    else if (str == "VARIABLE_DEFINITION")
        return DirectiveLocation::VARIABLE_DEFINITION;
    return std::nullopt;
};
};  // namespace parsers::file::client::ast
