#ifndef GRAPHQL_JSON_PARSER
#define GRAPHQL_JSON_PARSER

#include <expected>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <string>
#include <tuple>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/schema/schema.hpp"
namespace json {
namespace parser {
enum class ParsingErrorType {
    NO_MEMBER,
    INVALID_MEMBER,
};
std::expected<std::vector<GQLToken>, std::string> parseTokensArray(
    rapidjson::Document const &document) noexcept;
std::expected<Location, std::tuple<std::string, ParsingErrorType>>
parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &document) noexcept;

const parsers::schema::Schema parseSchema(const rapidjson::Document& document);
};  // namespace parser
};  // namespace json
#endif
