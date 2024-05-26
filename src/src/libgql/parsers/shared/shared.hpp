#ifndef LIBGQL_PARSERS_SHARED
#define LIBGQL_PARSERS_SHARED

#include <exception>
#include <filesystem>
#include <format>
#include <memory>
#include <optional>
#include <string>
#include <variant>

#include "libgql/lexer/token.hpp"
namespace parsers {
namespace shared {
namespace ast {

struct SourceFile {
    std::filesystem::path filepath;
    std::string buffer;
};

struct NodeLocation {
    GQLToken startToken;
    GQLToken endToken;
    std::shared_ptr<SourceFile> source;
};

struct NameNode {
    NodeLocation location;
    std::string name;
};

struct LiteralIntNode {
    NodeLocation location;
    int value = 0;
};

struct LiteralFloatNode {
    NodeLocation location;
    float value = 0.0;
};

struct LiteralStringNode {
    NodeLocation location;
    std::string value;
};

struct LiteralBooleanNode {
    NodeLocation location;
    bool value = false;
};

struct LiteralEnumValueNode {
    NodeLocation location;
    std::string value;
};

using LiteralNode =
    std::variant<LiteralIntNode, LiteralFloatNode, LiteralStringNode,
                 LiteralBooleanNode, LiteralEnumValueNode>;

struct NamedTypeNode {
    NodeLocation location;
    NameNode name;
    bool nullable = true;
};

struct ListTypeNode {
    NodeLocation location;
    NamedTypeNode type;
    bool nullable = true;
};

using TypeNode = std::variant<NamedTypeNode, ListTypeNode>;

struct InputValueDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::TypeNode type;
    std::optional<shared::ast::LiteralNode> defaultValue;
};
};  // namespace ast

class ParserError : public std::exception {
    GQLToken token;
    std::string error;

public:
    [[nodiscard]] Location getLocation() const noexcept {
        return token.location;
    };
    explicit ParserError(const GQLToken t, const std::string e)
        : token{ t }, error{ e } {};
    [[nodiscard]] const char *what() const noexcept override {
        return error.c_str();
    };
    const static ParserError createEOF(const GQLToken token) noexcept {
        return ParserError(token, "EOF");
    };

    const static ParserError wrongType(
        const GQLToken token, const GQLTokenType expectedType) noexcept {
        return ParserError(token, std::string("Expected ") +
                                      gqlTokenTypeToString(expectedType) +
                                      " type, got " +
                                      gqlTokenTypeToString(token.type) +
                                      ", at: " + (std::string)token.location);
    };

    const static ParserError identifierIsKeyword(
        const GQLToken token) noexcept {
        return ParserError(token, token.lexeme + " is reserved keyword");
    };

    const static ParserError unexpectedIdentifier(
        const GQLToken token) noexcept {
        return ParserError(token,
                           "Unexpected identifier: \"" + token.lexeme + "\"");
    };

    const static ParserError wrongLexeme(const GQLToken &token,
                                         const std::string &expectedLexeme) {
        return ParserError(token, std::format(R"((Expected: "{}", got "{}"))",
                                              expectedLexeme, token.lexeme));
    };
};

void assertIsNotKeyword(const GQLToken token);
const bool isKeyword(const std::string lexeme) noexcept;
};  // namespace shared
};  // namespace parsers

#endif