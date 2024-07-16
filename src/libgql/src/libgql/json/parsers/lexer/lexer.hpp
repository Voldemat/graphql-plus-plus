#pragma once

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <expected>
#include <string>
#include <vector>

#include "../../utils.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/location.hpp"

namespace json::parsers::lexer {

std::vector<::lexer::GQLToken> parseTokensArray(const JSONArray &document);
::lexer::Location parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &document,
    const std::string &path);

};  // namespace json::parsers::lexer
