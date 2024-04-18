
#include "json/serializer.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "lexer/token.hpp"


void json::serializer::writeTokenAsJSON(
    rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
        &writer,
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
        writer.Int(token.location.line);
        writer.String("start");
        writer.Int(token.location.start);
        writer.String("end");
        writer.Int(token.location.end);
        writer.EndObject();
    };
    writer.EndObject();
};
