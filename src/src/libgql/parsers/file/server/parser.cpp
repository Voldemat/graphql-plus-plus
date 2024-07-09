#include "./parser.hpp"

#include <format>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "../shared/ast.hpp"
#include "../shared/parser_error.hpp"
#include "./ast.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace parsers::file;
using namespace parsers::file::server;
using namespace lexer;

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
            throw shared::ParserError(currentToken, "Unexpected node type",
                                      source);
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
        throw shared::ParserError::wrongType(
            currentToken, ComplexTokenType::IDENTIFIER, source);
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
    throw shared::ParserError(
        currentToken,
        std::format("Unknown identifier: {}", currentToken.lexeme), source);
};

ast::ExtendTypeNode Parser::parseExtendTypeNode() {
    const GQLToken startToken = currentToken;
    consume(ComplexTokenType::IDENTIFIER);
    if (currentToken.lexeme != "type") {
        throw shared::ParserError(currentToken, "Expected \"type\" identifier",
                                  source);
    };
    const auto &typeNode = parseObjectTypeDefinitionNode();
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .typeNode = typeNode };
};

ast::ObjectDefinitionNode Parser::parseObjectTypeDefinitionNode() {
    const GQLToken startToken = currentToken;
    const auto &nameNode = parseNameNode();
    std::vector<shared::ast::NameNode> interfaces;
    if (lookahead()->lexeme == "implements") {
        consume(ComplexTokenType::IDENTIFIER);
        interfaces.push_back(parseNameNode());
        while (consumeIfIsAhead(SimpleTokenType::COMMA)) {
            interfaces.push_back(parseNameNode());
        };
    };
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<ast::FieldDefinitionNode> fields;
    while (lookahead()->type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
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
    while (lookahead()->type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
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
    std::vector<shared::ast::NameNode> values = { parseNameNode() };
    while (consumeIfIsAhead(SimpleTokenType::VSLASH)) {
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
    while (lookahead()->type == (GQLTokenType)ComplexTokenType::IDENTIFIER) {
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
