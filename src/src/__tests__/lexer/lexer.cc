#include "libgql/lexer/lexer.hpp"

#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>

#include <algorithm>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "./lexer_utils.hpp"
#include "gtest/gtest.h"
#include "libgql/lexer/token.hpp"

class Fixture : public testing::TestWithParam<TestCase> {};

using namespace lexer;
TEST_P(Fixture, TestLexer) {
    auto testCase = GetParam();
    std::istringstream buffer(testCase.schema);
    VectorTokensAccumulator accumulator;
    Lexer lexer(std::move(buffer), testCase.sourceFile, &accumulator);
    const auto &result = lexer.parse();
    if (!result.has_value()) {
        const std::vector<GQLToken> tokens = accumulator.getTokens();
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
    } else {
        const auto error = result.value();
        const auto errorLocation = error.getLocation();
        ASSERT_TRUE(testCase.error.has_value()) << error.what();
        const auto expectedError = testCase.error.value();
        const auto expectedErrorLocation = expectedError.getLocation();
        ASSERT_EQ(errorLocation.line, expectedErrorLocation.line);
        ASSERT_EQ(errorLocation.start, expectedErrorLocation.start);
        ASSERT_EQ(errorLocation.end, expectedErrorLocation.end);
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
