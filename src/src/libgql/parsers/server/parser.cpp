#include "./parser.hpp"

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"

using namespace parsers::server;

Parser::Parser(std::vector<GQLToken> tokens,
               std::shared_ptr<ast::SourceFile> source) noexcept
    : tokens{ tokens },
      source{ std::move(source) },
      currentToken{ tokens[0] } {};

ast::FileNodes Parser::parse() {
    std::vector<ast::TypeDefinitionNode> definitions;
    std::vector<ast::ExtendTypeNode> extensions;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        const auto &node = parseASTNode();
        if (std::holds_alternative<ast::TypeDefinitionNode>(node)) {
            definitions.push_back(std::get<ast::TypeDefinitionNode>(node));
        } else if (std::holds_alternative<ast::ExtendTypeNode>(node)) {
            extensions.push_back(std::get<ast::ExtendTypeNode>(node));
        } else {
            throw ParserError(currentToken, "Unexpected node type");
        };
    };
    return { .source = source,
             .definitions = definitions,
             .extensions = extensions };
};

ast::ASTNode Parser::parseASTNode() {
    if (currentToken.type != (GQLTokenType)ComplexTokenType::IDENTIFIER) {
        throw ParserError::wrongType(currentToken,
                                     ComplexTokenType::IDENTIFIER);
    };
    if (currentToken.lexeme == "scalar")
        return parseScalarTypeDefinitionNode();
    else if (currentToken.lexeme == "union")
        return parseUnionTypeDefinitionNode();
    else if (currentToken.lexeme == "enum")
        return parseEnumTypeDefinitionNode();
    else if (currentToken.lexeme == "interface")
        return parseInterfaceTypeDefinitionNode();
    else if (currentToken.lexeme == "type")
        return parseObjectTypeDefinitionNode();
    else if (currentToken.lexeme == "input") {
        const auto &interfaceNode = parseInterfaceTypeDefinitionNode();
        return (
            ast::InputObjectDefinitionNode){ .location = interfaceNode.location,
                                             .name = interfaceNode.name,
                                             .fields = interfaceNode.fields };
    } else if (currentToken.lexeme == "extend") {
        return parseExtendTypeNode();
    };
    throw ParserError(currentToken, "Unknown identifier");
};

ast::ExtendTypeNode Parser::parseExtendTypeNode() {
    const GQLToken startToken = currentToken;
    consume(ComplexTokenType::IDENTIFIER);
    if (currentToken.lexeme != "type") {
        throw ParserError(currentToken, "Expected \"type\" identifier");
    };
    const auto &typeNode = parseObjectTypeDefinitionNode();
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .typeNode = typeNode };
};

ast::NameNode Parser::parseNameNode(bool raiseOnKeyword) {
    consume(ComplexTokenType::IDENTIFIER);
    if (raiseOnKeyword) assertIsNotKeyword(currentToken);
    return { .location = { .startToken = currentToken,
                           .endToken = currentToken,
                           .source = source },
             .name = currentToken.lexeme };
};

ast::ObjectDefinitionNode Parser::parseObjectTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    std::vector<ast::NameNode> interfaces;
    if (lookahead().lexeme == "implements") {
        consume(ComplexTokenType::IDENTIFIER);
        interfaces.push_back(parseNameNode());
        while (consumeIfIsAhead(SimpleTokenType::COMMA)) {
            interfaces.push_back(parseNameNode());
        };
    };
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<ast::FieldDefinitionNode> fields;
    while (lookahead().type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
        fields.push_back(parseFieldDefinitionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode,
             .interfaces = interfaces,
             .fields = fields };
};

ast::ScalarDefinitionNode Parser::parseScalarTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    const GQLToken endToken = nameNode.location.endToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode };
};

ast::EnumDefinitionNode Parser::parseEnumTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<ast::EnumValueDefinitionNode> values;
    while (lookahead().type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
        values.push_back(parseEnumValueDefinitionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .name = nameNode,
             .values = values };
};

ast::EnumValueDefinitionNode Parser::parseEnumValueDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    const GQLToken endToken = nameNode.location.endToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .value = nameNode };
};

ast::UnionDefinitionNode Parser::parseUnionTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::EQUAL);
    std::vector<ast::NameNode> values;
    values.push_back(parseNameNode());
    while (lookahead().type == (GQLTokenType)SimpleTokenType::VSLASH) {
        consume(SimpleTokenType::VSLASH);
        values.push_back(parseNameNode());
    };
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode,
             .values = values };
};

ast::InterfaceDefinitionNode Parser::parseInterfaceTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<ast::FieldDefinitionNode> fields;
    while (lookahead().type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
        fields.push_back(parseFieldDefinitionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode,
             .fields = fields };
};

