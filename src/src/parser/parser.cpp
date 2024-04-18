#include "parser/parser.hpp"

#include <string>
#include <vector>

#include "lexer/token.hpp"

using namespace parser;

Parser::Parser(const std::vector<GQLToken> tokens) noexcept
    : tokens{ tokens } {};

const ASTProgram Parser::getAstTree() noexcept {
    std::vector<ASTNode> nodes = {};
    // while (true) {
    //     if (!nodeOrError.has_value()) return nodes;
    //     const auto node = nodeOrError.value();
    //     if (std::holds_alternative<const ParserError>(node)) {
    //         return std::get<const ParserError>(node);
    //     };
    //     const auto n = std::get<const ASTNode>(node);
    //     nodes.push_back(n);
    // };
    return { .nodes = nodes };
};

const bool isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "mutation" || lexeme == "query"
            || lexeme == "input" || lexeme == "extend");
};
