#include "./lexer.hpp"

#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace lexer;

void json::serializers::lexer::writeTokens(
    JSONWriter &writer, const std::vector<GQLToken> &tokens) {
    writer.StartArray();
    for (const auto &token : tokens) {
        writeToken(writer, token);
    };
    writer.EndArray();
};

void json::serializers::lexer::writeToken(JSONWriter &writer,
                                          const GQLToken &token) {
    writer.StartObject();
    writer.String("type");
    writer.String(gqlTokenTypeToString(token.type).c_str());
    writer.String("lexeme");
    writer.String(token.lexeme.c_str());
    writer.String("location");
    {
        writer.StartObject();
        writer.String("line");
        writer.Uint(token.location.line);
        writer.String("start");
        writer.Uint(token.location.start);
        writer.String("end");
        writer.Uint(token.location.end);
        writer.EndObject();
    };
    writer.EndObject();
};
