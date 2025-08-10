#pragma once

#include <rapidjson/document.h>

#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace gql::json::parsers::schema::client {
::gql::parsers::schema::ClientSchema parseSchema(
    ::gql::parsers::schema::TypeRegistry &registry,
    const rapidjson::Document &document);
};  // namespace gql::json::parsers::schema::client
