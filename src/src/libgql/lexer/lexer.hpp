#ifndef GRAPHQL_LEXER
#define GRAPHQL_LEXER

#include <optional>
#include <string>
#include <string_view>

#include "./itokens_accumulator.hpp"
#include "./lexer_error.hpp"
#include "./location.hpp"
#include "./token.hpp"
#include "./token_type.hpp"

namespace lexer {
class LexerState {
    std::string buffer;
    std::optional<ComplexTokenType> type;
    Location location;
    ITokensAccumulator *tokensAccumulator;
    std::optional<LexerError> feedNew(char c);
    void feedWithType(char c, ComplexTokenType tokenType);
    GQLToken extractToken();
    std::optional<GQLToken> maybeExtractToken();
    void extractAndSaveToken();

public:
    LexerState(ITokensAccumulator *tokensAccumulator)
        : tokensAccumulator{ tokensAccumulator } {};
    std::optional<LexerError> feed(char c);
    void maybeExtractAndSaveToken();
};

class Lexer {
    LexerState state;
    std::string_view stream;

public:
    Lexer(const std::string_view &s, ITokensAccumulator *tokensAccumulator);
    std::optional<LexerError> parse();
};
};  // namespace lexer
#endif
