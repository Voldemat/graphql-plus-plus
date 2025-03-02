#include "./shared.hpp"

#include <memory>
#include <string>

#include "./ast.hpp"
#include "./parser_error.hpp"
#include "libgql/lexer/token.hpp"

void parsers::file::shared::assertIsNotKeyword(
    const lexer::GQLToken token,
    const std::shared_ptr<ast::SourceFile> &source) {
    if (isKeyword(token.lexeme))
        throw ParserError::identifierIsKeyword(token, source);
};

const bool parsers::file::shared::isKeyword(const std::string lexeme) {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input" ||
            lexeme == "extend" || lexeme == "directive");
};
