#include "parsers/server/parser.hpp"

#include <map>
#include <string>
#include <variant>
#include <vector>

#include "lexer/token.hpp"

using namespace parsers::server;

std::string parsers::server::astGQLSimpleTypeToString(
    const ASTGQLSimpleType &type) noexcept {
    switch (type) {
        case parsers::server::ASTGQLSimpleType::INT:
            return "Int";
        case parsers::server::ASTGQLSimpleType::FLOAT:
            return "Float";
        case parsers::server::ASTGQLSimpleType::STRING:
            return "String";
        case parsers::server::ASTGQLSimpleType::BOOLEAN:
            return "Boolean";
    };
};
Parser::Parser(std::vector<GQLToken> tokens) noexcept
    : tokens{ tokens }, currentToken{ tokens[0] } {};

const ASTProgram Parser::getAstTree() {
    std::vector<ASTNode> nodes = {};
    while (index + 1 < tokens.size()) {
        nodes.push_back(parseNode());
    };
    return { .nodes = nodes };
};

const ASTNode Parser::parseNode() {
    if (index != 0) {
        consume(ComplexTokenType::IDENTIFIER);
    };
    return parseComplexToken();
};

const GQLToken Parser::lookahead() { return tokens[index + 1]; };

void Parser::consume(const GQLTokenType type) {
    index += 1;
    currentToken = tokens[index];
    if (currentToken.type != type) {
        throw ParserError::wrongType(currentToken, type);
    };
};

const ASTNode Parser::parseComplexToken() {
    const auto type = std::get<ComplexTokenType>(currentToken.type);
    if (type != ComplexTokenType::IDENTIFIER) {
        throw ParserError::wrongType(currentToken,
                                     ComplexTokenType::IDENTIFIER);
    };
    if (currentToken.lexeme == "type") {
        return parseTypeNode(false);
    } else if (currentToken.lexeme == "input") {
        return parseTypeNode(true);
    } else if (currentToken.lexeme == "extend") {
        consume(ComplexTokenType::IDENTIFIER);
        const auto typeSpec = parseTypeNode(false);
        return (ASTExtendNode){ .type = typeSpec };
    } else if (currentToken.lexeme == "enum") {
        return parseEnumNode();
    } else if (currentToken.lexeme == "union") {
        return parseUnionNode();
    };
    throw ParserError::unexpectedIdentifier(currentToken);
};

const ASTEnumNode Parser::parseEnumNode() {
    consumeIdentifier();
    const std::string name = currentToken.lexeme;
    std::vector<std::string> items;
    consume(SimpleTokenType::LEFT_BRACE);
    while (lookahead().type != (GQLTokenType)SimpleTokenType::RIGHT_BRACE) {
        consume(ComplexTokenType::IDENTIFIER);
        assertIsNotKeyword(currentToken);
        items.push_back(currentToken.lexeme);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .name = name, .items = items };
};

const ASTUnionNode Parser::parseUnionNode() {
    consumeIdentifier();
    const std::string name = currentToken.lexeme;
    consume(SimpleTokenType::EQUAL);
    consumeIdentifier();
    std::vector<ASTGQLReferenceType> items;
    items.push_back({ .name = currentToken.lexeme });
    while (lookahead().type == (GQLTokenType)SimpleTokenType::VSLASH) {
        consume(SimpleTokenType::VSLASH);
        consumeIdentifier();
        items.push_back({ .name = currentToken.lexeme });
    };
    return { .name = name, .items = items };
};

void Parser::consumeIdentifier() {
    consume(ComplexTokenType::IDENTIFIER);
    assertIsNotKeyword(currentToken);
};

const ASTTypeDefinition Parser::parseTypeNode(bool isInput) {
    consumeIdentifier();
    const std::string name = currentToken.lexeme;
    consume(SimpleTokenType::LEFT_BRACE);
    std::map<std::string, ASTTypeSpec> definitions;
    while (lookahead().type != (GQLTokenType)SimpleTokenType::RIGHT_BRACE) {
        const std::string key = parseIdentifier();
        consume(SimpleTokenType::COLON);
        definitions[key] = parseTypeSpecNode();
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .name = name, .fields = definitions, .isInput = isInput };
};

const std::string Parser::parseIdentifier() {
    consumeIdentifier();
    return currentToken.lexeme;
};

const ASTTypeSpec Parser::parseTypeSpecNode() {
    if (lookahead().type == (GQLTokenType)SimpleTokenType::LEFT_BRACKET) {
        return parseArrayTypeSpecNode();
    } else {
        return parseTrivialTypeSpecNode();
    };
};
const ASTArrayTypeSpec Parser::parseArrayTypeSpecNode() {
    consume(SimpleTokenType::LEFT_BRACKET);
    const auto& trivialType = parseTrivialTypeSpecNode();
    consume(SimpleTokenType::RIGHT_BRACKET);
    bool nullable = true;
    if (lookahead().type == (GQLTokenType)SimpleTokenType::BANG) {
        consume(SimpleTokenType::BANG);
        nullable = false;
    };
    return { .type = trivialType, .nullable = nullable };
};
const ASTTrivialTypeSpec Parser::parseTrivialTypeSpecNode() {
    const ASTGQLType type = parseGQLType();
    const bool nullable
        = lookahead().type != (GQLTokenType)SimpleTokenType::BANG;
    if (!nullable) consume(SimpleTokenType::BANG);
    return { .type = type, .nullable = nullable };
};

const ASTGQLType Parser::parseGQLType() {
    consumeIdentifier();
    if (currentToken.lexeme == "Int")
        return ASTGQLSimpleType::INT;
    else if (currentToken.lexeme == "Float")
        return ASTGQLSimpleType::FLOAT;
    else if (currentToken.lexeme == "String")
        return ASTGQLSimpleType::STRING;
    else if (currentToken.lexeme == "Boolean")
        return ASTGQLSimpleType::BOOLEAN;
    return (ASTGQLReferenceType){ .name = currentToken.lexeme };
};

void parsers::server::assertIsNotKeyword(const GQLToken token) {
    if (isKeyword(token.lexeme)) throw ParserError::identifierIsKeyword(token);
};

const bool parsers::server::isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input"
            || lexeme == "extend");
};