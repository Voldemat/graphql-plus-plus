#include "./lexer.hpp"

#include <_ctype.h>

#include <cctype>
#include <functional>
#include <optional>
#include <sstream>
#include <string>

Lexer::Lexer(std::istringstream s) { stream.swap(s); };
std::optional<GQL_TOKEN*> Lexer::nextToken() noexcept {
    char c = stream.get();
    if (c == -1) return std::nullopt;
    switch (c) {
        case '=':
            return new GQL_TOKEN((GQLOperator){ .type = EQUAL });
        case ' ':
        case '\r':
        case '\t':
        case '\n':
            return nextToken();
        default: {
            if (isnumber(c)) return new GQL_TOKEN(buildNumber(c));
            if (isalpha(c)) return new GQL_TOKEN(buildIdentifier(c));
        }
    }
    return std::nullopt;
};

GQLNumberLiteral Lexer::buildNumber(char start) noexcept {
    std::string buffer =
        readWhilePredicateIsTrue(start, [](char c) { return isnumber(c); });
    return { .value =  std::stol(buffer) };
};

GQLVariable Lexer::buildIdentifier(char start) noexcept {
    std::string name =
        readWhilePredicateIsTrue(start, [](char c) { return isalpha(c); });
    return { .name = name };
};

std::string Lexer::readWhilePredicateIsTrue(
    char start, const std::function<bool(char)> predicate) {
    std::string buffer;
    buffer += start;
    char current;
    while (true) {
        current = stream.peek();
        if (!predicate(current)) {
            return buffer;
        }
        buffer += stream.get();
    };
};
