#include "./parser.hpp"

#include <format>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "../shared/ast.hpp"
#include "../shared/parser_error.hpp"
#include "./ast.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace gql::lexer;
namespace gql::parsers::file::server {
std::vector<ast::ASTNode> Parser::parse() {
    std::vector<ast::ASTNode> nodes;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        const auto &node = parseASTNode();
        nodes.emplace_back(node);
    };
    return nodes;
};

ast::ASTNode Parser::parseASTNode() {
    if (currentToken.lexeme == "scalar") {
        return parseScalarTypeDefinitionNode();
    } else if (currentToken.lexeme == "union") {
        return parseUnionTypeDefinitionNode();
    } else if (currentToken.lexeme == "enum") {
        return parseEnumTypeDefinitionNode();
    } else if (currentToken.lexeme == "interface") {
        return parseInterfaceTypeDefinitionNode();
    } else if (currentToken.lexeme == "type") {
        return parseObjectTypeDefinitionNode();
    } else if (currentToken.lexeme == "directive") {
        return parseDirectiveNode();
    } else if (currentToken.lexeme == "input") {
        return parseInputObjectDefinitionNode();
    } else if (currentToken.lexeme == "extend") {
        return parseExtendTypeNode();
    };
    throw shared::ParserError(
        currentToken,
        std::format("Unknown identifier: {}", currentToken.lexeme), source);
};

ast::ScalarDefinitionNode Parser::parseScalarTypeDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    return { .location = { .startToken = startToken,
                           .endToken = nameNode.location.endToken,
                           .source = source },
             .name = nameNode };
};

ast::UnionDefinitionNode Parser::parseUnionTypeDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::EQUAL);
    std::vector<shared::ast::NameNode> values = { parseNameNode() };
    while (consumeIfIsAhead(SimpleTokenType::VSLASH)) {
        values.emplace_back(parseNameNode());
    };
    return { .location = { .startToken = startToken,
                           .endToken = values.back().location.endToken,
                           .source = source },
             .name = nameNode,
             .values = values };
};

ast::EnumDefinitionNode Parser::parseEnumTypeDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<ast::EnumValueDefinitionNode> values;
    while (isAhead(ComplexTokenType::IDENTIFIER)) {
        values.emplace_back(parseEnumValueDefinitionNode());
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
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    return { .location = { .startToken = startToken,
                           .endToken = nameNode.location.endToken,
                           .source = source },
             .value = nameNode };
};

ast::InterfaceDefinitionNode Parser::parseInterfaceTypeDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<ast::FieldDefinitionNode> fields;
    while (isAhead(ComplexTokenType::IDENTIFIER)) {
        fields.emplace_back(parseFieldDefinitionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .name = nameNode,
             .fields = fields };
};

ast::InputObjectDefinitionNode Parser::parseInputObjectDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    consume(SimpleTokenType::LEFT_BRACE);
    std::vector<shared::ast::InputFieldDefinitionNode> fields;
    while (isAhead(ComplexTokenType::IDENTIFIER)) {
        fields.emplace_back(parseInputFieldDefinitionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .name = nameNode,
             .fields = fields };
};

std::vector<shared::ast::NameNode> Parser::parseImplementsClause() {
    std::vector<shared::ast::NameNode> interfaces;
    if (consumeIdentifierByLexemeIfIsAhead("implements")) {
        interfaces.emplace_back(parseNameNode());
        while (consumeIfIsAhead(SimpleTokenType::COMMA)) {
            interfaces.emplace_back(parseNameNode());
        };
    };
    return interfaces;
};

ast::ObjectDefinitionNode Parser::parseObjectTypeDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    const auto &interfaces = parseImplementsClause();
    std::vector<ast::FieldDefinitionNode> fields;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_BRACE)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            fields.emplace_back(parseFieldDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_BRACE);
    };
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .name = nameNode,
             .interfaces = interfaces,
             .fields = fields };
};

ast::ExtendTypeNode Parser::parseExtendTypeNode() {
    const auto startToken = currentToken;
    consumeIdentifierByLexeme("type");
    const auto &typeNode = parseObjectTypeDefinitionNode();
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .typeNode = typeNode };
};

ast::FieldDefinitionNode Parser::parseFieldDefinitionNode() {
    const auto startToken = currentToken;
    const auto &nameNode = parseNameNode();
    const auto &arguments = parseInputFieldDefinitionNodes();
    consume(SimpleTokenType::COLON);
    const auto &typeNode = parseTypeNode();
    std::optional<shared::ast::LiteralNode> defaultValue;
    if (consumeIfIsAhead(SimpleTokenType::EQUAL)) {
        defaultValue = parseLiteralNode();
    };
    std::vector<shared::ast::DirectiveInvocationNode> directives;
    while (consumeIfIsAhead(SimpleTokenType::AT_SIGN)) {
        directives.push_back(parseDirectiveInvocationNode());
    };
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .name = nameNode,
             .type = typeNode,
             .arguments = arguments,
             .directives = directives
        };
};

ast::DirectiveLocation Parser::parseDirectiveLocation() {
    consumeIdentifier();
    const auto &value = ast::stringToDirectiveLocation(currentToken.lexeme);
    if (!value.has_value()) {
        throw shared::ParserError(currentToken, "Unknown directive location",
                                  source);
    };
    return value.value();
};
};  // namespace gql::parsers::file::server
