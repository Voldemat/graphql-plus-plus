#include <vector>

#include "./token.hpp"
#include "./tokens_accumulators.hpp"

void lexer::VectorTokensAccumulator::addToken(const lexer::GQLToken &token) {
    tokens.emplace_back(token);
};

std::vector<lexer::GQLToken> lexer::VectorTokensAccumulator::getTokens() const {
    return tokens;
};
