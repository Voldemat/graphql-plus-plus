#include "./parser.hpp"

#include <memory>
#include <optional>
#include <stdexcept>
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

ast::OperationArg Parser::parseOperationArg() {
    const auto &name = parseNameNode();
    consume(SimpleTokenType::COLON);
    const auto &paramName = parseNameNode();
    shared::ast::NodeLocation location = { .startToken =
                                               name.location.startToken,
                                           .endToken =
                                               paramName.location.endToken,
                                           .source = source };
    return { .location = location, .name = name, .paramName = paramName };
};

ast::OperationSpec Parser::parseOperationSpec() {
    const auto &[name, selectionName] = parseNameAndSelectionName();
    shared::ast::NodeLocation location = {
        .startToken = selectionName.location.startToken, .source = source
    };
    std::vector<ast::OperationArg> args;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            args.push_back(parseOperationArg());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    return { .location = location,
             .selectionName = selectionName,
             .name = name,
             .args = args };
};

ast::OperationDefinition Parser::parseOperationDefinition() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &type = parseOpType(currentToken.lexeme);
    const auto &name = parseNameNode();
    std::vector<shared::ast::InputValueDefinitionNode> parameters;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            parameters.push_back(parseInputValueDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    consume(SimpleTokenType::LEFT_BRACE);
    const auto &spec = parseOperationSpec();
    ast::FragmentSpec fragment = parseFragmentSpec();
    consume(SimpleTokenType::RIGHT_BRACE);
    location.endToken = currentToken;
    return { .location = location,
             .type = type,
             .name = name,
             .parameters = parameters,
             .spec = spec,
             .fragment = fragment };
};

ast::Argument Parser::parseArgument() {
    const auto &name = parseNameNode();
    consume(SimpleTokenType::COLON);
    const auto &argAliasName = parseNameNode();
    return { .name = name, .argAliasName = argAliasName };
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
    if (!isAhead(ComplexTokenType::SPREAD)) {
        return parseFieldSelectionNode();
    };
    consume(ComplexTokenType::SPREAD);
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
    std::optional<std::vector<shared::ast::InputValueDefinitionNode>> arguments;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        std::vector<shared::ast::InputValueDefinitionNode> args;
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            args.push_back(parseInputValueDefinitionNode());
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        arguments = args;
        consume(SimpleTokenType::RIGHT_PAREN);
        location.endToken = currentToken;
    };
    return { .selectionName = selectionName,
             .name = fieldName,
             .arguments = arguments };
};

ast::FieldSelectionNode Parser::parseFieldSelectionNode() {
    const auto &fieldSpec = parseObjectFieldSpec();
    std::optional<std::shared_ptr<ast::FragmentSpec>> spec;
    if (isAhead(SimpleTokenType::LEFT_BRACE)) {
        spec = std::make_shared<ast::FragmentSpec>(parseFragmentSpec());
    };
    shared::ast::NodeLocation location = { .startToken =
                                               fieldSpec.location.startToken,
                                           .endToken =
                                               fieldSpec.location.endToken,
                                           .source = source };
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
        throw shared::ParserError::wrongLexeme(currentToken, lexeme);
    };
};

ast::OpType Parser::parseOpType(const std::string &lexeme) {
    if (lexeme == "mutation") return ast::OpType::MUTATION;
    if (lexeme == "query") return ast::OpType::QUERY;
    if (lexeme == "subscription") return ast::OpType::SUBSCRIPTION;
    throw std::runtime_error("Unexpected token " + lexeme);
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
    if (raiseOnKeyword) shared::assertIsNotKeyword(currentToken);
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
        throw shared::ParserError(currentToken, "Expected literal node");
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
            throw shared::ParserError(currentToken, "Unexpected spread usage");
        };
    };
};

void Parser::advance() {
    index += 1;
    currentToken = tokens[index];
};