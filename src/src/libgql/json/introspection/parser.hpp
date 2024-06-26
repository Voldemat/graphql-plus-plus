#pragma once

#include <rapidjson/document.h>

#include "../utils.hpp"
#include "libgql/parsers/schema/schema.hpp"

namespace json {
namespace parser {
namespace introspection {

const parsers::schema::ServerSchema parseIntrospectionSchema(
    const JSONObject &document);
};
};  // namespace parser
};  // namespace json
