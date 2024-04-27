#ifndef GRAPHQL_JSON_PARSER
#define GRAPHQL_JSON_PARSER

#include <__expected/expected.h>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <memory>
#include <string>
#include <tuple>
#include <vector>

#include "lexer/token.hpp"
namespace json {
namespace parser {
enum class ParsingErrorType {
    NO_MEMBER,
    INVALID_MEMBER,
};
std::expected<std::vector<GQLToken>, std::string> parseTokensArray(
    rapidjson::Document const &document,
    std::shared_ptr<SourceFile> sourceFile) noexcept;
std::expected<Location, std::tuple<std::string, ParsingErrorType>> parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &document,
    std::shared_ptr<SourceFile> sourceFile) noexcept;
};  // namespace parser
};  // namespace json
#endif
