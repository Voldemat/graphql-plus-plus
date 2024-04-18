#include "lexer/lexer.hpp"

#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>

#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "gtest/gtest.h"
#include "lexer/token.hpp"
#include "./lexer_utils.hpp"

class Fixture : public testing::TestWithParam<TestCase> {};

using namespace lexer;
TEST_P(Fixture, TestLexer) {
    auto testCase = GetParam();
    std::istringstream buffer(testCase.schema);
    Lexer lexer(std::move(buffer), testCase.sourceFile);
    try {
        const std::vector<GQLToken> tokens = lexer.getTokens();
        EXPECT_EQ(tokens.size(), testCase.expectedTokens.size());
        int index = 0;
        for (const auto &token : tokens) {
            const auto &expectedToken = testCase.expectedTokens[index];
            ASSERT_EQ(token.type, expectedToken.type)
                << "token: " << token << "\nexpectedToken: " << expectedToken;
            ASSERT_EQ(token.lexeme, expectedToken.lexeme)
                << "token: " << token << "\nexpectedToken: " << expectedToken;
            ASSERT_EQ(token.location, expectedToken.location)
                << "token: " << token << "\nexpectedToken: " << expectedToken;
            index++;
        };
    } catch (LexerError &error) {
        ASSERT_TRUE(testCase.error.has_value())
            << error.what();
        const auto expectedError = *testCase.error.value();
        ASSERT_EQ(error.location.line, expectedError.location.line);
        ASSERT_EQ(error.location.start, expectedError.location.start);
        ASSERT_EQ(error.location.end, expectedError.location.end);
        ASSERT_STREQ(error.what(), expectedError.what());
        return;
    };
};
INSTANTIATE_TEST_SUITE_P(
    LexerCasesTests, Fixture, testing::ValuesIn(getCases()),
    [](const testing::TestParamInfo<Fixture::ParamType> &info) {
        std::string testname = info.param.filename;
        std::replace(testname.begin(), testname.end(), '.', '_');
        std::replace(testname.begin(), testname.end(), '-', '_');
        return testname;
    });
