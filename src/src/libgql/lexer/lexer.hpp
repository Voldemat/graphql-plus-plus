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
    Location location;
public:
    LexerError(const std::string message, const Location location) noexcept;
    [[nodiscard]] Location getLocation() const noexcept;
    [[nodiscard]] const char* what() const noexcept override;
};

class ITokensAccumulator {
public:
    ITokensAccumulator(const ITokensAccumulator &) = default;
    ITokensAccumulator(ITokensAccumulator &&) = delete;
    ITokensAccumulator &operator=(const ITokensAccumulator &) = default;
    ITokensAccumulator &operator=(ITokensAccumulator &&) = delete;
    virtual ~ITokensAccumulator() = default;
    ITokensAccumulator() = default;
    virtual void addToken(const GQLToken token) noexcept = 0;
};
class VectorTokensAccumulator : public ITokensAccumulator {
    std::vector<GQLToken> tokens;
    void addToken(const GQLToken token) noexcept override {
        tokens.push_back(token);
    };
public:
    VectorTokensAccumulator() = default;
    VectorTokensAccumulator(const VectorTokensAccumulator &) = default;
    VectorTokensAccumulator(VectorTokensAccumulator &&) = delete;
    VectorTokensAccumulator &operator=(const VectorTokensAccumulator &)
        = default;
    VectorTokensAccumulator &operator=(VectorTokensAccumulator &&) = delete;
    [[nodiscard]] std::vector<GQLToken> getTokens() const noexcept { return tokens; };
    ~VectorTokensAccumulator() override = default;
};
class LexerState {
    std::string buffer;
    std::optional<ComplexTokenType> type;
    Location location;
    ITokensAccumulator* tokensAccumulator;
    std::optional<LexerError> feedNew(char c) noexcept;
    [[nodiscard]] std::optional<GQLTokenType> getTypeForChar(char c) const noexcept;
    std::optional<LexerError> feedWithType(
        char c, ComplexTokenType tokenType) noexcept;
    GQLToken extractToken();
    std::optional<GQLToken> maybeExtractToken() noexcept;
    void extractAndSaveToken() noexcept;
public:
    LexerState(
        std::shared_ptr<SourceFile> source,
        ITokensAccumulator* tokensAccumulator
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
        ITokensAccumulator* tokensAccumulator
    );
    std::optional<LexerError> parse() noexcept;
};
};  // namespace lexer
#endif
