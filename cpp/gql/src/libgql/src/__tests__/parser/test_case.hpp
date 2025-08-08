#pragma once

#include <filesystem>
#include <ostream>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/file/server/ast.hpp"

struct ParserTestCase {
    std::filesystem::path filepath;
    std::vector<lexer::GQLToken> tokens;
    std::vector<parsers::file::server::ast::ASTNode> expectedNodes;

    friend std::ostream &operator<<(std::ostream &os,
                                    const ParserTestCase &self) {
        os << "TestCase(filename: " << self.filepath.filename() << ")";
        return os;
    };
};
