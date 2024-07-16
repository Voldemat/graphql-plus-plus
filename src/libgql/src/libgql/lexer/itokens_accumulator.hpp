#pragma once

#include <optional>

#include "./token.hpp"

namespace lexer {
class ITokensAccumulator {
public:
    virtual void addToken(const GQLToken &token) = 0;
    void addMaybeToken(const std::optional<GQLToken> &maybeToken);
};
};  // namespace lexer
