#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>

#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <optional>
#include <ostream>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

#include "libgql/lexer/lexer_error.hpp"
#include "libgql/lexer/location.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace lexer;
struct LexerTestCase {
    std::string filename;
    std::string schema;
    std::vector<GQLToken> expectedTokens;
    std::optional<lexer::LexerError> error;
};
inline std::ostream &operator<<(std::ostream &os, const LexerTestCase &self) {
    os << "TestCase(filename: " << self.filename << ", schema: ";
    os << self.schema << ", tokens: " << self.expectedTokens.size();
    os << ")";
    return os;
};
const std::filesystem::path casesPath =
    std::filesystem::path(__FILE__).parent_path().append("cases");
const std::regex casesRegex = std::regex(".*\\.case\\.json");
inline std::vector<LexerTestCase> getLexerCases() noexcept {
    std::cout << casesPath << std::endl;
    std::vector<LexerTestCase> cases;
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
        std::string schema = d["schema"].GetString();
        std::vector<GQLToken> expectedTokens;
        std::optional<LexerError> error;
        if (d.HasMember("tokens")) {
            for (const auto &jsonToken : d["tokens"].GetArray()) {
                assert(jsonToken.IsObject());
                const auto &location = jsonToken["location"];
                expectedTokens.push_back(
                    { .type =
                          gqlTokenTypeFromString(jsonToken["type"].GetString())
                              .value(),
                      .lexeme = jsonToken["lexeme"].GetString(),
                      .location = Location(location["line"].GetUint(),
                                           location["start"].GetUint(),
                                           location["end"].GetUint()) });
            };
        } else {
            const auto &errorObj = d["error"];
            const auto &location = errorObj["location"];
            error = LexerError(errorObj["message"].GetString(),
                               Location(location["line"].GetUint(),
                                        location["start"].GetUint(),
                                        location["end"].GetUint()));
        };
        cases.push_back({ .filename = filepath.filename(),
                          .schema = schema,
                          .expectedTokens = expectedTokens,
                          .error = error });
    };
    return cases;
};
