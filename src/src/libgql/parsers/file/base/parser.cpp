#include "./parser.hpp"

#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <variant>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/file/shared/shared.hpp"

using namespace lexer;

namespace parsers::file {
BaseParser::BaseParser(const std::vector<GQLToken> &tokens,
                       const std::shared_ptr<shared::ast::SourceFile> &source)
    : tokens{ tokens }, source{ source }, currentToken{ tokens[0] } {};

shared::ast::NameNode BaseParser::parseNameNode(bool raiseOnKeyword) {
    consume(ComplexTokenType::IDENTIFIER);
    if (raiseOnKeyword) shared::assertIsNotKeyword(currentToken, source);
    return { .location = { .startToken = currentToken,
                           .endToken = currentToken,
                           .source = source },
             .name = currentToken.lexeme };
};

shared::ast::InputValueDefinitionNode
BaseParser::parseInputValueDefinitionNode() {
    const auto &nameNode = parseNameNode();
    const auto &startToken = currentToken;
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
             .defaultValue = defaultValue };
};

void BaseParser::advance() {
    index += 1;
    currentToken = tokens[index];
};

std::optional<shared::ast::LiteralIntNode> BaseParser::parseLiteralIntNode() {
    try {
        return (shared::ast::LiteralIntNode){
            .location = { .startToken = currentToken,
                          .endToken = currentToken,
                          .source = source },
            .value = std::stoi(currentToken.lexeme)
        };
    } catch (const std::invalid_argument &) {
        return std::nullopt;
    } catch (const std::out_of_range &) {
        return std::nullopt;
    };
};

shared::ast::LiteralNode BaseParser::parseLiteralNode() {
    advance();
    if (!std::holds_alternative<ComplexTokenType>(currentToken.type)) {
        throw shared::ParserError(currentToken, "Expected literal node",
                                  source);
    };
    switch (std::get<ComplexTokenType>(currentToken.type)) {
        case ComplexTokenType::NUMBER: {
            const auto& node = parseLiteralIntNode();
            if (node.has_value()) return node.value();
            return (shared::ast::LiteralFloatNode){
                .location = {}, .value = std::stof(currentToken.lexeme)
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
            throw shared::ParserError(currentToken,
                                      "Unexpected spread operator", source);
        }
    };
};

shared::ast::TypeNode BaseParser::parseTypeNode() {
    if (isAhead(SimpleTokenType::LEFT_BRACKET)) {
        return parseListTypeNode();
    } else {
        return parseNamedTypeNode();
    };
};

shared::ast::NamedTypeNode BaseParser::parseNamedTypeNode() {
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

shared::ast::ListTypeNode BaseParser::parseListTypeNode() {
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

std::optional<GQLToken> BaseParser::lookahead() {
    if (index + 1 >= tokens.size()) return std::nullopt;
    return tokens.at(index + 1);
};

void BaseParser::consume(const GQLTokenType type) {
    index += 1;
    currentToken = tokens[index];
    if (currentToken.type != type) {
        throw shared::ParserError::wrongType(currentToken, type, source);
    };
};

void BaseParser::consumeIdentifier() {
    consume(ComplexTokenType::IDENTIFIER);
    shared::assertIsNotKeyword(currentToken, source);
};

bool BaseParser::consumeIfIsAhead(GQLTokenType expectedType) {
    bool tokenIsAhead = isAhead(expectedType);
    if (tokenIsAhead) {
        consume(expectedType);
    };
    return tokenIsAhead;
};

bool BaseParser::isAhead(GQLTokenType expectedType) {
    const auto &t = lookahead();
    if (!t.has_value()) return false;
    return t->type == expectedType;
};

bool BaseParser::isAheadByLexeme(const std::string &lexeme) {
    const auto &t = lookahead();
    if (!t.has_value()) return false;
    return t.value().lexeme == lexeme;
};

bool BaseParser::consumeIdentifierByLexemeIfIsAhead(const std::string &lexeme) {
    const auto &t = lookahead();
    if (!t.has_value()) return false;
    bool isAhead = t.value().lexeme == lexeme;
    if (isAhead) {
        consumeIdentifierByLexeme(lexeme);
    };
    return isAhead;
};

void BaseParser::consumeIdentifierByLexeme(const std::string &lexeme) {
    consumeIdentifier();
    if (currentToken.lexeme != lexeme) {
        throw shared::ParserError::wrongLexeme(currentToken, lexeme, source);
    };
};
};  // namespace parsers::file
