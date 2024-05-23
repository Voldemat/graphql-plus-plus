#include "./shared.hpp"

#include <string>

#include "libgql/lexer/token.hpp"

void parsers::shared::assertIsNotKeyword(const GQLToken token) {
    if (isKeyword(token.lexeme)) throw ParserError::identifierIsKeyword(token);
};

const bool parsers::shared::isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input" ||
            lexeme == "extend");
};
