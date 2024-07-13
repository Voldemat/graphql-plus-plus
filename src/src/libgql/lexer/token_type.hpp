#pragma once

#include <functional>
#include <optional>
#include <ostream>
#include <string>
#include <variant>

namespace lexer {
enum class SimpleTokenType {
    EQUAL,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    BANG,
    SEMICOLON,
    COLON,
    COMMA,
    VSLASH,
    LEFT_BRACKET,
    RIGHT_BRACKET
};
enum class ComplexTokenType { IDENTIFIER, STRING, NUMBER, BOOLEAN, SPREAD };
using GQLTokenType = std::variant<SimpleTokenType, ComplexTokenType>;

std::ostream &operator<<(std::ostream &os, const GQLTokenType &type);

std::string gqlTokenTypeToString(GQLTokenType type);
std::optional<GQLTokenType> gqlTokenTypeFromString(std::string t);

const std::function<bool(const char &, const std::string &)> &
getConditionForComplexTokenType(const ComplexTokenType &tokenType);

std::optional<GQLTokenType> tokenTypeFromChar(char c);
};  // namespace lexer
