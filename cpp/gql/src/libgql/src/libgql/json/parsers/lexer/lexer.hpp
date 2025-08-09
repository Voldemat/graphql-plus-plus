#pragma once

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <expected>
#include <string>
#include <vector>

#include "../../utils.hpp"
#include "libgql/lexer/location.hpp"
#include "libgql/lexer/token.hpp"

namespace gql::json::parsers::lexer {

std::vector<::gql::lexer::GQLToken> parseTokensArray(const JSONArray &document);
::gql::lexer::Location parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &document,
    const std::string &path);

};  // namespace gql::json::parsers::lexer
