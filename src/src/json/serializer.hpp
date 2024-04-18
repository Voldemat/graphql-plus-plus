#ifndef GRAPHQL_JSON_SERIALIZER
#define GRAPHQL_JSON_SERIALIZER

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>
#include <rapidjson/reader.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "lexer/token.hpp"

namespace json {
namespace serializer {
void writeTokenAsJSON(
    rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
        &writer,
    const GQLToken &token);
};  // namespace serializer
};  // namespace json
#endif
