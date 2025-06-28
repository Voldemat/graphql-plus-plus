#pragma once

#include <vector>

#include "./itokens_accumulator.hpp"
#include "./token.hpp"

namespace lexer {

class VectorTokensAccumulator : public ITokensAccumulator {
    std::vector<GQLToken> tokens;

public:
    void addToken(const GQLToken &token) override;
    [[nodiscard]] std::vector<GQLToken> getTokens() const;
};
};  // namespace lexer
