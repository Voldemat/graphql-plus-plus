#include "./shared.hpp"

#include <algorithm>
#include <memory>
#include <sstream>
#include <string>

#include "./ast.hpp"
#include "./parser_error.hpp"
#include "libgql/lexer/location.hpp"
#include "libgql/lexer/token.hpp"

namespace gql::parsers::file::shared {
void assertIsNotKeyword(const lexer::GQLToken token,
                        const std::shared_ptr<ast::SourceFile> &source) {
    if (isKeyword(token.lexeme))
        throw ParserError::identifierIsKeyword(token, source);
};

const bool isKeyword(const std::string lexeme) {
    return (lexeme == "type" || lexeme == "query" || lexeme == "input" ||
            lexeme == "extend" || lexeme == "directive");
};

std::string getSourceText(const std::string &sourceBuffer,
                          const lexer::Location &startTokenLocation,
                          const lexer::Location &endTokenLocation) {
    std::string buffer;
    std::string line;
    unsigned int currentLine = 1;
    std::istringstream stream = (std::istringstream)sourceBuffer;
    while (std::getline(stream, line)) {
        if (startTokenLocation.getLine() == currentLine) {
            buffer += std::string(line.begin() + startTokenLocation.getStart(),
                                  line.end());
        }
        if (endTokenLocation.getLine() == currentLine) {
            buffer += std::string(line.begin() + endTokenLocation.getStart(),
                                  line.begin() + endTokenLocation.getEnd() + 1);
        }
        if (currentLine > startTokenLocation.getLine() &&
            currentLine < endTokenLocation.getLine()) {
            buffer += line;
        };
        currentLine += 1;
    };

    std::string::iterator new_end = std::unique(
        buffer.begin(), buffer.end(), [](const char &lhs, const char &rhs) {
            return (lhs == ' ') && (lhs == rhs);
        });
    buffer.erase(new_end, buffer.end());
    return buffer;
}
}  // namespace gql::parsers::file::shared
