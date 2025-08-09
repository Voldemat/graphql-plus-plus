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

using namespace gql::lexer;

namespace gql::json::parsers::lexer {
std::vector<GQLToken> parseTokensArray(const JSONArray &document) {
    unsigned int index = 0;
    std::vector<GQLToken> tokens;
    for (const auto &item : document) {
        if (!item.IsObject()) {
            throw shared::ParsingError(
                std::format("Element {} is not an object", index),
                shared::ParsingErrorType::INVALID_TYPE);
        };
        if (!item.HasMember("type")) {
            throw shared::ParsingError(
                std::format("Element {} doesn`t have \"type\" key", index),
                shared::ParsingErrorType::NO_MEMBER);
        };
        auto maybeType = gqlTokenTypeFromString(item["type"].GetString());
        if (!maybeType.has_value()) {
            throw shared::ParsingError(
                std::format("Element {}.type is invalid", index),
                shared::ParsingErrorType::INVALID_MEMBER);
        };
        const GQLTokenType type = maybeType.value();
        if (!item.HasMember("lexeme")) {
            throw shared::ParsingError(
                std::format("Element {} doesn`t have \"lexeme\" key", index),
                shared::ParsingErrorType::NO_MEMBER);
        };
        const std::string lexeme = item["lexeme"].GetString();
        if (!item.HasMember("location")) {
            throw shared::ParsingError(
                std::format("Element {} doesn`t have \"location\" key", index),
                shared::ParsingErrorType::NO_MEMBER);
        };
        const auto &location =
            parseLocation(item["location"], std::format("{}.location", index));
        index++;
        tokens.push_back(
            { .type = type, .lexeme = lexeme, .location = location });
    };
    return tokens;
};

Location parseLocation(rapidjson::GenericValue<rapidjson::UTF8<>> const &value,
                       const std::string &path) {
    return Location(utils::extractUint(value, "line", path),
                             utils::extractUint(value, "start", path),
                             utils::extractUint(value, "end", path));
};
};  // namespace gql::json::parsers::lexer
