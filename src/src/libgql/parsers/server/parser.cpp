#include "./parser.hpp"

#include <map>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"

using namespace parsers::server;

Parser::Parser(std::vector<GQLToken> tokens) noexcept
    : tokens{ tokens }, currentToken{ tokens[0] } {};

const ast::ASTProgram Parser::getAstTree() {
    std::vector<ast::ASTNode> nodes = {};
    while (index + 1 < tokens.size()) {
        nodes.push_back(parseNode());
    };
    return { .nodes = nodes };
};

const ast::ASTNode Parser::parseNode() {
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

const ast::ASTNode Parser::parseComplexToken() {
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
        return (ast::ASTExtendNode){ .type = typeSpec };
    } else if (currentToken.lexeme == "enum") {
        return parseEnumNode();
    } else if (currentToken.lexeme == "union") {
        return parseUnionNode();
    };
    throw ParserError::unexpectedIdentifier(currentToken);
};

const ast::ASTEnumNode Parser::parseEnumNode() {
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

const ast::ASTUnionNode Parser::parseUnionNode() {
    consumeIdentifier();
    const std::string name = currentToken.lexeme;
    consume(SimpleTokenType::EQUAL);
    consumeIdentifier();
    std::vector<ast::ASTGQLReferenceType> items;
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

const ast::ASTTypeDefinition Parser::parseTypeNode(bool isInput) {
    consumeIdentifier();
    const std::string name = currentToken.lexeme;
    consume(SimpleTokenType::LEFT_BRACE);
    std::map<std::string, ast::ASTTypeSpec> definitions;
    while (lookahead().type != (GQLTokenType)SimpleTokenType::RIGHT_BRACE) {
        const std::string key = parseIdentifier();
        definitions[key] = parseTypeSpecNode();
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .name = name, .fields = definitions, .isInput = isInput };
};

const std::string Parser::parseIdentifier() {
    consumeIdentifier();
    return currentToken.lexeme;
};

const ast::ASTTypeSpec Parser::parseTypeSpecNode() {
    const auto &token = lookahead();
    if (token.type == (GQLTokenType)SimpleTokenType::LEFT_PAREN) {
        return parseCallableTypeSpecNode();
    } else {
        return parseLiteralTypeSpecNode();
    };
};

const ast::ASTLiteralTypeSpec Parser::parseLiteralTypeSpecNode() {
    consume(SimpleTokenType::COLON);
    const auto &token = lookahead();
    if (token.type == (GQLTokenType)SimpleTokenType::LEFT_BRACKET) {
        return parseArrayTypeSpecNode();
    } else {
        return parseTrivialTypeSpecNode();
    };
};

const ast::ASTCallableTypeSpec Parser::parseCallableTypeSpecNode() {
    std::map<std::string, ast::ASTLiteralTypeSpec> arguments;
    consume(SimpleTokenType::LEFT_PAREN);
    while (lookahead().type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
        consumeIdentifier();
        assertIsNotKeyword(currentToken);
        std::string name = currentToken.lexeme;
        const auto &type = parseLiteralTypeSpecNode();
        arguments[name] = type;
        if (lookahead().type == (GQLTokenType)SimpleTokenType::COMMA) {
            consume(SimpleTokenType::COMMA);
        };
    };
    consume(SimpleTokenType::RIGHT_PAREN);
    const auto &returnType = parseLiteralTypeSpecNode();
    return { .returnType = returnType, .arguments = arguments };
};

const ast::ASTArrayTypeSpec Parser::parseArrayTypeSpecNode() {
    consume(SimpleTokenType::LEFT_BRACKET);
    const auto &trivialType = parseTrivialTypeSpecNode();
    consume(SimpleTokenType::RIGHT_BRACKET);
    bool nullable = true;
    if (lookahead().type == (GQLTokenType)SimpleTokenType::BANG) {
        consume(SimpleTokenType::BANG);
        nullable = false;
    };
    std::optional<ast::ASTArrayLiteral> defaultValue;
    if (std::holds_alternative<ast::ASTGQLSimpleType>(trivialType.type)) {
        defaultValue = maybeParseArrayLiteralNode(
            std::get<ast::ASTGQLSimpleType>(trivialType.type));
    };
    return { .type = trivialType,
             .nullable = nullable,
             .defaultValue = defaultValue };
};
const ast::ASTTrivialTypeSpec Parser::parseTrivialTypeSpecNode() {
    const ast::ASTGQLType type = parseGQLType();
    bool nullable = true;
    if (lookahead().type == (GQLTokenType)SimpleTokenType::BANG) {
        consume(SimpleTokenType::BANG);
        nullable = false;
    };
    std::optional<ast::ASTLiteral> defaultValue;
    if (std::holds_alternative<ast::ASTGQLSimpleType>(type)) {
        defaultValue
            = maybeParseLiteralNode(std::get<ast::ASTGQLSimpleType>(type));
    };
    return { .type = type, .nullable = nullable, .defaultValue = defaultValue };
};

const ast::ASTGQLType Parser::parseGQLType() {
    consumeIdentifier();
    if (currentToken.lexeme == "Int")
        return ast::ASTGQLSimpleType::INT;
    else if (currentToken.lexeme == "Float")
        return ast::ASTGQLSimpleType::FLOAT;
    else if (currentToken.lexeme == "String")
        return ast::ASTGQLSimpleType::STRING;
    else if (currentToken.lexeme == "Boolean")
        return ast::ASTGQLSimpleType::BOOLEAN;
    return (ast::ASTGQLReferenceType){ .name = currentToken.lexeme };
};

const std::optional<ast::ASTLiteral> Parser::maybeParseLiteralNode(
    ast::ASTGQLSimpleType t) {
    if (lookahead().type == (GQLTokenType)SimpleTokenType::EQUAL) {
        consume(SimpleTokenType::EQUAL);
        return parseLiteralNode(t);
    };
    return std::nullopt;
};

const std::optional<ast::ASTArrayLiteral> Parser::maybeParseArrayLiteralNode(
    ast::ASTGQLSimpleType t) {
    if (lookahead().type == (GQLTokenType)SimpleTokenType::EQUAL) {
        consume(SimpleTokenType::EQUAL);
        return parseArrayLiteralNode(t);
    };
    return std::nullopt;
};

const ast::ASTLiteral Parser::parseLiteralNode(ast::ASTGQLSimpleType t) {
    switch (t) {
        case ast::ASTGQLSimpleType::INT: {
            consume(ComplexTokenType::NUMBER);
            return (ast::ASTIntLiteral){ .value
                                         = std::stoi(currentToken.lexeme) };
        };
        case ast::ASTGQLSimpleType::FLOAT: {
            consume(ComplexTokenType::NUMBER);
            return (ast::ASTFloatLiteral){ .value
                                           = std::stof(currentToken.lexeme) };
        };
        case ast::ASTGQLSimpleType::BOOLEAN: {
            consume(ComplexTokenType::BOOLEAN);
            return (ast::ASTBooleanLiteral){ .value
                                             = currentToken.lexeme == "true" };
        };
        case ast::ASTGQLSimpleType::STRING: {
            consume(ComplexTokenType::STRING);
            return (ast::ASTStringLiteral){ .value = currentToken.lexeme };
        };
        case ast::ASTGQLSimpleType::ID: {
            throw ParserError(currentToken,
                              "ID default values are not supported");
        };
    };
};

void Parser::maybeConsumeComma() {
    if (lookahead().type == (GQLTokenType)SimpleTokenType::COMMA) {
        consume(SimpleTokenType::COMMA);
    };
};
const ast::ASTArrayLiteral Parser::parseArrayLiteralNode(
    ast::ASTGQLSimpleType t) {
    consume(SimpleTokenType::LEFT_BRACKET);
    switch (t) {
        case ast::ASTGQLSimpleType::INT: {
            ast::ASTIntArrayLiteral items;
            while (lookahead().type == (GQLTokenType)ComplexTokenType::NUMBER) {
                items.push_back(
                    std::get<ast::ASTIntLiteral>(parseLiteralNode(t)));
                maybeConsumeComma();
            };
            return items;
        };
        case ast::ASTGQLSimpleType::FLOAT: {
            ast::ASTFloatArrayLiteral items;
            while (lookahead().type == (GQLTokenType)ComplexTokenType::NUMBER) {
                items.push_back(
                    std::get<ast::ASTFloatLiteral>(parseLiteralNode(t)));
                maybeConsumeComma();
            };
            return items;
        };
        case ast::ASTGQLSimpleType::BOOLEAN: {
            ast::ASTBooleanArrayLiteral items;
            while (lookahead().type == (GQLTokenType)ComplexTokenType::BOOLEAN) {
                items.push_back(
                    std::get<ast::ASTBooleanLiteral>(parseLiteralNode(t)));
                maybeConsumeComma();
            };
            return items;
        };
        case ast::ASTGQLSimpleType::STRING: {
            ast::ASTStringArrayLiteral items;
            while (lookahead().type == (GQLTokenType)ComplexTokenType::STRING) {
                items.push_back(
                    std::get<ast::ASTStringLiteral>(parseLiteralNode(t)));
                maybeConsumeComma();
            };
            return items;
        };
        case ast::ASTGQLSimpleType::ID: {
            throw ParserError(currentToken,
                              "ID default values are not supported");
        };
    };
    consume(SimpleTokenType::RIGHT_BRACKET);
};
void parsers::server::assertIsNotKeyword(const GQLToken token) {
    if (isKeyword(token.lexeme)) throw ParserError::identifierIsKeyword(token);
};

const bool parsers::server::isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input"
            || lexeme == "extend");
};
