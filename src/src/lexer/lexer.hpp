#ifndef GRAPHQL_LEXER
#define GRAPHQL_LEXER
#include <_ctype.h>

#include <optional>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

enum GQLTokenType {
    EQUAL = 1,
    LEFT_PAREN = 2,
    RIGHT_PAREN = 3,
    LEFT_BRACE = 4,
    RIGHT_BRACE = 5,
    BANG = 6,
    IDENTIFIER = 7,
    STRING = 8,
    NUMBER = 9,
    SEMICOLON = 10
};

std::optional<GQLTokenType> gqlTokenTypeFromString(std::string t) noexcept;
std::string gqlTokenTypeToString(GQLTokenType type) noexcept;

struct GQLToken {
    GQLTokenType type;
    std::string lexeme;
    unsigned int line = 1;
    unsigned int pos = 1;

    bool operator==(const GQLToken &token) const {
        return type == token.type && lexeme == token.lexeme &&
               line == token.line && token.pos;
    };
};

class Lexer {
    unsigned int line = 1;
    unsigned int pos = 0;
    std::optional<GQLToken> saved;
    std::istringstream stream;
    std::optional<GQLToken> changeStateAndNextToken(
        char c, GQLTokenType type, std::optional<GQLToken> state) noexcept;
    char nextChar() noexcept;
    std::optional<GQLToken> returnOrSaveToken(
        GQLTokenType type,
        std::string lexeme,
        std::optional<GQLToken> state
    ) noexcept;

public:
    Lexer(std::istringstream s);
    std::optional<GQLToken> nextToken(
        std::optional<GQLToken> state = std::nullopt) noexcept;
    std::vector<GQLToken> getTokens() noexcept;
};
std::ostream &operator<<(std::ostream &os, const GQLToken &self);
std::ostream &operator<<(std::ostream &os, const GQLTokenType &type);
#endif
