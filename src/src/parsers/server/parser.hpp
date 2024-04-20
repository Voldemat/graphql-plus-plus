#ifndef GRAPHQL_PARSER
#define GRAPHQL_PARSER

#include <exception>
#include <map>
#include <string>
#include <variant>
#include <vector>

#include "lexer/token.hpp"

namespace parsers {
namespace server {
enum class ASTGQLSimpleType { STRING, INT, FLOAT, BOOLEAN };
struct ASTGQLReferenceType {
    std::string name;
};

using ASTGQLType = std::variant<ASTGQLSimpleType, ASTGQLReferenceType>;

struct ASTTypeSpec {
    ASTGQLType type;
    bool nullable;
};

struct ASTTypeDefinition {
    const std::string name;
    std::map<std::string, ASTTypeSpec> fields;
    const bool isInput;
};

struct ASTExtendNode {
    const ASTTypeDefinition type;
};

struct ASTEnumNode {
    const std::string name;
    const std::vector<std::string> items;
};

struct ASTUnionNode {
    const std::string name;
    const std::vector<ASTGQLReferenceType> items;
};

using ASTNode = std::variant<ASTTypeDefinition, ASTTypeSpec, ASTExtendNode,
                             ASTUnionNode, ASTEnumNode>;

struct ASTProgram {
    const std::vector<ASTNode> nodes;
};

class ParserError : public std::exception {
    const GQLToken token;
    const std::string error;
    explicit ParserError(const GQLToken t, const std::string e)
        : token{ t }, error{ e } {};

public:
    const char *what() const noexcept { return error.c_str(); };
    const static ParserError createEOF(const GQLToken token) noexcept {
        return ParserError(token, "EOF");
    };

    const static ParserError wrongType(
        const GQLToken token, const GQLTokenType expectedType) noexcept {
        return ParserError(
            token, std::string("Expected ") + gqlTokenTypeToString(expectedType)
                       + " type, got " + gqlTokenTypeToString(token.type));
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
    const ASTNode parseNode();
    const ASTTypeSpec getTypeSpec();
    const GQLToken lookahead();
    void consume(const GQLTokenType expectedType);
    const ASTNode parseComplexToken();
    const ASTTypeDefinition parseTypeNode(bool isInput);
    const ASTEnumNode parseEnumNode();
    const ASTUnionNode parseUnionNode();
    const ASTExtendNode parseExtendNode();
    const ASTGQLType parseGQLType();
    const std::string parseIdentifier();
    const ASTTypeSpec parseTypeSpecNode();
    void consumeIdentifier();

public:
    Parser(std::vector<GQLToken> tokens) noexcept;
    const ASTProgram getAstTree();
};
void assertIsNotKeyword(const GQLToken token);
const bool isKeyword(const std::string lexeme) noexcept;
};  // namespace server
};  // namespace parsers
#endif
