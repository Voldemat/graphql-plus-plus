#include "./lexer_error.hpp"

#include <format>
#include <string>

#include "./location.hpp"

namespace gql::lexer {
LexerError::LexerError(const std::string message, const Location location)
    : message{ message },
      location{ location },
      finalMessage{ std::format("{}:{}:{}: {}", location.getLine(),
                                location.getStart(), location.getEnd(),
                                message) } {};
const char *LexerError::what() const noexcept { return finalMessage.c_str(); };
};  // namespace gql::lexer
