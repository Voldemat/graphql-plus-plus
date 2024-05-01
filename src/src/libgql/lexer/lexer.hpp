#ifndef GRAPHQL_LEXER
#define GRAPHQL_LEXER

#include <exception>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

#include "./token.hpp"

namespace lexer {
class LexerError : public std::exception {
    std::string message;
public:
    LexerError(const std::string message, const Location location) noexcept;
    Location location;
    const char* what() const noexcept;
};

class ITokensAccumulator {
public:
    virtual ~ITokensAccumulator() {};
    virtual void addToken(const GQLToken token) noexcept = 0;
};
class VectorTokensAccumulator : public ITokensAccumulator {
    std::vector<GQLToken> tokens;
    virtual void addToken(const GQLToken token) noexcept {
        tokens.push_back(token);
    };
public:
    std::vector<GQLToken> getTokens() const noexcept {
        return tokens;
    };
    virtual ~VectorTokensAccumulator() {};
};
class LexerState {
    std::string buffer;
    std::optional<ComplexTokenType> type;
    Location location;
    ITokensAccumulator& tokensAccumulator;
    std::optional<LexerError> feedNew(char c) noexcept;
    std::optional<GQLTokenType> getTypeForChar(char c) const noexcept;
    std::optional<LexerError> feedWithType(
        char c, ComplexTokenType tokenType) noexcept;
    GQLToken extractToken();
    std::optional<GQLToken> maybeExtractToken() noexcept;
    void extractAndSaveToken() noexcept;
public:
    LexerState(
        std::shared_ptr<SourceFile> source,
        ITokensAccumulator& tokensAccumulator
    ) noexcept : tokensAccumulator{tokensAccumulator} {
        location.source = source;
    };
    std::optional<LexerError> feed(char c) noexcept;
};

class Lexer {
    LexerState state;
    std::istringstream stream;
public:
    Lexer(
        std::istringstream stream,
        std::shared_ptr<SourceFile> source,
        ITokensAccumulator& tokensAccumulator
    );
    std::optional<LexerError> parse() noexcept;
};
};  // namespace lexer
#endif
