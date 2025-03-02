#include "./lexer.hpp"

#include <cctype>
#include <functional>
#include <optional>
#include <string>
#include <string_view>
#include <variant>

#include "./itokens_accumulator.hpp"
#include "./lexer_error.hpp"
#include "./token.hpp"
#include "./token_type.hpp"

using namespace lexer;

Lexer::Lexer(const std::string_view &s, ITokensAccumulator *tokensAccumulator)
    : state{ tokensAccumulator }, stream{ s } {};

void Lexer::parse() {
    for (const auto &c : stream) {
        state.feed(c);
    };
    state.maybeExtractAndSaveToken();
};

void LexerState::feed(char c) {
    if (c == '\n') {
        maybeExtractAndSaveToken();
        location.newLine();
        return;
    };
    if (type.has_value()) {
        const auto& oldType = type.value();
        feedWithType(c, type.value());
        if (type.has_value()) return;
        if (oldType == ComplexTokenType::STRING && !type.has_value()) return;
    };
    feedNew(c);
};

void LexerState::maybeExtractAndSaveToken() {
    tokensAccumulator->addMaybeToken(maybeExtractToken());
};

std::optional<GQLToken> LexerState::maybeExtractToken() {
    if (!type.has_value()) return std::nullopt;
    return extractToken();
};

GQLToken LexerState::extractToken() {
    if (buffer == "true" || buffer == "false") {
        type = ComplexTokenType::BOOLEAN;
    };
    const GQLToken token = { .type = type.value(),
                             .lexeme = buffer,
                             .location = location };
    type = std::nullopt;
    buffer = "";
    location.unlockStart();
    return token;
};

void LexerState::feedWithType(char c, ComplexTokenType tokenType) {
    const auto &condition = getConditionForComplexTokenType(tokenType);
    if (!condition(c, buffer)) {
        extractAndSaveToken();
        return;
    };
    buffer += c;
    location.advance();
    return;
};

void LexerState::extractAndSaveToken() {
    const auto &token = extractToken();
    tokensAccumulator->addToken(token);
};

void LexerState::feedNew(char c) {
    location.advance();
    if (c == ' ') return;
    const auto optTokenType = tokenTypeFromChar(c);
    if (!optTokenType.has_value()) {
        throw LexerError(
            std::string("Cannot determine token type for char: \"") + c + '\"',
            location);
    };

    const auto &tokenType = optTokenType.value();
    if (!std::holds_alternative<ComplexTokenType>(tokenType)) {
        tokensAccumulator->addToken({ .type = tokenType,
                                      .lexeme = std::string(1, c),
                                      .location = location });
        return;
    };

    const auto &complexTokenType = std::get<ComplexTokenType>(tokenType);
    type = complexTokenType;
    if (complexTokenType != ComplexTokenType::STRING) {
        buffer = std::string(1, c);
    };
    location.lockStart();
};
