#include <gtest/gtest.h>
#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>

#include <algorithm>
#include <cassert>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <memory>
#include <ostream>
#include <ranges>
#include <regex>
#include <string>
#include <variant>
#include <vector>

#include "__tests__/parser/test_case.hpp"
#include "gtest/gtest.h"
#include "libgql/json/parsers/lexer/lexer.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/server/parser.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "utils.hpp"

using namespace parsers::file;
using namespace parsers::file::server;
using namespace parsers::file::server::ast;

struct ASTNameDifference {
    const ASTNode *leftNode;
    const ASTNode *rightNode;
    const std::string *leftNodeName;
    const std::string *rightNodeName;

    explicit operator std::string() const {
        return std::format(
            "ASTNameDifference(leftNodeName: \"{}\", rightNodeName: \"{}\")",
            *leftNodeName, *rightNodeName);
    };
};

template <class... _Types>
class CustomVariant : public std::variant<_Types...> {
public:
    explicit operator std::string() const {
        return std::visit<std::string>(overloaded{ [](const auto &arg) {
                                           return static_cast<std::string>(arg);
                                       } },
                                       *this);
    };

    friend std::ostream &operator<<(std::ostream &os,
                                    const CustomVariant<_Types...> &var) {
        os << (std::string)var;
        return os;
    };
};

using FileNodesDifference = CustomVariant<ASTNameDifference>;

class FileNodesComparator {
public:
    std::vector<FileNodesDifference> compare(
        const ast::FileNodes &leftNodes,
        const ast::FileNodes &rightNodes) const {
        std::vector<FileNodesDifference> differences;
        return differences;
    };
};

class ParserFixture : public testing::TestWithParam<ParserTestCase> {};
TEST_P(ParserFixture, TestParser) {
    auto param = GetParam();
    Parser parser(param.tokens, param.expectedNodes.source);
    const auto ast = parser.parse();
    FileNodesComparator comparator;
    const auto &differences = comparator.compare(ast, param.expectedNodes);
    EXPECT_EQ(differences.size(), 0);
    for (const auto &diff : differences) {
        std::cout << diff << std::endl;
    };
};

const std::filesystem::path casesPath =
    std::filesystem::path(__FILE__).parent_path().append("cases");
const std::regex casesRegex = std::regex(".*\\.case\\.json");
std::vector<ParserTestCase> getParserCases() {
    std::vector<ParserTestCase> cases;
    for (const auto &filepath :
         std::filesystem::directory_iterator(casesPath) |
             std::ranges::views::transform(
                 [](std::filesystem::directory_entry entry) {
                     return entry.path();
                 }) |
             std::ranges::views::filter([](std::filesystem::path path) {
                 return std::regex_match(path.filename().string(), casesRegex);
             })) {
        std::ifstream file(filepath);
        rapidjson::IStreamWrapper isw(file);
        rapidjson::Document d;
        d.ParseStream(isw);
        assert(d.IsObject());
        ParserTestCase testCase = {
            .filepath = filepath,
            .tokens =
                json::parsers::lexer::parseTokensArray(d["tokens"].GetArray()),
            .expectedNodes = { .source =
                                   std::make_shared<shared::ast::SourceFile>(
                                       filepath, ""),
                               .definitions = {},
                               .extensions = {} }
        };
        cases.push_back(testCase);
    };
    return cases;
};

INSTANTIATE_TEST_SUITE_P(
    ParserCasesTests, ParserFixture, testing::ValuesIn(getParserCases()),
    [](const testing::TestParamInfo<ParserFixture::ParamType> &info) {
        auto name = info.param.filepath.filename().string();
        std::replace(name.begin(), name.end(), '.', '_');
        std::replace(name.begin(), name.end(), '-', '_');
        return name;
    });
