#include "./lexer.hpp"

#include <_ctype.h>

#include <cctype>
#include <iostream>
#include <optional>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

Lexer::Lexer(std::istringstream s) { stream.swap(s); };

char Lexer::nextChar() noexcept {
    pos++;
    char c = stream.get();
    return c;
};

std::optional<GQLToken> Lexer::nextToken(
    std::optional<GQLToken> state
) noexcept {
    GQLToken token;
    if (saved.has_value()) {
        token = saved.value();
        saved.reset();
        return token;
    };
    char c = nextChar();
    std::cout << "Char: " << c << std::endl;
    if (c == -1) return std::nullopt;
    switch (c) {
        case '=':
            return returnOrSaveToken(EQUAL, "=", state);
        case ' ':
        case '\r':
        case '\t':
            if (state.has_value()) return state;
            return nextToken();
        case '\n':
            line++;
            pos = 0;
            if (state.has_value()) return state;
            return nextToken();
        case '{':
            return returnOrSaveToken(LEFT_BRACE, "{", state);
        case '}':
            return returnOrSaveToken(RIGHT_BRACE, "}", state);
        case ':':
            return returnOrSaveToken(SEMICOLON, ":", state);
        case '!':
            return returnOrSaveToken(BANG, "!", state);
        case '(':
            return returnOrSaveToken(LEFT_PAREN, "(", state);
        case ')':
            return returnOrSaveToken(RIGHT_PAREN, ")", state);
        case '"':
            if (state.has_value()) {
                if (state.value().type == STRING) return state;
                saved = (GQLToken){ .type = STRING, .lexeme = "", .line = line, .pos = pos };
                return state;
            };
            return nextToken((GQLToken){ .type = STRING, .lexeme = "", .line = line, .pos = pos});
        default: {
            if (isnumber(c)) {
                return changeStateAndNextToken(
                    c,
                    NUMBER,
                    state
                );
            };
            if (isalpha(c)) {
                return changeStateAndNextToken(
                    c,
                    IDENTIFIER,
                    state
                );
            };
        }
    };
    if (state.has_value()) return state;
    return std::nullopt;
};

std::optional<GQLToken> Lexer::changeStateAndNextToken(
    char c,
    GQLTokenType type,
    std::optional<GQLToken> state
) noexcept {
    if (!state.has_value()) {
        state = (GQLToken){
            .type = type,
            .lexeme = std::string(1, c),
            .line = line,
            .pos = pos
        };
    } else {
        state.value().lexeme += c;
    };
    return nextToken(state);
};

std::vector<GQLToken> Lexer::getTokens() noexcept {
    std::vector<GQLToken> tokens;
    std::optional<GQLToken> current;
    while (true) {
        current = nextToken();
        if (!current.has_value()) break;
        tokens.push_back(current.value());
    };
    return tokens;
};

std::optional<GQLTokenType> gqlTokenTypeFromString(std::string t) noexcept {
    if (t == "EQUAL") return EQUAL;
    else if (t == "LEFT_PAREN") return LEFT_PAREN;
    else if (t == "RIGHT_PAREN") return RIGHT_PAREN;
    else if (t == "LEFT_BRACE") return LEFT_BRACE;
    else if (t == "RIGHT_BRACE") return RIGHT_BRACE;
    else if (t == "BANG") return BANG;
    else if (t == "IDENTIFIER") return IDENTIFIER;
    else if (t == "STRING") return STRING;
    else if (t == "NUMBER") return NUMBER;
    else if (t == "SEMICOLON") return SEMICOLON;
    return std::nullopt;
};

std::string gqlTokenTypeToString(GQLTokenType type) noexcept {
    switch (type) {
        case EQUAL: return "EQUAL";
        case LEFT_PAREN: return "LEFT_PAREN";
        case RIGHT_PAREN: return "RIGHT_PAREN";
        case LEFT_BRACE: return "LEFT_BRACE";
        case RIGHT_BRACE: return "RIGHT_BRACE";
        case BANG: return "BANG";
        case IDENTIFIER: return "IDENTIFIER";
        case STRING: return "STRING";
        case NUMBER: return "NUMBER";
        case SEMICOLON: return "SEMICOLON";
    };
};

std::optional<GQLToken> Lexer::returnOrSaveToken(
    GQLTokenType type,
    std::string lexeme,
    std::optional<GQLToken> state
) noexcept {
    GQLToken token = { .type = type, .lexeme = lexeme, .line = line, .pos = pos };
    if (state.has_value()) {
        saved = token;
        return state;
    };
    return token;
};

std::ostream &operator<<(std::ostream &os, const GQLToken &self) {
    os << "GQLToken(type: " << gqlTokenTypeToString(self.type);
    os << ", lexeme: \"" << self.lexeme << "\", line: " << self.line;
    os << ", pos: " << self.pos << ")";
    return os;
};
std::ostream &operator<<(std::ostream &os, const GQLTokenType &type) {
    os << gqlTokenTypeToString(type);
    return os;
};
