#include <gtest/gtest.h>
#include <optional>
#include <sstream>
#include <utility>

#include "./lexer.hpp"


TEST(Lexer, TestLexer) {
    std::istringstream buffer("a = 1\nbasad = 2");
    Lexer lexer(std::move(buffer));
    std::optional<GQL_TOKEN*> token;
    token = lexer.nextToken();
    EXPECT_TRUE(token.has_value());
    EXPECT_EQ(std::get<GQLVariable>(*token.value()).name, "a");
    token = lexer.nextToken();
    EXPECT_TRUE(token.has_value());
    EXPECT_EQ(std::get<GQLOperator>(*token.value()).type, EQUAL);
    token = lexer.nextToken();
    EXPECT_TRUE(token.has_value());
    EXPECT_EQ(std::get<GQLNumberLiteral>(*token.value()).value, 1);
    token = lexer.nextToken();
    EXPECT_TRUE(token.has_value());
    EXPECT_EQ(std::get<GQLVariable>(*token.value()).name, "basad");
    token = lexer.nextToken();
    EXPECT_TRUE(token.has_value());
    EXPECT_EQ(std::get<GQLOperator>(*token.value()).type, EQUAL);
    token = lexer.nextToken();
    EXPECT_TRUE(token.has_value());
    EXPECT_EQ(std::get<GQLNumberLiteral>(*token.value()).value, 2);
    token = lexer.nextToken();
    EXPECT_EQ(token, std::nullopt);
}
