#pragma once

#include <optional>

#include "./token.hpp"

namespace lexer {
class ITokensAccumulator {
public:
    ITokensAccumulator(const ITokensAccumulator &) = default;
    ITokensAccumulator(ITokensAccumulator &&) = delete;
    ITokensAccumulator &operator=(const ITokensAccumulator &) = default;
    ITokensAccumulator &operator=(ITokensAccumulator &&) = delete;
    virtual ~ITokensAccumulator() = default;
    ITokensAccumulator() = default;
    virtual void addToken(const GQLToken &token) = 0;
    void addMaybeToken(const std::optional<GQLToken> &maybeToken);
};
};  // namespace lexer
