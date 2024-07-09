#include "./lexer.hpp"

#include <rapidjson/allocators.h>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <cassert>
#include <expected>
#include <format>
#include <string>
#include <vector>

#include "../putils.hpp"
#include "../shared.hpp"
#include "libgql/json/utils.hpp"
#include "libgql/lexer/location.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace lexer;
using namespace json::parsers;
using namespace json::parsers::lexer;
using namespace json::parsers::utils;
using namespace json::parsers::shared;

std::vector<GQLToken> json::parsers::lexer::parseTokensArray(
    const JSONArray &document) {
    unsigned int index = 0;
    std::vector<GQLToken> tokens;
    for (const auto &item : document) {
        if (!item.IsObject()) {
            throw ParsingError(
                std::format("Element {} is not an object", index),
                ParsingErrorType::INVALID_TYPE);
        };
        if (!item.HasMember("type")) {
            throw ParsingError(
                std::format("Element {} doesn`t have \"type\" key", index),
                ParsingErrorType::NO_MEMBER);
        };
        auto maybeType = gqlTokenTypeFromString(item["type"].GetString());
        if (!maybeType.has_value()) {
            throw ParsingError(std::format("Element {}.type is invalid", index),
                               ParsingErrorType::INVALID_MEMBER);
        };
        const GQLTokenType type = maybeType.value();
        if (!item.HasMember("lexeme")) {
            throw ParsingError(
                std::format("Element {} doesn`t have \"lexeme\" key", index),
                ParsingErrorType::NO_MEMBER);
        };
        const std::string lexeme = item["lexeme"].GetString();
        if (!item.HasMember("location")) {
            throw ParsingError(
                std::format("Element {} doesn`t have \"location\" key", index),
                ParsingErrorType::NO_MEMBER);
        };
        const auto &location =
            parseLocation(item["location"], std::format("{}.location", index));
        index++;
        tokens.push_back(
            { .type = type, .lexeme = lexeme, .location = location });
    };
    return tokens;
};

Location json::parsers::lexer::parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &value,
    const std::string &path) {
    return (Location){ .line = extractUint(value, "line", path),
                       .start = extractUint(value, "start", path),
                       .end = extractUint(value, "end", path) };
};
