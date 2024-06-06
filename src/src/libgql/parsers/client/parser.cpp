#include "./parser.hpp"

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/client/ast.hpp"
#include "libgql/parsers/shared/shared.hpp"

using namespace parsers;
using namespace parsers::client;

parsers::client::Parser::Parser(
    std::vector<GQLToken> tokens,
    std::shared_ptr<shared::ast::SourceFile> source) noexcept
    : tokens{ tokens },
      source{ std::move(source) },
      currentToken{ tokens[0] } {};

std::vector<ast::ClientDefinition> Parser::parse() {
    std::vector<ast::ClientDefinition> operations;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        operations.emplace_back(parseClientDefinition());
    };
    return operations;
};

ast::ClientDefinition Parser::parseClientDefinition() {
    if (currentToken.lexeme == "fragment") {
        return parseFragmentDefinition();
    };
    return parseOperationDefinition();
};

ast::FragmentDefinition Parser::parseFragmentDefinition() {
    shared::ast::NodeLocation location = {
        .startToken = currentToken,
        .source = source,
    };
    const auto &name = parseNameNode();
    consumeIdentifierByLexeme("on");
    const auto &typeName = parseNameNode();
    const auto &spec = parseFragmentSpec();
    location.endToken = spec.location.endToken;
    return {
        .location = location, .name = name, .typeName = typeName, .spec = spec
    };
};

ast::OperationDefinition Parser::parseOperationDefinition() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &type = client::ast::opTypeFromClientOp(currentToken.lexeme);
    if (!type.has_value()) {
        throw shared::ParserError(currentToken,
                                  "Unexpected client operation type", source);
    };
    const auto &name = parseNameNode();
    std::map<std::string, shared::ast::InputValueDefinitionNode> parameters;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            const auto &node = parseInputValueDefinitionNode();
            if (parameters.contains(node.name.name)) {
                throw shared::ParserError(
                    node.name.location.startToken,
                    "Parameter with this name already exists",
                    node.name.location.source);
            };
            parameters[node.name.name] = node;
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    ast::FragmentSpec fragment = parseFragmentSpec();
    location.endToken = fragment.location.endToken;
    return { .location = location,
             .type = type.value(),
             .name = name,
             .parameters = parameters,
             .fragment = fragment };
};

ast::Argument Parser::parseArgument() {
    const auto &name = parseNameNode();
    consume(SimpleTokenType::COLON);
    const auto &paramName = parseNameNode();
    return { .name = name, .paramName = paramName };
};

ast::FragmentSpec Parser::parseFragmentSpec() {
    consume(SimpleTokenType::LEFT_BRACE);
    shared::ast::NodeLocation location = {
        .startToken = currentToken,
        .source = source,
    };
    ast::FragmentSpec spec;
    while (isAhead(ComplexTokenType::IDENTIFIER) ||
           isAhead(ComplexTokenType::SPREAD)) {
        spec.selections.emplace_back(parseSelectionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    location.endToken = currentToken;
    spec.location = location;
    return spec;
};

ast::ConditionalSpreadSelectionNode
Parser::parseConditionalSpreadSelectionNode() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    consumeIdentifierByLexeme("on");
    const auto &typeName = parseNameNode();
    const auto &fragmentSpec = parseFragmentSpec();
    location.endToken = fragmentSpec.location.endToken;
    return { .location = location,
             .typeName = typeName,
             .fragment = std::make_shared<ast::FragmentSpec>(fragmentSpec) };
}

ast::SelectionNode Parser::parseSelectionNode() {
    if (!consumeIfIsAhead(ComplexTokenType::SPREAD)) {
        return parseFieldSelectionNode();
    };
    if (isAheadByLexeme("on")) {
        return parseConditionalSpreadSelectionNode();
    }
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &fragmentName = parseNameNode();
    location.endToken = fragmentName.location.endToken;
    return (ast::SpreadSelectionNode){ .fragmentName = fragmentName };
};

bool Parser::isAheadByLexeme(const std::string &lexeme) {
    return lookahead().lexeme == lexeme;
};

std::pair<shared::ast::NameNode, shared::ast::NameNode>
Parser::parseNameAndSelectionName() {
    shared::ast::NameNode selectionName = parseNameNode();
    shared::ast::NameNode fieldName = selectionName;
    std::optional<std::vector<shared::ast::InputValueDefinitionNode>> arguments;
    if (consumeIfIsAhead(SimpleTokenType::COLON)) {
        fieldName = parseNameNode();
    };
    return { fieldName, selectionName };
};

ast::ObjectFieldSpec Parser::parseObjectFieldSpec() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };

    const auto &[fieldName, selectionName] = parseNameAndSelectionName();
    location.endToken = fieldName.location.endToken;
    if (!consumeIfIsAhead(SimpleTokenType::LEFT_PAREN))
        return (ast::ObjectLiteralFieldSpec){ .location = location,
                                              .selectionName = selectionName,
                                              .name = fieldName };
    std::vector<ast::Argument> args;
    while (isAhead(ComplexTokenType::IDENTIFIER)) {
        args.push_back(parseArgument());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_PAREN);
    location.endToken = currentToken;
    return (ast::ObjectCallableFieldSpec){ .selectionName = selectionName,
                                           .name = fieldName,
                                           .arguments = args };
};

