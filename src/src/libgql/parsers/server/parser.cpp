#include "./parser.hpp"

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/shared/shared.hpp"

using namespace parsers;
using namespace parsers::server;

Parser::Parser(std::vector<GQLToken> tokens,
               std::shared_ptr<shared::ast::SourceFile> source) noexcept
    : tokens{ tokens },
      source{ std::move(source) },
      currentToken{ tokens[0] } {};

ast::FileNodes Parser::parse() {
    std::vector<ast::TypeDefinitionNode> definitions;
    std::vector<ast::ExtendTypeNode> extensions;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        const auto &[name, node] = parseASTNode();
        if (std::holds_alternative<ast::TypeDefinitionNode>(node)) {
            definitions.push_back(std::get<ast::TypeDefinitionNode>(node));
        } else if (std::holds_alternative<ast::ExtendTypeNode>(node)) {
            extensions.push_back(std::get<ast::ExtendTypeNode>(node));
        } else {
            throw shared::ParserError(currentToken, "Unexpected node type");
        };
    };
    return {
        .source = source,
        .definitions = definitions,
        .extensions = extensions,
    };
};

std::pair<std::string, ast::ASTNode> Parser::parseASTNode() {
    if (currentToken.type != (GQLTokenType)ComplexTokenType::IDENTIFIER) {
        throw shared::ParserError::wrongType(currentToken,
                                     ComplexTokenType::IDENTIFIER);
    };
    if (currentToken.lexeme == "scalar") {
        const auto &node = parseScalarTypeDefinitionNode();
        return { node.name.name, node };
    } else if (currentToken.lexeme == "union") {
        const auto &node = parseUnionTypeDefinitionNode();
        return { node.name.name, node };
    } else if (currentToken.lexeme == "enum") {
        const auto &node = parseEnumTypeDefinitionNode();
        return { node.name.name, node };
    } else if (currentToken.lexeme == "interface") {
        const auto &node = parseInterfaceTypeDefinitionNode();
        return { node.name.name, node };
    } else if (currentToken.lexeme == "type") {
        const auto &node = parseObjectTypeDefinitionNode();
        return { node.name.name, node };
    } else if (currentToken.lexeme == "input") {
        const auto &interfaceNode = parseInterfaceTypeDefinitionNode();
        return { interfaceNode.name.name,
                 (ast::InputObjectDefinitionNode){
                     .location = interfaceNode.location,
                     .name = interfaceNode.name,
                     .fields = interfaceNode.fields } };
    } else if (currentToken.lexeme == "extend") {
        const auto &node = parseExtendTypeNode();
        return { node.typeNode.name.name, node };
    };
    throw shared::ParserError(currentToken, "Unknown identifier");
};

ast::ExtendTypeNode Parser::parseExtendTypeNode() {
    const GQLToken startToken = currentToken;
    consume(ComplexTokenType::IDENTIFIER);
    if (currentToken.lexeme != "type") {
        throw shared::ParserError(currentToken, "Expected \"type\" identifier");
    };
    const auto &typeNode = parseObjectTypeDefinitionNode();
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .typeNode = typeNode };
};

shared::ast::NameNode Parser::parseNameNode(bool raiseOnKeyword) {
    consume(ComplexTokenType::IDENTIFIER);
    if (raiseOnKeyword) shared::assertIsNotKeyword(currentToken);
    return { .location = { .startToken = currentToken,
                           .endToken = currentToken,
                           .source = source },
             .name = currentToken.lexeme };
};

ast::ObjectDefinitionNode Parser::parseObjectTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    std::vector<shared::ast::NameNode> interfaces;
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
    std::vector<shared::ast::NameNode> values;
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
    std::vector<shared::ast::InputValueDefinitionNode> arguments;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            arguments.push_back(parseInputValueDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    consume(SimpleTokenType::COLON);
    const auto &typeNode = parseTypeNode();
    std::optional<shared::ast::LiteralNode> defaultValue;
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

shared::ast::InputValueDefinitionNode Parser::parseInputValueDefinitionNode() {
    const auto &nameNode = parseNameNode();
    const GQLToken startToken = currentToken;
    consume(SimpleTokenType::COLON);
    const auto &typeNode = parseTypeNode();
    std::optional<shared::ast::LiteralNode> defaultValue;
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

shared::ast::LiteralNode Parser::parseLiteralNode() {
    advance();
    if (!std::holds_alternative<ComplexTokenType>(currentToken.type)) {
        throw shared::ParserError(currentToken, "Expected literal node");
    };
    switch (std::get<ComplexTokenType>(currentToken.type)) {
        case ComplexTokenType::NUMBER: {
            shared::ast::NodeLocation location = { .startToken = currentToken,
                                                   .endToken = currentToken,
                                                   .source = source };
            try {
                return (shared::ast::LiteralIntNode){ .location = location,
                                              .value = std::stoi(
                                                  currentToken.lexeme) };
            } catch (...) {
            };
            return (shared::ast::LiteralFloatNode){
                .location = location, .value = std::stof(currentToken.lexeme)
            };
        };
        case ComplexTokenType::BOOLEAN: {
            return (shared::ast::LiteralBooleanNode){
                .location = { .startToken = currentToken,
                              .endToken = currentToken,
                              .source = source },
                .value = currentToken.lexeme == "true"
            };
        };
        case ComplexTokenType::STRING: {
            return (shared::ast::LiteralStringNode){
                .location = { .startToken = currentToken,
                              .endToken = currentToken,
                              .source = source },
                .value = currentToken.lexeme
            };
        };
        case ComplexTokenType::IDENTIFIER: {
            return (shared::ast::LiteralEnumValueNode){
                .location = { .startToken = currentToken,
                              .endToken = currentToken,
                              .source = source },
                .value = currentToken.lexeme
            };
        };
    };
};

shared::ast::TypeNode Parser::parseTypeNode() {
    if (isAhead(SimpleTokenType::LEFT_BRACKET)) {
        return parseListTypeNode();
    } else {
        return parseNamedTypeNode();
    };
};

shared::ast::NamedTypeNode Parser::parseNamedTypeNode() {
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

shared::ast::ListTypeNode Parser::parseListTypeNode() {
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
        throw shared::ParserError::wrongType(currentToken, type);
    };
};

void Parser::consumeIdentifier() {
    consume(ComplexTokenType::IDENTIFIER);
    shared::assertIsNotKeyword(currentToken);
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
