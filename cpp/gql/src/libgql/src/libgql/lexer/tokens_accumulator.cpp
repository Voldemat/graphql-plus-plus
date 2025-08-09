#include <vector>

#include "./token.hpp"
#include "./tokens_accumulators.hpp"

namespace gql::lexer {
void VectorTokensAccumulator::addToken(const GQLToken &token) {
    tokens.emplace_back(token);
};

std::vector<GQLToken> VectorTokensAccumulator::getTokens() const {
    return tokens;
};
};  // namespace gql::lexer
