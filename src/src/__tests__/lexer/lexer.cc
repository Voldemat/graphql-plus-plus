#include "lexer/lexer.hpp"

#include <absl/strings/str_format.h>
#include <absl/strings/string_view.h>
#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>

#include <algorithm>
#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <optional>
#include <ostream>
#include <ranges>
#include <regex>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "gtest/gtest.h"

struct TestCase {
    const std::string filename;
    const std::string schema;
    const std::vector<GQLToken> expectedTokens;

    template <typename Sink>
    friend void AbslStringify(Sink &sink, const TestCase &tCase) {
        absl::Format(&sink,
                     "TestCase(filename: %s, schema: %s, tokens count: %d)",
                     tCase.filename, tCase.schema, tCase.expectedTokens.size());
    }
};

std::ostream &operator<<(std::ostream &os, const TestCase &self) {
    os << "TestCase(filename: " << self.filename << ", schema: ";
    os << self.schema << ", tokens: " << self.expectedTokens.size();
    os << ")";
    return os;
};
const std::filesystem::path casesPath =
    std::filesystem::path(__FILE__).parent_path().append("cases");
const std::regex casesRegex = std::regex(".*\\.case\\.json");
std::vector<TestCase> getCases() noexcept {
    std::vector<TestCase> cases;
    for (const auto &filepath :
         std::filesystem::directory_iterator(casesPath) |
             std::ranges::views::transform(
                 [](std::filesystem::directory_entry entry) {
                     return entry.path();
                 }) |
             std::ranges::views::filter([](std::filesystem::path path) {
                 return std::regex_match(path.string(), casesRegex);
             })) {
        std::cout << "File: " << filepath.filename() << std::endl;
        std::ifstream file(filepath);
        rapidjson::IStreamWrapper isw(file);
        rapidjson::Document d;
        d.ParseStream(isw);
        assert(d.IsObject());
        std::string schema = d["schema"].GetString();
        std::vector<GQLToken> expectedTokens;
        for (const auto &jsonToken : d["tokens"].GetArray()) {
            assert(jsonToken.IsObject());
            GQLToken token;
            token.type =
                gqlTokenTypeFromString(jsonToken["type"].GetString()).value();
            token.lexeme = jsonToken["lexeme"].GetString();
            token.line = jsonToken["line"].GetUint();
            token.pos = jsonToken["pos"].GetUint();
            expectedTokens.push_back(token);
        };
        cases.push_back({.filename = filepath.filename(),
                         .schema = schema,
                         .expectedTokens = expectedTokens});
    };
    return cases;
};

class Fixture : public testing::TestWithParam<TestCase> {};

TEST_P(Fixture, TestLexer) {
    auto testCase = GetParam();
    std::istringstream buffer(testCase.schema);
    Lexer lexer(std::move(buffer));
    const std::vector<GQLToken> tokens = lexer.getTokens();
    EXPECT_EQ(tokens.size(), testCase.expectedTokens.size());
    int index = 0;
    for (const auto &token : tokens) {
        const auto &expectedToken = testCase.expectedTokens[index];
        ASSERT_EQ(token.type, expectedToken.type)
            << "token: " << token
            << "\nexpectedToken: " << expectedToken;
        ASSERT_EQ(token.lexeme, expectedToken.lexeme)
            << "token: " << token
            << "\nexpectedToken: " << expectedToken;
        ASSERT_EQ(token.line, expectedToken.line)
            << "token: " << token
            << "\nexpectedToken: " << expectedToken;
        ASSERT_EQ(token.pos, expectedToken.pos)
            << "token: " << token
            << "\nexpectedToken: " << expectedToken;
        index++;
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
