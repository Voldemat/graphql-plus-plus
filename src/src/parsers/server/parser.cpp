#include "parsers/server/parser.hpp"

#include <iostream>
#include <map>
#include <string>
#include <variant>
#include <vector>

#include "lexer/token.hpp"

using namespace parsers::server;

Parser::Parser(std::vector<GQLToken> tokens) noexcept
    : tokens{ tokens }, currentToken{ tokens[0] } {
    };

const ASTProgram Parser::getAstTree() {
    std::vector<ASTNode> nodes = {};
    while (index + 1 < tokens.size()) {
        std::cout << "Index: " << index << ", size: " << tokens.size() << std::endl;
        nodes.push_back(parseNode());
    };
    return { .nodes = nodes };
};

const ASTNode Parser::parseNode() {
    return parseComplexToken();
};

const GQLToken Parser::lookahead() {
    return tokens[index + 1];
};

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

const ASTEnumNode Parser::parseEnumNode(){
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
    const ASTGQLType type = parseGQLType();
    const bool nullable = lookahead().type != (GQLTokenType)SimpleTokenType::BANG;
    if (!nullable) consume(SimpleTokenType::BANG);
    return { .type = type, .nullable = nullable };
};

const ASTGQLType Parser::parseGQLType() {
    consumeIdentifier();
    std::cout << "parseGQLType: " << currentToken << std::endl;
    if (currentToken.lexeme == "Int") return ASTGQLSimpleType::INT;
    else if (currentToken.lexeme == "Float") return ASTGQLSimpleType::FLOAT;
    else if (currentToken.lexeme == "String") return ASTGQLSimpleType::STRING;
    else if (currentToken.lexeme == "Boolean") return ASTGQLSimpleType::BOOLEAN;
    return (ASTGQLReferenceType){ .name = currentToken.lexeme };
};

void parsers::server::assertIsNotKeyword(const GQLToken token) {
    if (isKeyword(token.lexeme))
        throw ParserError::identifierIsKeyword(token);
};

const bool parsers::server::isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input"
            || lexeme == "extend");
};

