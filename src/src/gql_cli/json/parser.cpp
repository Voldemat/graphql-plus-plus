#include "./parser.hpp"

#include <__expected/expected.h>
#include <__expected/unexpected.h>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <format>
#include <memory>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "libgql/lexer/token.hpp"

using namespace json::parser;
std::expected<std::vector<GQLToken>, std::string>
json::parser::parseTokensArray(
    rapidjson::Document const &document,
    std::shared_ptr<SourceFile> sourceFile) noexcept {
    if (!document.IsArray())
        return std::unexpected("Json root is not an array");
    unsigned int index = 0;
    std::vector<GQLToken> tokens;
    for (const auto &item : document.GetArray()) {
        if (!item.IsObject()) {
            return std::unexpected(
                std::format("Element {} is not an object", index));
        };
        if (!item.HasMember("type")) {
            return std::unexpected(
                std::format("Element {} doesn`t have \"type\" key", index));
        };
        auto maybeType = gqlTokenTypeFromString(item["type"].GetString());
        if (!maybeType.has_value()) {
            return std::unexpected(
                std::format("Element {}.type is invalid", index));
        };
        const GQLTokenType type = maybeType.value();
        if (!item.HasMember("lexeme")) {
            return std::unexpected(
                std::format("Element {} doesn`t have \"lexeme\" key", index));
        };
        const std::string lexeme = item["lexeme"].GetString();
        if (!item.HasMember("location")) {
            return std::unexpected(
                std::format("Element {} doesn`t have \"location\" key", index));
        };
        const auto locationOrError
            = parseLocation(item["location"], sourceFile);
        if (!locationOrError.has_value()) {
            const auto &[key, eType] = locationOrError.error();
            if (eType == ParsingErrorType::INVALID_MEMBER) {
                return std::unexpected(std::format(
                    "Element {}.location.{} is invalid", index, key));
            };
            return std::unexpected(std::format(
                "Element {}.location doesn`t have \"{}\" key", index, key));
        };
        const Location location = locationOrError.value();
        index++;
        tokens.push_back(
            { .type = type, .lexeme = lexeme, .location = location });
    };
    return tokens;
};

#define ASSERT_HAS_MEMBER(key)                                 \
    if (!value.HasMember(key)) {                               \
        return std::unexpected(                                \
            std::make_pair(key, ParsingErrorType::NO_MEMBER)); \
    }

#define ASSERT_MEMBER_IS_VALID(key, isValid)\
    if (!isValid) {\
        return std::unexpected(                                     \
            std::make_pair(key, ParsingErrorType::INVALID_MEMBER)); \
    }
std::expected<Location, std::tuple<std::string, ParsingErrorType>>
json::parser::parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &value,
    std::shared_ptr<SourceFile> sourceFile) noexcept {
    ASSERT_HAS_MEMBER("line");
    ASSERT_MEMBER_IS_VALID("line", value["line"].IsUint());
    unsigned int line = value["line"].GetUint();
    ASSERT_HAS_MEMBER("start");
    ASSERT_MEMBER_IS_VALID("start", value["start"].IsUint());
    unsigned int start = value["start"].GetUint();
    ASSERT_HAS_MEMBER("end");
    ASSERT_MEMBER_IS_VALID("end", value["end"].IsUint());
    unsigned int end = value["end"].GetUint();
    return (Location){
        .source = sourceFile, .line = line, .start = start, .end = end
    };
};
