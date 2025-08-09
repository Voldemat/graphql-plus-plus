#include "./lexer.hpp"

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace gql::lexer;

namespace gql::json::serializers::lexer {
void writeTokens(JSONWriter &writer, const std::vector<GQLToken> &tokens) {
    writer.StartArray();
    for (const auto &token : tokens) {
        writeToken(writer, token);
    };
    writer.EndArray();
};

void writeToken(JSONWriter &writer, const GQLToken &token) {
    writer.StartObject();
    writer.String("type");
    writer.String(gqlTokenTypeToString(token.type).c_str());
    writer.String("lexeme");
    writer.String(token.lexeme.c_str());
    writer.String("location");
    {
        writer.StartObject();
        writer.String("line");
        writer.Uint(token.location.getLine());
        writer.String("start");
        writer.Uint(token.location.getStart());
        writer.String("end");
        writer.Uint(token.location.getEnd());
        writer.EndObject();
    };
    writer.EndObject();
};
};  // namespace gql::json::serializers::lexer
