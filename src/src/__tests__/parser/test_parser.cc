#include <gtest/gtest.h>

#include <memory>
#include <vector>

#include "gtest/gtest.h"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/server/parser.hpp"

using namespace parsers::server;
using namespace parsers::server::ast;

TEST(ParserTest, BasicTest) {
    const std::vector<GQLToken> tokens = {
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "type",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "Product",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::LEFT_BRACE,
          .lexeme = "{",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "amount",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::COLON,
          .lexeme = ":",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "Int",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::BANG,
          .lexeme = "!",
          .location = { .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::RIGHT_BRACE,
          .lexeme = "}",
          .location = { .line = 1, .start = 1, .end = 1 } }
    };
    std::shared_ptr<ast::SourceFile> source =
        std::make_shared<ast::SourceFile>();
    Parser parser(tokens, source);
    const auto ast = parser.parse();
    ASSERT_EQ(ast.definitions.size(), 1);
    ASSERT_EQ(ast.extensions.size(), 0);
    ASSERT_EQ(ast.source, source);
    const ast::ObjectDefinitionNode def =
        std::get<ast::ObjectDefinitionNode>(ast.definitions[0]);
    ASSERT_EQ(def.name.name, "Product");
    ASSERT_EQ(def.name.location.startToken, tokens[1]);
    ASSERT_EQ(def.name.location.endToken, tokens[1]);
    ASSERT_EQ(def.fields.size(), 1);
    ASSERT_EQ(def.location.startToken, tokens[0]);
    ASSERT_EQ(def.location.endToken, tokens.back());
};
