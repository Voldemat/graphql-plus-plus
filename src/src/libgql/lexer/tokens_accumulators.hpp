#pragma once

#include <vector>

#include "./itokens_accumulator.hpp"
#include "./token.hpp"

namespace lexer {

class VectorTokensAccumulator : public ITokensAccumulator {
    std::vector<GQLToken> tokens;

public:
    void addToken(const GQLToken &token) override;
    VectorTokensAccumulator() = default;
    VectorTokensAccumulator(const VectorTokensAccumulator &) = default;
    VectorTokensAccumulator(VectorTokensAccumulator &&) = delete;
    VectorTokensAccumulator &operator=(const VectorTokensAccumulator &) =
        default;
    VectorTokensAccumulator &operator=(VectorTokensAccumulator &&) = delete;
    [[nodiscard]] std::vector<GQLToken> getTokens() const;
    ~VectorTokensAccumulator() override = default;
};
};  // namespace lexer
