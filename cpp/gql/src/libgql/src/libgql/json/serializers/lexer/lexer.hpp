#pragma once

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/lexer/token.hpp"

namespace gql::json::serializers::lexer {

void writeTokens(JSONWriter &writer,
                 const std::vector<::gql::lexer::GQLToken> &tokens);
void writeToken(JSONWriter &writer, const ::gql::lexer::GQLToken &token);
};  // namespace gql::json::serializers::lexer
