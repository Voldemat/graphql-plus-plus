#ifndef GRAPHQL_JSON_SERIALIZER
#define GRAPHQL_JSON_SERIALIZER

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/reader.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/schema/schema.hpp"

namespace json {
namespace serializer {
void writeTokenAsJSON(rapidjson::PrettyWriter<rapidjson::StringBuffer> &writer,
                      const GQLToken &token);

class ASTJSONWriter {
    rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
        *writer;

public:
    ASTJSONWriter(
        rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
            *writer);
};

void writeSchemaNodes(rapidjson::PrettyWriter<rapidjson::StringBuffer> &writer,
                      const std::vector<parsers::schema::SchemaNode> &nodes);
};  // namespace serializer
};  // namespace json
#endif