ast::FieldDefinitionNode Parser::parseFieldDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode(false);
    std::vector<ast::InputValueDefinitionNode> arguments;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            arguments.push_back(parseInputValueDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    consume(SimpleTokenType::COLON);
    const auto &typeNode = parseTypeNode();
    std::optional<ast::LiteralNode> defaultValue;
    if (consumeIfIsAhead(SimpleTokenType::EQUAL)) {
        defaultValue = parseLiteralNode();
    };
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode,
             .type = typeNode,
             .arguments = arguments };
};

ast::InputValueDefinitionNode Parser::parseInputValueDefinitionNode() {
    const auto &nameNode = parseNameNode();
    const GQLToken startToken = currentToken;
    consume(SimpleTokenType::COLON);
    const auto &typeNode = parseTypeNode();
    std::optional<ast::LiteralNode> defaultValue;
    if (consumeIfIsAhead(SimpleTokenType::EQUAL)) {
        defaultValue = parseLiteralNode();
    };
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode,
             .type = typeNode,
             .defaultValue = defaultValue };
};

void Parser::advance() {
    index += 1;
    currentToken = tokens[index];
};

ast::LiteralNode Parser::parseLiteralNode() {
    advance();
    if (!std::holds_alternative<ComplexTokenType>(currentToken.type)) {
        throw ParserError(currentToken, "Expected literal node");
    };
    switch (std::get<ComplexTokenType>(currentToken.type)) {
        case ComplexTokenType::NUMBER: {
            ast::NodeLocation location = { .startToken = currentToken,
                                           .endToken = currentToken,
                                           .source = source };
            try {
                return (ast::LiteralIntNode){ .location = location,
                                              .value = std::stoi(
                                                  currentToken.lexeme) };
            } catch (...) {
            };
            return (ast::LiteralFloatNode){
                .location = location, .value = std::stof(currentToken.lexeme)
            };
        };
        case ComplexTokenType::BOOLEAN: {
            return (ast::LiteralBooleanNode){
                .location = { .startToken = currentToken,
                              .endToken = currentToken,
                              .source = source },
                .value = currentToken.lexeme == "true"
            };
        };
        case ComplexTokenType::STRING: {
            return (ast::LiteralStringNode){
                .location = { .startToken = currentToken,
                              .endToken = currentToken,
                              .source = source },
                .value = currentToken.lexeme
            };
        };
        case ComplexTokenType::IDENTIFIER: {
            return (ast::LiteralEnumValueNode){
                .location = { .startToken = currentToken,
                              .endToken = currentToken,
                              .source = source },
                .value = currentToken.lexeme
            };
        };
    };
};

ast::TypeNode Parser::parseTypeNode() {
    if (isAhead(SimpleTokenType::LEFT_BRACKET)) {
        return parseListTypeNode();
    } else {
        return parseNamedTypeNode();
    };
};

ast::NamedTypeNode Parser::parseNamedTypeNode() {
    const auto &nameNode = parseNameNode();
    const GQLToken startToken = currentToken;
    bool nullable = !consumeIfIsAhead(SimpleTokenType::BANG);
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .name = nameNode,
             .nullable = nullable };
};

ast::ListTypeNode Parser::parseListTypeNode() {
    consume(SimpleTokenType::LEFT_BRACKET);
    const GQLToken startToken = currentToken;
    const auto &typeNode = parseNamedTypeNode();
    consume(SimpleTokenType::RIGHT_BRACKET);
    bool nullable = !consumeIfIsAhead(SimpleTokenType::BANG);
    const GQLToken endToken = currentToken;
    return { .location = { .startToken = startToken,
                           .endToken = endToken,
                           .source = source },
             .type = typeNode,
             .nullable = nullable };
};

const GQLToken Parser::lookahead() { return tokens[index + 1]; };

void Parser::consume(const GQLTokenType type) {
    index += 1;
    currentToken = tokens[index];
    if (currentToken.type != type) {
        throw ParserError::wrongType(currentToken, type);
    };
};

void Parser::consumeIdentifier() {
    consume(ComplexTokenType::IDENTIFIER);
    assertIsNotKeyword(currentToken);
};

bool Parser::consumeIfIsAhead(GQLTokenType expectedType) {
    bool tokenIsAhead = isAhead(expectedType);
    if (tokenIsAhead) {
        consume(expectedType);
    };
    return tokenIsAhead;
};

bool Parser::isAhead(GQLTokenType expectedType) {
    return lookahead().type == expectedType;
};
void parsers::server::assertIsNotKeyword(const GQLToken token) {
    if (isKeyword(token.lexeme)) throw ParserError::identifierIsKeyword(token);
};

const bool parsers::server::isKeyword(const std::string lexeme) noexcept {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input" ||
            lexeme == "extend");
};
