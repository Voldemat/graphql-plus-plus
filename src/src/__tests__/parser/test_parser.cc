#include <gtest/gtest.h>

#include <filesystem>
#include <memory>
#include <optional>
#include <vector>

#include "gtest/gtest.h"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/server/parser.hpp"

using namespace parsers::server;
using namespace parsers::server::ast;

TEST(ParserTest, BasicTest) {
    std::shared_ptr<SourceFile> source
        = std::make_shared<SourceFile>(std::filesystem::path("check.graphql"));
    const std::vector<GQLToken> tokens = {
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "type",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "Product",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::LEFT_BRACE,
          .lexeme = "{",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "amount",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::COLON,
          .lexeme = ":",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = ComplexTokenType::IDENTIFIER,
          .lexeme = "Int",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::BANG,
          .lexeme = "!",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } },
        { .type = SimpleTokenType::RIGHT_BRACE,
          .lexeme = "}",
          .location = { .source = source, .line = 1, .start = 1, .end = 1 } }
    };
    Parser parser(tokens);
    const auto ast = parser.getAstTree();
    ASSERT_EQ(ast.nodes.size(), 1);
    const auto node = ast.nodes[0];
    ASTGQLTypeDefinition typeDefinition
        = std::get<ASTGQLTypeDefinition>(std::get<ASTTypeDefinition>(node));
    ASSERT_STREQ(typeDefinition.name.c_str(), "Product");
    ASSERT_EQ(typeDefinition.fields.size(), 1);
    ASSERT_TRUE(typeDefinition.fields.contains("amount"));
    ASTTrivialTypeSpec typeSpec = std::get<ASTTrivialTypeSpec>(
        std::get<ASTLiteralTypeSpec>(typeDefinition.fields["amount"]));
    ASTGQLSimpleType type = std::get<ASTGQLSimpleType>(typeSpec.type);
    ASSERT_EQ(type, ASTGQLSimpleType::INT);
    ASSERT_EQ(typeSpec.nullable, false);
    ASSERT_EQ(typeDefinition.implements, std::nullopt);
};
