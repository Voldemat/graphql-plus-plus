#pragma once

#include <rapidjson/document.h>

#include "libgql/parsers/schema/schema.hpp"

namespace gql::json::introspection {
const parsers::schema::ServerSchema parseIntrospectionSchema(
    const rapidjson::Document &document);
};  // namespace gql::json::introspection
