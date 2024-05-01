#include "./lexer.hpp"

#include <_ctype.h>

#include <algorithm>
#include <cctype>
#include <iostream>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <variant>

#include "./token.hpp"

using namespace lexer;

LexerError::LexerError(const std::string message,
                       const Location location) noexcept
    : location{ location }, message{message}{};
const char* LexerError::what() const noexcept {
    return message.c_str();
};
Lexer::Lexer(
    std::istringstream s,
    std::shared_ptr<SourceFile> source,
    ITokensAccumulator& tokensAccumulator
)
    : state{ source, tokensAccumulator } {
    stream.swap(s);
};

std::optional<LexerError> LexerState::feed(char c) noexcept{
    if (c == -1) {
        const auto& maybeToken = maybeExtractToken();
        if (maybeToken.has_value()) tokensAccumulator.addToken(maybeToken.value());
        return std::nullopt;
    } else if (c == '\n') {
        const auto& maybeToken = maybeExtractToken();
        if (maybeToken.has_value()) tokensAccumulator.addToken(maybeToken.value());
        location.line += 1;
        location.start = -1;
        location.end = -1;
        return std::nullopt;
    };
    std::optional<LexerError> result;
    if (type.has_value()) {
        result = feedWithType(c, type.value());
        if (result.has_value()) return result;
        if (!type.has_value()) {
            location.start = location.end;
        } else {
            location.end += 1;
            return std::nullopt;
        };
    };
    result = feedNew(c);
    if (result.has_value()) return result;
    return result;
    return std::nullopt;
};

std::optional<GQLTokenType> LexerState::getTypeForChar(char c) const noexcept {
    if (isalpha(c) || c == '_') return ComplexTokenType::IDENTIFIER;
    if (isnumber(c)) return ComplexTokenType::NUMBER;
    if (c == '"') return ComplexTokenType::STRING;
    switch (c) {
        case '!':
            return SimpleTokenType::BANG;
        case '=':
            return SimpleTokenType::EQUAL;
        case '(':
            return SimpleTokenType::LEFT_PAREN;
        case ')':
            return SimpleTokenType::RIGHT_PAREN;
        case '{':
            return SimpleTokenType::LEFT_BRACE;
        case '}':
            return SimpleTokenType::RIGHT_BRACE;
        case ';':
            return SimpleTokenType::SEMICOLON;
        case ':':
            return SimpleTokenType::COLON;
        case ',':
            return SimpleTokenType::COMMA;
        case '|':
            return SimpleTokenType::VSLASH;
        case '[':
            return SimpleTokenType::LEFT_BRACKET;
        case ']':
            return SimpleTokenType::RIGHT_BRACKET;
    };
    return std::nullopt;
};

void LexerState::extractAndSaveToken() noexcept {
    const auto& token = extractToken();
    tokensAccumulator.addToken(token);
};

std::optional<LexerError> LexerState::feedWithType(
    char c, ComplexTokenType tokenType) noexcept {
    switch (tokenType) {
        case ComplexTokenType::NUMBER: {
            if (!isnumber(c)) {
                extractAndSaveToken();
                return std::nullopt;
            };
            break;
        }
        case ComplexTokenType::STRING: {
            if (c == '"') {
                extractAndSaveToken();
                return std::nullopt;
            };
            break;
        }
        case ComplexTokenType::IDENTIFIER: {
            if (!(isalpha(c) || c == '_' || c == '-')) {
                extractAndSaveToken();
                return std::nullopt;
            };
            break;
        };
    };
    buffer += c;
    return std::nullopt;
};

GQLToken LexerState::extractToken() {
    const GQLToken token
        = { .type = type.value(), .lexeme = buffer, .location = location };
    location.start = location.end;
    type = std::nullopt;
    buffer = "";
    return token;
};
std::optional<GQLToken> LexerState::maybeExtractToken() noexcept {
    if (!type.has_value()) return std::nullopt;
    return extractToken();
};
std::optional<LexerError> Lexer::parse() noexcept {
    while (true) {
        char c = stream.get();
        auto result = state.feed(c);
        if (result.has_value()) return result;
        if (c == -1) break;
    };
    return std::nullopt;
};
std::optional<LexerError> LexerState::feedNew(char c) noexcept {
    location.start += 1;
    location.end += 1;
    if (c == ' ') return std::nullopt;
    const auto optTokenType = getTypeForChar(c);
    if (!optTokenType.has_value()) {
        location.end = std::max((int)location.end, 0);
        return LexerError(
            std::string("Cannot determine token type for char: \"") + c + '\"',
            location);
    };
    const auto tokenType = optTokenType.value();
    if (std::holds_alternative<SimpleTokenType>(tokenType)) {
        tokensAccumulator.addToken({ .type = tokenType,
                                                .lexeme = std::string() + c,
                                                .location = location });
        return std::nullopt;
    };
    const auto complexTokenType = std::get<ComplexTokenType>(tokenType);
    type = complexTokenType;
    if (complexTokenType != ComplexTokenType::STRING) {
        buffer = std::string() + c;
    };
    return std::nullopt;
}
