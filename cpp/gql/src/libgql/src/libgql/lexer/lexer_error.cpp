#include "./lexer_error.hpp"

#include <format>
#include <string>

#include "./location.hpp"

lexer::LexerError::LexerError(const std::string message,
                              const Location location)
    : message{ message },
      location{ location },
      finalMessage{ std::format("{}:{}:{}: {}", location.getLine(),
                                location.getStart(), location.getEnd(),
                                message) } {
      };
const char *lexer::LexerError::what() const noexcept {
    return finalMessage.c_str();
};
