#pragma once

#include <exception>
#include <format>
#include <memory>
#include <string>

#include "./ast.hpp"
#include "libgql/lexer/location.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

namespace gql::parsers::file::shared {
class ParserError : public std::exception {
    lexer::GQLToken token;
    std::string error;
    std::shared_ptr<const ast::SourceFile> source;

public:
    explicit ParserError(const lexer::GQLToken t, const std::string e,
                         const std::shared_ptr<const ast::SourceFile> &source)
        : token{ t }, error{ e }, source{ source } {};

    inline const char *what() const noexcept override { return error.c_str(); };

    inline const std::shared_ptr<const ast::SourceFile> getSource() const {
        return source;
    };

    inline lexer::Location getLocation() const { return token.location; };

    static ParserError createEOF(
        const lexer::GQLToken token,
        const std::shared_ptr<const ast::SourceFile> &source) noexcept {
        return ParserError(token, "EOF", source);
    };

    static ParserError wrongType(
        const lexer::GQLToken token, const lexer::GQLTokenType expectedType,
        const std::shared_ptr<const ast::SourceFile> &source) noexcept {
        return ParserError(
            token,
            std::string("Expected ") + gqlTokenTypeToString(expectedType) +
                " type, got " + gqlTokenTypeToString(token.type) +
                ", at: " + (std::string)token.location,
            source);
    };

    static ParserError identifierIsKeyword(
        const lexer::GQLToken token,
        const std::shared_ptr<const ast::SourceFile> &source) noexcept {
        return ParserError(token, token.lexeme + " is reserved keyword",
                           source);
    };

    static ParserError unexpectedIdentifier(
        const lexer::GQLToken token,
        const std::shared_ptr<const ast::SourceFile> &source) noexcept {
        return ParserError(
            token, "Unexpected identifier: \"" + token.lexeme + "\"", source);
    };

    static ParserError wrongLexeme(
        const lexer::GQLToken &token, const std::string &expectedLexeme,
        const std::shared_ptr<ast::SourceFile> &source) {
        return ParserError(token,
                           std::format(R"((Expected: "{}", got "{}"))",
                                       expectedLexeme, token.lexeme),
                           source);
    };
};
};  // namespace gql::parsers::file::shared
