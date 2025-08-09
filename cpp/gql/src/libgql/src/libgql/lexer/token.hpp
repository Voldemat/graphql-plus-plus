#pragma once

#include <iostream>
#include <ostream>
#include <string>

#include "./location.hpp"
#include "./token_type.hpp"

namespace gql::lexer {
struct GQLToken {
    GQLTokenType type;
    std::string lexeme;
    Location location;
};
bool operator==(const GQLToken &self, const GQLToken &token) noexcept;
std::ostream &operator<<(std::ostream &os, const GQLToken &self);
};  // namespace lexer
