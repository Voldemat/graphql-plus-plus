#include "./ast.hpp"

#include <optional>
#include <string>

using namespace parsers::file::client::ast;

std::optional<OpType> parsers::file::client::ast::opTypeFromObjectName(
    const std::string &value) {
    if (value == "Query") return OpType::QUERY;
    if (value == "Mutation") return OpType::MUTATION;
    if (value == "Subscription") return OpType::SUBSCRIPTION;
    return std::nullopt;
};

std::optional<OpType> parsers::file::client::ast::opTypeFromClientOp(
    const std::string &lexeme) {
    if (lexeme == "mutation") return ast::OpType::MUTATION;
    if (lexeme == "query") return ast::OpType::QUERY;
    if (lexeme == "subscription") return ast::OpType::SUBSCRIPTION;
    return std::nullopt;
};
