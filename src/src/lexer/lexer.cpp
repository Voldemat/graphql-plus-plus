#include "./lexer.hpp"

#include <_ctype.h>

#include <algorithm>
#include <cctype>
#include <cstdio>
#include <iostream>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <tuple>
#include <utility>
#include <variant>
#include <vector>

#include "./token.hpp"

using namespace lexer;

LexerError::LexerError(const std::string message,
                       const Location location) noexcept
    : location{ location }, message{message}{};
const char* LexerError::what() const noexcept {
    return message.c_str();
};
Lexer::Lexer(std::istringstream s, std::shared_ptr<SourceFile> source)
    : state{ source } {
    stream.swap(s);
};

std::vector<GQLToken> Lexer::getTokens() {
    std::vector<GQLToken> tokensList;
    bool isAcknowledged = false;
    std::optional<GQLToken> token;
    char c;
    while (true) {
        c = stream.get();
        do {
            std::tie(isAcknowledged, token) = state.feed(c);
            if (token.has_value()) {
                tokensList.push_back(token.value());
            }
        } while (!isAcknowledged);
        isAcknowledged = false;
        token = std::nullopt;
        if (c == -1) break;
    };
    return tokensList;
};

std::tuple<bool, std::optional<GQLToken>> LexerState::feed(char c) {
    if (c == -1) {
        return std::make_pair(true, maybeExtractToken());
    };
    adjustLocation(c);
    std::tuple<bool, std::optional<GQLToken>> result;
    if (type.has_value()) {
        result = feedWithType(c, type.value());
    } else {
        result = feedNew(c);
    };
    return result;
};

void LexerState::adjustLocation(char c) {
    if (c == '\n') {
        location.line += 1;
        location.start = -1;
        location.end = -1;
    } else if (!type.has_value()) location.start += 1;
};

std::optional<GQLTokenType> LexerState::getTypeForChar(char c) const noexcept {
    if (isalpha(c)) return ComplexTokenType::IDENTIFIER;
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
    };
    return std::nullopt;
};

std::tuple<bool, std::optional<GQLToken>> LexerState::returnTuple(
    bool isAcknowledged, bool shouldReturnToken, char c 
) noexcept {
    if (isAcknowledged) location.end += 1;
    return std::make_pair(
            isAcknowledged,
            (shouldReturnToken)
                ? std::make_optional(extractToken())
                : std::nullopt
    );
};
std::tuple<bool, std::optional<GQLToken>> LexerState::feedWithType(
    char c, ComplexTokenType tokenType) {
    switch (tokenType) {
        case ComplexTokenType::NUMBER: {
            if (!isnumber(c)) return returnTuple(false, true, c);
        }
        case ComplexTokenType::STRING: {
            if (c == '"') return returnTuple(true, true, c);
        }
        case ComplexTokenType::IDENTIFIER: {
            if (!isalpha(c)) return returnTuple(false, true, c);
        }
        default: {
            buffer += c;
            return returnTuple(true, false, c);
        }
    };
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

std::tuple<bool, std::optional<GQLToken>> LexerState::feedNew(char c) {
    if (c == '\n') return std::make_pair(true, std::nullopt);
    else if (c == ' ') return returnTuple(true, false, c);
    const auto optTokenType = getTypeForChar(c);
    if (!optTokenType.has_value()) {
        location.end = std::max((int)location.end, 0);
        throw LexerError(
            std::string("Cannot determine token type for char: \"") + c + '\"',
            location);
    };
    const auto tokenType = optTokenType.value();
    if (std::holds_alternative<SimpleTokenType>(tokenType)) {
        location.end += 1;
        return std::make_pair(true, (GQLToken){ .type = tokenType,
                                                .lexeme = std::string() + c,
                                                .location = location });
    };
    const auto complexTokenType = std::get<ComplexTokenType>(tokenType);
    type = complexTokenType;
    if (complexTokenType != ComplexTokenType::STRING) {
        buffer = std::string() + c;
    };
    return returnTuple(true, false, c);
};
