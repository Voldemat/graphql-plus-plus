#include "./token_type.hpp"

#include <gtest/gtest.h>

#include <ios>
#include <iostream>
#include <ostream>
#include <string>
#include <vector>

struct TokenConditionCase {
    std::string name;
    std::string lexeme;
    std::vector<bool> expectedResults;
};
std::ostream &operator<<(std::ostream &os, const TokenConditionCase &self) {
    os << "TokenConditionCase(name: " << self.name << ", lexeme: ";
    os << self.lexeme << ", expectedResults: [" << std::boolalpha;
    for (const auto &r : self.expectedResults) {
        os << r;
        os << ", ";
    };
    os << "])";
    return os;
};

class TokenConditions : public testing::TestWithParam<TokenConditionCase> {};
TEST_P(TokenConditions, TestNumberCondition) {
    const auto &condition =
        lexer::getConditionForComplexTokenType(lexer::ComplexTokenType::NUMBER);
    const auto &param = GetParam();
    std::string buffer;
    std::vector<bool> results;
    for (const char &c : param.lexeme) {
        const auto &result = condition(c, buffer);
        if (result) buffer += c;
        results.push_back(result);
    };
    EXPECT_EQ(results.size(), param.expectedResults.size());
    int index = 0;
    for (const auto &r : results) {
        ASSERT_EQ(r, param.expectedResults[index]) << ", at: " << index;
        index += 1;
    };
};

std::vector<TokenConditionCase> getTokenConditionsCases() {
    return { { .name = "integer",
               .lexeme = "12a",
               .expectedResults = { true, true, false } },
             { .name = "float_point",
               .lexeme = "1.2",
               .expectedResults = { true, true, true } },
             { .name = "float_point_and_f",
               .lexeme = "1.2f",
               .expectedResults = { true, true, true, true } },
             { .name = "float_with_double_f",
               .lexeme = "1.2ff",
               .expectedResults = { true, true, true, true, false } } };
};

INSTANTIATE_TEST_SUITE_P(
    TokenConditionsCases, TokenConditions,
    testing::ValuesIn(getTokenConditionsCases()),
    [](const testing::TestParamInfo<TokenConditions::ParamType> &info) {
        return info.param.name;
    });
