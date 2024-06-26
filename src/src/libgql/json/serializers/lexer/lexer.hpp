#pragma once

#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/lexer/token.hpp"
namespace json::serializers::lexer {

void writeTokens(
    JSONWriter &writer,
    const std::vector<GQLToken> &tokens);
void writeToken(JSONWriter &writer,
                const GQLToken &token);
};  // namespace json::serializers::lexer
