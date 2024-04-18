#ifndef GRAPHQL_LEXER
#define GRAPHQL_LEXER

#include <exception>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

#include "./token.hpp"

namespace lexer {
class LexerError : public std::exception {
    const std::string message;
public:
    LexerError(const std::string message, const Location location) noexcept;
    const Location location;
    const char* what() const noexcept;
};

class LexerState {
    std::string buffer;
    std::optional<ComplexTokenType> type;
    Location location;
    std::tuple<bool, std::optional<GQLToken>> feedNew(char c);
    std::optional<GQLTokenType> getTypeForChar(char c) const noexcept;
    std::tuple<bool, std::optional<GQLToken>> feedWithType(
        char c, ComplexTokenType tokenType);
    GQLToken extractToken();
    std::optional<GQLToken> maybeExtractToken() noexcept;
    void adjustLocation(char c);
    std::tuple<bool, std::optional<GQLToken>> returnTuple(
        bool isAcknowledged, bool shouldReturnToken, char c
    ) noexcept;

public:
    LexerState(std::shared_ptr<SourceFile> source) noexcept {
        location.source = source;
    };
    std::tuple<bool, std::optional<GQLToken>> feed(char c);
};

class Lexer {
    LexerState state;
    std::istringstream stream;

public:
    Lexer(std::istringstream stream, std::shared_ptr<SourceFile> source);
    std::vector<GQLToken> getTokens();
};
};  // namespace lexer
#endif
