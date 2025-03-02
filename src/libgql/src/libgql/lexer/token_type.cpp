#include "./token_type.hpp"

#include <cctype>
#include <functional>
#include <iostream>
#include <map>
#include <optional>
#include <ostream>
#include <string>

#include "utils.hpp"

using namespace lexer;

const std::map<std::string, GQLTokenType> &stringToTokenType{
    { "EQUAL", SimpleTokenType::EQUAL },
    { "LEFT_PAREN", SimpleTokenType::LEFT_PAREN },
    { "RIGHT_PAREN", SimpleTokenType::RIGHT_PAREN },
    { "LEFT_BRACE", SimpleTokenType::LEFT_BRACE },
    { "RIGHT_BRACE", SimpleTokenType::RIGHT_BRACE },
    { "BANG", SimpleTokenType::BANG },
    { "SEMICOLON", SimpleTokenType::SEMICOLON },
    { "IDENTIFIER", ComplexTokenType::IDENTIFIER },
    { "STRING", ComplexTokenType::STRING },
    { "NUMBER", ComplexTokenType::NUMBER },
    { "COLON", SimpleTokenType::COLON },
    { "COMMA", SimpleTokenType::COMMA },
    { "VSLASH", SimpleTokenType::VSLASH },
    { "RIGHT_BRACKET", SimpleTokenType::RIGHT_BRACKET },
    { "LEFT_BRACKET", SimpleTokenType::LEFT_BRACKET },
    { "AT_SIGN", SimpleTokenType::AT_SIGN },
    { "SPREAD", ComplexTokenType::SPREAD },
    { "BOOLEAN", ComplexTokenType::BOOLEAN },
};

const std::map<GQLTokenType, std::string> &tokenTypeToString =
    flip_map(stringToTokenType);

std::optional<GQLTokenType> lexer::gqlTokenTypeFromString(const std::string t) {
    if (stringToTokenType.contains(t)) return stringToTokenType.at(t);
    return std::nullopt;
};

std::string lexer::gqlTokenTypeToString(GQLTokenType type) {
    return tokenTypeToString.at(type);
};

std::ostream &lexer::operator<<(std::ostream &os, const GQLTokenType &type) {
    os << gqlTokenTypeToString(type);
    return os;
};

const auto &numberCondition =
    std::function([](const char &c, const std::string &buffer) {
        const auto& hasFChar = buffer.back() == 'f';
        if (hasFChar) return false;
        const auto &isDigit = std::isdigit(c) != 0;
        const auto &hasPoint = buffer.contains('.');
        if (hasPoint && isDigit) return true;
        const auto& lastCharIsDigit = std::isdigit(buffer.back()) != 0;
        const auto &isChar = c == '.' || c == 'f';
        return isDigit || (lastCharIsDigit && isChar);
    });
const auto &stringCondition = std::function(
    [](const char &c, const std::string &buffer) { return c != '"'; });
const auto &spreadCondition = std::function(
    [](const char &c, const std::string &buffer) { return c == '.'; });
const auto &identifierCondition =
    std::function([](const char &c, const std::string &buffer) {
        return std::isalpha(c) || std::isdigit(c) || c == '_' || c == '-';
    });

const std::function<bool(const char &, const std::string &buffer)> &
lexer::getConditionForComplexTokenType(const ComplexTokenType &tokenType) {
    switch (tokenType) {
        case ComplexTokenType::NUMBER:
            return numberCondition;
        case ComplexTokenType::STRING:
            return stringCondition;
        case ComplexTokenType::SPREAD:
            return spreadCondition;
        case ComplexTokenType::BOOLEAN:
        case ComplexTokenType::IDENTIFIER:
            return identifierCondition;
    };
};

const std::map<char, GQLTokenType> &charToTokenType = {
    { '!', SimpleTokenType::BANG },
    { '=', SimpleTokenType::EQUAL },
    { '(', SimpleTokenType::LEFT_PAREN },
    { ')', SimpleTokenType::RIGHT_PAREN },
    { '{', SimpleTokenType::LEFT_BRACE },
    { '}', SimpleTokenType::RIGHT_BRACE },
    { ';', SimpleTokenType::SEMICOLON },
    { ':', SimpleTokenType::COLON },
    { ',', SimpleTokenType::COMMA },
    { '|', SimpleTokenType::VSLASH },
    { '[', SimpleTokenType::LEFT_BRACKET },
    { ']', SimpleTokenType::RIGHT_BRACKET },
    { '@', SimpleTokenType::AT_SIGN },
    { '"', ComplexTokenType::STRING },
    { '.', ComplexTokenType::SPREAD },
    { '$', ComplexTokenType::IDENTIFIER },
};

std::optional<GQLTokenType> lexer::tokenTypeFromChar(char c) {
    if (std::isalpha(c) || c == '_') return ComplexTokenType::IDENTIFIER;
    if (std::isdigit(c)) return ComplexTokenType::NUMBER;
    if (charToTokenType.contains(c)) return charToTokenType.at(c);
    return std::nullopt;
};
