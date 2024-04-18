#include <gtest/gtest.h>

#include <vector>

#include "gtest/gtest.h"
#include "lexer/token.hpp"
#include "parser/parser.hpp"

TEST(ParserTest, BasicTest) {
    const std::vector<GQLToken> tokens = {};
    parser::Parser parser(tokens);
};
