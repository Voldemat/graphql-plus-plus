#include "./error.hpp"

#include <algorithm>
#include <format>
#include <limits>
#include <sstream>
#include <string>

#include "libgql/lexer/location.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"

namespace cli::formatting {
std::string formatLine(const std::string &line, const unsigned int &currentLine,
                       const gql::lexer::Location &location,
                       const gql::parsers::file::shared::ParserError &exc) {
    std::string linestr = std::to_string(currentLine);
    std::string buffer = std::format("{}: {}\n", linestr, line);
    if (currentLine == location.getLine()) {
        std::string underline;
        for (unsigned int i = 0; i < location.getStart() + 2 + linestr.size();
             i++) {
            underline += " ";
        };
        for (unsigned int i = location.getStart(); i < location.getEnd() + 1;
             i++) {
            underline += "~";
        };
        underline += std::format(" Error: {}\n", exc.what());
        buffer += underline;
    };
    return buffer;
};

std::string formatError(const gql::parsers::file::shared::ParserError &exc) {
    std::string buffer =
        std::format("{}\n", exc.getSource()->filepath.string());
    const gql::lexer::Location &location = exc.getLocation();
    unsigned int firstLineToShow = std::clamp((int)location.getLine() - 4, 1,
                                              std::numeric_limits<int>::max());
    unsigned int lastLineToShow = location.getLine() + 4;
    std::string line;
    unsigned int currentLine = 1;
    std::istringstream stream = (std::istringstream)exc.getSource()->buffer;
    while (std::getline(stream, line)) {
        if (firstLineToShow <= currentLine && currentLine <= lastLineToShow) {
            buffer += formatLine(line, currentLine, location, exc);
        };
        currentLine += 1;
    };
    return buffer;
};
};  // namespace cli::formatting
