#include "./itokens_accumulator.hpp"

#include <optional>

#include "libgql/lexer/token.hpp"

namespace gql::lexer {
void ITokensAccumulator::addMaybeToken(
    const std::optional<GQLToken> &maybeToken) {
    if (maybeToken.has_value()) addToken(maybeToken.value());
};
};  // namespace gql::lexer
