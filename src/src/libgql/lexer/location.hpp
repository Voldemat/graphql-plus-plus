#pragma once

#include <format>
#include <ostream>
#include <string>

namespace lexer {
class Location {
    unsigned int line = 1;
    unsigned int start = -1;
    unsigned int end = -1;
    bool isStartLocked = false;

public:
    explicit Location(unsigned int line, unsigned int start, unsigned int end);
    Location(){};

    void lockStart();
    void advance();
    void unlockStart();
    void newLine();

    inline unsigned int getLine() const { return line; }; 
    inline unsigned int getStart() const { return start; };
    inline unsigned int getEnd() const { return end; };

    explicit operator std::string() const {
        return std::format("Line {} {}:{}", line, start, end);
    };
};
bool operator==(const Location &self, const Location &another) noexcept;
std::ostream &operator<<(std::ostream &os, const Location &self) noexcept;
};  // namespace lexer
