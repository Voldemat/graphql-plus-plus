#include "./location.hpp"

#include <ostream>
#include <string>

void lexer::Location::newLine() {
    line += 1;
    start = -1;
    end = -1;
};

std::ostream &lexer::operator<<(std::ostream &os,
                         const lexer::Location &self) noexcept {
    os << (std::string)self << std::endl;
    return os;
};

bool lexer::operator==(const lexer::Location &self,
                const lexer::Location &another) noexcept {
    return another.line == self.line && another.start == self.start &&
           another.end == self.end;
};
