#pragma once

#include <filesystem>
#include <ostream>
#include <vector>
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/server/ast.hpp"

struct ParserTestCase {
    std::filesystem::path filepath;
    std::vector<GQLToken> tokens;
    parsers::server::ast::FileNodes expectedNodes;


    friend std::ostream &operator<<(std::ostream &os, const ParserTestCase &self) {
        os << "TestCase(filename: " << self.filepath.filename() << ")";
        return os;
    };
};
