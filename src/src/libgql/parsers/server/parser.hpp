#ifndef GRAPHQL_PARSER
#define GRAPHQL_PARSER

#include <exception>
#include <optional>
#include <string>
#include <vector>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"

namespace parsers {
namespace server {
class ParserError : public std::exception {
    const GQLToken token;
    const std::string error;

public:
    explicit ParserError(const GQLToken t, const std::string e)
        : token{ t }, error{ e } {};
    const char *what() const noexcept { return error.c_str(); };
    const static ParserError createEOF(const GQLToken token) noexcept {
        return ParserError(token, "EOF");
    };

    const static ParserError wrongType(
        const GQLToken token, const GQLTokenType expectedType) noexcept {
        return ParserError(
            token, std::string("Expected ") + gqlTokenTypeToString(expectedType)
                       + " type, got " + gqlTokenTypeToString(token.type)
                       + ", at: " + (std::string)token.location);
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
    const std::vector<GQLToken> tokens;
    GQLToken currentToken;
    const ast::ASTNode parseNode();
    const ast::ASTTrivialTypeSpec getTypeSpec();
    const GQLToken lookahead();
    void consume(const GQLTokenType expectedType);
    const ast::ASTNode parseComplexToken();
    const ast::ASTTypeDefinition parseTypeNode(bool isInput);
    const ast::ASTEnumNode parseEnumNode();
    const ast::ASTUnionNode parseUnionNode();
    const ast::ASTExtendNode parseExtendNode();
    const ast::ASTGQLType parseGQLType();
    const std::string parseIdentifier();
    void consumeIdentifier();
    const ast::ASTTypeSpec parseTypeSpecNode();
    const ast::ASTCallableTypeSpec parseCallableTypeSpecNode();
    const ast::ASTLiteralTypeSpec parseLiteralTypeSpecNode();
    const ast::ASTLiteral parseLiteralNode(ast::ASTGQLSimpleType type);
    const std::optional<ast::ASTLiteral> maybeParseLiteralNode(ast::ASTGQLSimpleType type);
    const ast::ASTArrayTypeSpec parseArrayTypeSpecNode();
    const std::optional<ast::ASTArrayLiteral> maybeParseArrayLiteralNode(ast::ASTGQLSimpleType type);
    const ast::ASTArrayLiteral parseArrayLiteralNode(ast::ASTGQLSimpleType type);
    const ast::ASTTrivialTypeSpec parseTrivialTypeSpecNode();
    void maybeConsumeComma();

public:
    Parser(std::vector<GQLToken> tokens) noexcept;
    const ast::ASTProgram getAstTree();
};
void assertIsNotKeyword(const GQLToken token);
const bool isKeyword(const std::string lexeme) noexcept;
};  // namespace server
};  // namespace parsers
#endif
