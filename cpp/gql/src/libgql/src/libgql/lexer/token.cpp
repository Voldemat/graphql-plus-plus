#include "./token.hpp"

#include <cctype>
#include <iostream>
#include <ostream>
#include <string>

namespace gql::lexer {
std::ostream &operator<<(std::ostream &os, const GQLToken &self) {
    os << "GQLToken(type: " << self.type;
    os << ", lexeme: \"" << self.lexeme << "\", location: " << self.location
       << ")";
    return os;
};

bool operator==(const GQLToken &self, const GQLToken &token) noexcept {
    return self.type == token.type && self.lexeme == token.lexeme &&
           self.location == token.location;
};
};  // namespace gql::lexer
