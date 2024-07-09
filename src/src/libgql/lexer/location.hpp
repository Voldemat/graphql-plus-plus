#pragma once

#include <format>
#include <ostream>
#include <string>

namespace lexer {
struct Location {
    unsigned int line = 1;
    unsigned int start = -1;
    unsigned int end = -1;

    void newLine();

    explicit operator std::string() const {
        return std::format("Line {} {}:{}", line, start, end);
    };
};
bool operator==(const Location &self, const Location &another) noexcept;
std::ostream &operator<<(std::ostream &os, const Location &self) noexcept;
};  // namespace lexer
