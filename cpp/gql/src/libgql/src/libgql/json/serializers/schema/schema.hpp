#pragma once

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <optional>

#include "libgql/parsers/schema/schema.hpp"

namespace gql::json::serializers::schema {
void writeServerSchema(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const ::gql::parsers::schema::ServerSchema &schema,
    const std::optional<::gql::parsers::schema::ClientSchema> &clientSchema);
void writeClientSchema(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                       const ::gql::parsers::schema::ClientSchema &schema);
};  // namespace gql::json::serializers::schema
