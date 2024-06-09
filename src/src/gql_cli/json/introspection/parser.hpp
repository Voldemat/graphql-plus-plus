#pragma once

#include <rapidjson/document.h>

#include "libgql/parsers/schema/schema.hpp"
namespace json {
namespace parser {
namespace introspection {

const parsers::schema::ServerSchema parseIntrospectionSchema(
    const rapidjson::Document &document);
};
};  // namespace parser
};  // namespace json
