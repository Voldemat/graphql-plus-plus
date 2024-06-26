#pragma once

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "libgql/parsers/schema/schema.hpp"

namespace json::serializers::schema {
void writeSchemaNodes(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                      const ::parsers::schema::Schema &schema);
};
