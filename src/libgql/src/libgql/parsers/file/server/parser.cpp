#include "./parser.hpp"

#include <format>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "../shared/ast.hpp"
#include "../shared/parser_error.hpp"
#include "./ast.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"
#include "libgql/parsers/file/base/parser.hpp"
#include "utils.hpp"

using namespace parsers::file;
using namespace parsers::file::server;
using namespace lexer;

ast::FileNodes Parser::parse() {
    std::vector<ast::TypeDefinitionNode> definitions;
    std::vector<ast::ExtendTypeNode> extensions;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        const auto &node = parseASTNode();
        std::visit<void>(
            overloaded{
                [&definitions](const ast::TypeDefinitionNode &tNode) -> void {
                    definitions.emplace_back(tNode);
                },
                [&extensions](const ast::ExtendTypeNode &eNode) -> void {
                    extensions.emplace_back(eNode);
                } },
            node);
    };
    return {
        .source = source,
        .definitions = definitions,
        .extensions = extensions,
    };
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
    } else if (currentToken.lexeme == "input") {
        const auto &interfaceNode = parseInterfaceTypeDefinitionNode();
        return (
            ast::InputObjectDefinitionNode){ .location = interfaceNode.location,
                                             .name = interfaceNode.name,
                                             .fields = interfaceNode.fields };
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
    // clang-format off
    USE_BRACE_CONTEXT(
        std::vector<ast::EnumValueDefinitionNode> values;
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            values.emplace_back(parseEnumValueDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
    );
    // clang-format on
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
    // clang-format off
    USE_BRACE_CONTEXT(
        std::vector<ast::FieldDefinitionNode> fields;
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            fields.emplace_back(parseFieldDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
    );
    // clang-format on
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
    // clang-format off
    USE_BRACE_CONTEXT(
        std::vector<ast::FieldDefinitionNode> fields;
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            fields.emplace_back(parseFieldDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
    )
    // clang-format on
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
    const auto &arguments = parseArguments();
    consume(SimpleTokenType::COLON);
    const auto &typeNode = parseTypeNode();
    std::optional<shared::ast::LiteralNode> defaultValue;
    if (consumeIfIsAhead(SimpleTokenType::EQUAL)) {
        defaultValue = parseLiteralNode();
    };
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .name = nameNode,
             .type = typeNode,
             .arguments = arguments };
};

std::vector<shared::ast::InputValueDefinitionNode> Parser::parseArguments() {
    std::vector<shared::ast::InputValueDefinitionNode> arguments;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            arguments.emplace_back(parseInputValueDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    return arguments;
};
