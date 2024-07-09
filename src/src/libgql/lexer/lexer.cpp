#include "./lexer.hpp"

#include <cctype>
#include <functional>
#include <optional>
#include <string>
#include <string_view>
#include <variant>

#include "./lexer_error.hpp"
#include "./token.hpp"
#include "./token_type.hpp"
#include "./itokens_accumulator.hpp"

using namespace lexer;

Lexer::Lexer(const std::string_view &s, ITokensAccumulator *tokensAccumulator)
    : state{ tokensAccumulator }, stream{ s } {};

std::optional<LexerError> Lexer::parse() {
    for (const auto &c : stream) {
        const auto &result = state.feed(c);
        if (result.has_value()) return result;
    };
    state.maybeExtractAndSaveToken();
    return std::nullopt;
};

std::optional<LexerError> LexerState::feed(char c) {
    if (c == '\n') {
        maybeExtractAndSaveToken();
        location.newLine();
        return std::nullopt;
    };
    if (type.has_value()) {
        feedWithType(c, type.value());
        if (!type.has_value()) {
            location.start = location.end;
        } else {
            location.end += 1;
            return std::nullopt;
        };
    };
    return feedNew(c);
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
    location.start = location.end;
    type = std::nullopt;
    buffer = "";
    return token;
};

void LexerState::feedWithType(char c, ComplexTokenType tokenType) {
    const auto &condition = getConditionForComplexTokenType(tokenType);
    if (!condition(c)) {
        extractAndSaveToken();
        return;
    };
    buffer += c;
};

void LexerState::extractAndSaveToken() {
    const auto &token = extractToken();
    tokensAccumulator->addToken(token);
};

std::optional<LexerError> LexerState::feedNew(char c) {
    location.start += 1;
    location.end += 1;
    if (c == ' ') return std::nullopt;
    const auto optTokenType = tokenTypeFromChar(c);
    if (!optTokenType.has_value()) {
        return LexerError(
            std::string("Cannot determine token type for char: \"") + c + '\"',
            location);
    };
    const auto &tokenType = optTokenType.value();
    if (!std::holds_alternative<ComplexTokenType>(tokenType)) {
        tokensAccumulator->addToken({ .type = tokenType,
                                      .lexeme = std::string(1, c),
                                      .location = location });
        return std::nullopt;
    };
    const auto &complexTokenType = std::get<ComplexTokenType>(tokenType);
    type = complexTokenType;
    if (complexTokenType != ComplexTokenType::STRING) {
        buffer = std::string(1, c);
    };
    return std::nullopt;
};
