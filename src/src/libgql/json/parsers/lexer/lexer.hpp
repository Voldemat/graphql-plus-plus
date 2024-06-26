#pragma once

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <expected>
#include <string>
#include <vector>

#include "../../utils.hpp"
#include "libgql/lexer/token.hpp"

namespace json::parsers::lexer {

std::vector<GQLToken> parseTokensArray(const JSONArray &document);
Location parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &document,
    const std::string &path);

};  // namespace json::parsers::lexer
