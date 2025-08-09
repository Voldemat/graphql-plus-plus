#pragma once

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "libgql/parsers/schema/schema.hpp"

namespace json::serializers::schema {
void writeServerSchema(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                      const ::parsers::schema::ServerSchema &schema);
void writeClientSchema(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                      const ::parsers::schema::ClientSchema &schema);
};
