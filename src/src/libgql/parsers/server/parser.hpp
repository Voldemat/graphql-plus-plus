#ifndef GRAPHQL_SERVER_PARSER
#define GRAPHQL_SERVER_PARSER

#include <exception>
#include <memory>
#include <string>
#include <vector>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"

namespace parsers {
namespace server {
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
};

class Parser {
    unsigned int index = 0;
    std::vector<GQLToken> tokens;
    std::shared_ptr<ast::SourceFile> source;
    GQLToken currentToken;
    const GQLToken lookahead();
    void advance();
    void consume(const GQLTokenType expectedType);
    void consumeIdentifier();
    bool consumeIfIsAhead(GQLTokenType expectedType);
    bool isAhead(GQLTokenType expectedType);
    ast::NameNode parseNameNode(bool raiseOnKeyword = false);
    ast::ScalarDefinitionNode parseScalarTypeDefinitionNode();
    ast::UnionDefinitionNode parseUnionTypeDefinitionNode();
    ast::ASTNode parseASTNode();
    ast::ExtendTypeNode parseExtendTypeNode();
    ast::EnumDefinitionNode parseEnumTypeDefinitionNode();
    ast::EnumValueDefinitionNode parseEnumValueDefinitionNode();
    ast::InterfaceDefinitionNode parseInterfaceTypeDefinitionNode();
    ast::FieldDefinitionNode parseFieldDefinitionNode();
    ast::TypeNode parseTypeNode();
    ast::NamedTypeNode parseNamedTypeNode();
    ast::ListTypeNode parseListTypeNode();
    ast::InputValueDefinitionNode parseInputValueDefinitionNode();
    ast::LiteralNode parseLiteralNode();
    ast::ObjectDefinitionNode parseObjectTypeDefinitionNode();

public:
    Parser(std::vector<GQLToken> tokens,
           std::shared_ptr<ast::SourceFile> source) noexcept;
    ast::FileNodes parse();
};
void assertIsNotKeyword(const GQLToken token);
const bool isKeyword(const std::string lexeme) noexcept;
};  // namespace server
};  // namespace parsers
#endif
