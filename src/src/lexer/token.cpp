#include "./token.hpp"

#include <iostream>
#include <optional>
#include <ostream>
#include <string>
#include <variant>

std::optional<GQLTokenType> gqlTokenTypeFromString(
    const std::string t) noexcept {
    if (t == "EQUAL")
        return SimpleTokenType::EQUAL;
    else if (t == "LEFT_PAREN")
        return SimpleTokenType::LEFT_PAREN;
    else if (t == "RIGHT_PAREN")
        return SimpleTokenType::RIGHT_PAREN;
    else if (t == "LEFT_BRACE")
        return SimpleTokenType::LEFT_BRACE;
    else if (t == "RIGHT_BRACE")
        return SimpleTokenType::RIGHT_BRACE;
    else if (t == "BANG")
        return SimpleTokenType::BANG;
    else if (t == "SEMICOLON")
        return SimpleTokenType::SEMICOLON;
    else if (t == "IDENTIFIER")
        return ComplexTokenType::IDENTIFIER;
    else if (t == "STRING")
        return ComplexTokenType::STRING;
    else if (t == "NUMBER")
        return ComplexTokenType::NUMBER;
    else if (t == "COLON")
        return SimpleTokenType::COLON;
    else if (t == "COMMA")
        return SimpleTokenType::COMMA;
    return std::nullopt;
};

std::string gqlTokenTypeToString(GQLTokenType type) noexcept {
    if (std::holds_alternative<ComplexTokenType>(type)) {
        const auto &complexType = std::get<ComplexTokenType>(type);
        switch (complexType) {
            case ComplexTokenType::STRING:
                return "STRING";
            case ComplexTokenType::NUMBER:
                return "NUMBER";
            case ComplexTokenType::IDENTIFIER:
                return "IDENTIFIER";
        };
    } else {
        const auto &simpleType = std::get<SimpleTokenType>(type);
        switch (simpleType) {
            case SimpleTokenType::EQUAL:
                return "EQUAL";
            case SimpleTokenType::LEFT_PAREN:
                return "LEFT_PAREN";
            case SimpleTokenType::RIGHT_PAREN:
                return "RIGHT_PAREN";
            case SimpleTokenType::LEFT_BRACE:
                return "LEFT_BRACE";
            case SimpleTokenType::RIGHT_BRACE:
                return "RIGHT_BRACE";
            case SimpleTokenType::BANG:
                return "BANG";
            case SimpleTokenType::SEMICOLON:
                return "SEMICOLON";
            case SimpleTokenType::COLON:
                return "COLON";
            case SimpleTokenType::COMMA:
                return "COMMA";
        };
    };
};

std::ostream &operator<<(std::ostream &os, const GQLToken &self) {
    os << "GQLToken(type: " << self.type;
    os << ", lexeme: \"" << self.lexeme << "\", location: " << self.location << ")";
    return os;
};
std::ostream &operator<<(std::ostream &os, const GQLTokenType &type) {
    os << gqlTokenTypeToString(type);
    return os;
};

std::ostream& operator<<(std::ostream& os, const Location& self) noexcept {
    os << self.source->filepath.filename().string() << ":" << self.line << " ";
    os << self.start << ":" << self.end;
    return os;
};

bool operator==(const Location &self, const Location &another) noexcept {
    return another.line == self.line && another.start == self.start
           && another.end == self.end;
};
bool operator==(const GQLToken &self, const GQLToken &token) noexcept {
    return self.type == token.type && self.lexeme == token.lexeme
           && self.location == token.location;
};
