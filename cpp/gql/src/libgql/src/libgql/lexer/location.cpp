#include "./location.hpp"

#include <ostream>
#include <string>

namespace gql::lexer {
Location::Location(unsigned int line, unsigned int start, unsigned int end)
    : line{ line }, start{ start }, end{ end } {};

void Location::newLine() {
    line += 1;
    start = -1;
    end = -1;
};

void Location::lockStart() { isStartLocked = true; };

void Location::advance() {
    end += 1;
    if (!isStartLocked) start += 1;
};

void Location::unlockStart() {
    start = end;
    isStartLocked = false;
};

std::ostream &operator<<(std::ostream &os,
                         const lexer::Location &self) noexcept {
    os << (std::string)self << std::endl;
    return os;
};

bool operator==(const lexer::Location &self,
                const lexer::Location &another) noexcept {
    return another.getLine() == self.getLine() &&
           another.getStart() == self.getStart() &&
           another.getEnd() == self.getEnd();
};
};  // namespace gql::lexer
