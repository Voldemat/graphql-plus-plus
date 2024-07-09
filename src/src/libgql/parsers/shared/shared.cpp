#include "./shared.hpp"

#include <memory>
#include <string>

#include "libgql/lexer/token.hpp"

void parsers::shared::assertIsNotKeyword(
    const lexer::GQLToken token,
    const std::shared_ptr<ast::SourceFile> &source) {
    if (isKeyword(token.lexeme))
        throw ParserError::identifierIsKeyword(token, source);
};

const bool parsers::shared::isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input" ||
            lexeme == "extend");
};
