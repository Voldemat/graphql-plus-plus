#include "./itokens_accumulator.hpp"
#include <optional>
#include "libgql/lexer/token.hpp"

void lexer::ITokensAccumulator::addMaybeToken(const std::optional<GQLToken> &maybeToken) {
    if (maybeToken.has_value()) addToken(maybeToken.value());
};
