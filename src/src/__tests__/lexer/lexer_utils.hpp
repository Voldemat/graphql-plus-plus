#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>

#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <optional>
#include <ostream>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/token.hpp"

using namespace lexer;
struct TestCase {
    std::shared_ptr<SourceFile> sourceFile;
    std::string filename;
    std::string schema;
    std::vector<GQLToken> expectedTokens;
    std::optional<LexerError> error;
};
inline std::ostream &operator<<(std::ostream &os, const TestCase &self) {
    os << "TestCase(filename: " << self.filename << ", schema: ";
    os << self.schema << ", tokens: " << self.expectedTokens.size();
    os << ")";
    return os;
};
const std::filesystem::path casesPath
    = std::filesystem::path(__FILE__).parent_path().append("cases");
const std::regex casesRegex = std::regex(".*\\.case\\.json");
inline std::vector<TestCase> getCases() noexcept {
    std::vector<TestCase> cases;
    for (const auto &filepath :
         std::filesystem::directory_iterator(casesPath)
             | std::ranges::views::transform(
                 [](std::filesystem::directory_entry entry) {
                     return entry.path();
                 })
             | std::ranges::views::filter([](std::filesystem::path path) {
                   return std::regex_match(path.filename().string(), casesRegex);
               })) {
        std::ifstream file(filepath);
        rapidjson::IStreamWrapper isw(file);
        rapidjson::Document d;
        d.ParseStream(isw);
        assert(d.IsObject());
        std::string schema = d["schema"].GetString();
        std::vector<GQLToken> expectedTokens;
        std::optional<LexerError> error;
        std::shared_ptr<SourceFile> sourceFile
            = std::make_shared<SourceFile>(filepath);
        if (d.HasMember("tokens")) {
            for (const auto &jsonToken : d["tokens"].GetArray()) {
                assert(jsonToken.IsObject());
                const auto &location = jsonToken["location"];
                expectedTokens.push_back(
                    { .type
                      = gqlTokenTypeFromString(jsonToken["type"].GetString())
                            .value(),
                      .lexeme = jsonToken["lexeme"].GetString(),
                      .location = { .source = sourceFile, .line = location["line"].GetUint(), .start=location["start"].GetUint(), .end=location["end"].GetUint() } });
            };
        } else {
            const auto &errorObj = d["error"];
            const auto &location = errorObj["location"];
            error = LexerError(
                errorObj["message"].GetString(),
                (Location){ .source = sourceFile,
                            .line = location["line"].GetUint(),
                            .start = location["start"].GetUint(),
                            .end = location["end"].GetUint() });
        };
        cases.push_back({ .sourceFile = sourceFile,
                          .filename = filepath.filename(),
                          .schema = schema,
                          .expectedTokens = expectedTokens,
                          .error = error });
    };
    return cases;
};
