#pragma once
#include <exception>
#include <string>

#include "./location.hpp"

namespace lexer {
class LexerError : public std::exception {
    std::string message;
    std::string finalMessage;
    Location location;

public:
    LexerError(const std::string message, const Location location);
    inline Location getLocation() const { return location; };
    [[nodiscard]] const char *what() const noexcept override;
};
};  // namespace lexer