ast::FieldSelectionNode Parser::parseFieldSelectionNode() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &fieldSpec = parseObjectFieldSpec();
    std::optional<std::shared_ptr<ast::FragmentSpec>> spec;
    if (isAhead(SimpleTokenType::LEFT_BRACE)) {
        spec = std::make_shared<ast::FragmentSpec>(parseFragmentSpec());
    };
    location.endToken = currentToken;
    return { .location = location, .field = fieldSpec, .spec = spec };
};

bool Parser::consumeIdentifierByLexemeIfIsAhead(const std::string &lexeme) {
    bool isAhead = lookahead().lexeme == lexeme;
    if (isAhead) {
        consumeIdentifierByLexeme(lexeme);
    };
    return isAhead;
};

void Parser::consumeIdentifierByLexeme(const std::string &lexeme) {
    consumeIdentifier();
    if (currentToken.lexeme != lexeme) {
        throw shared::ParserError::wrongLexeme(currentToken, lexeme, source);
    };
};

const GQLToken Parser::lookahead() { return tokens[index + 1]; };

void Parser::consume(const GQLTokenType type) {
    index += 1;
    currentToken = tokens[index];
    if (currentToken.type != type) {
        throw shared::ParserError::wrongType(currentToken, type, source);
    };
};

void Parser::consumeIdentifier() {
    consume(ComplexTokenType::IDENTIFIER);
    shared::assertIsNotKeyword(currentToken, source);
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

shared::ast::NameNode Parser::parseNameNode(bool raiseOnKeyword) {
    consume(ComplexTokenType::IDENTIFIER);
    if (raiseOnKeyword) shared::assertIsNotKeyword(currentToken, source);
    return { .location = { .startToken = currentToken,
                           .endToken = currentToken,
                           .source = source },
             .name = currentToken.lexeme };
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

shared::ast::LiteralNode Parser::parseLiteralNode() {
    advance();
    if (!std::holds_alternative<ComplexTokenType>(currentToken.type)) {
        throw shared::ParserError(currentToken, "Expected literal node",
                                  source);
    };
    switch (std::get<ComplexTokenType>(currentToken.type)) {
        case ComplexTokenType::NUMBER: {
            shared::ast::NodeLocation location = { .startToken = currentToken,
                                                   .endToken = currentToken,
                                                   .source = source };
            try {
                return (shared::ast::LiteralIntNode){
                    .location = location,
                    .value = std::stoi(currentToken.lexeme)
                };
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
        case ComplexTokenType::SPREAD: {
            throw shared::ParserError(currentToken, "Unexpected spread usage",
                                      source);
        };
    };
};

void Parser::advance() {
    index += 1;
    currentToken = tokens[index];
};
