#include "./serializer.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "libgql/lexer/token.hpp"

using namespace rapidjson;
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;
void json::serializer::writeTokenAsJSON(
    PrettyWriter<StringBuffer> &writer, const GQLToken &token) {
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

json::serializer::ASTJSONWriter::ASTJSONWriter(
    Writer<GenericStringBuffer<UTF8<>>> *writer)
    : writer{ writer } {};
