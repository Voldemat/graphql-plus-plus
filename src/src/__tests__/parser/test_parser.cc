#include <gtest/gtest.h>

#include <filesystem>
#include <memory>
#include <vector>

#include "gtest/gtest.h"
#include "lexer/token.hpp"
#include "parser/parser.hpp"

TEST(ParserTest, BasicTest) {
    std::shared_ptr<SourceFile> source = std::make_shared<SourceFile>(
        std::filesystem::path("check.graphql")
    );
    const std::vector<GQLToken> tokens = {
        (GQLToken){
            .type = ComplexTokenType::IDENTIFIER,
            .lexeme = "type",
            .location = { .source = source, .line = 1, .start = 1, .end = 1 }
        },
        (GQLToken) {
            .type = ComplexTokenType::IDENTIFIER,
            .lexeme = "Product",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        },
        (GQLToken) {
            .type = SimpleTokenType::LEFT_BRACE,
            .lexeme = "{",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        },
        (GQLToken) {
            .type = ComplexTokenType::IDENTIFIER,
            .lexeme = "amount",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        },
        (GQLToken) {
            .type = SimpleTokenType::COLON,
            .lexeme = ":",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        },
        (GQLToken) {
            .type = ComplexTokenType::IDENTIFIER,
            .lexeme = "Int",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        },
        (GQLToken) {
            .type = SimpleTokenType::BANG,
            .lexeme = "!",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        },
        (GQLToken) {
            .type = SimpleTokenType::RIGHT_BRACE,
            .lexeme = "}",
            .location = { .source = source, .line = 1, .start = 1, .end = 1}
        }
    };
    parser::Parser parser(tokens);
    const auto ast = parser.getAstTree();
    ASSERT_EQ(ast.nodes.size(), 1);
    const auto node = ast.nodes[0];
    parser::ASTTypeDefinition typeDefinition = std::get<parser::ASTTypeDefinition>(node);
    ASSERT_STREQ(typeDefinition.name.c_str(), "Product");
    ASSERT_EQ(typeDefinition.isInput, false);
    ASSERT_EQ(typeDefinition.fields.size(), 1);
    ASSERT_TRUE(typeDefinition.fields.contains("amount"));
    parser::ASTTypeSpec typeSpec = typeDefinition.fields["amount"];
    parser::ASTGQLSimpleType type = std::get<parser::ASTGQLSimpleType>(typeSpec.type);
    ASSERT_EQ(type, parser::ASTGQLSimpleType::INT);
    ASSERT_EQ(typeSpec.nullable, false);
};
