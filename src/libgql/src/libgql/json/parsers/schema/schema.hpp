#pragma once


#include <rapidjson/document.h>
#include "libgql/parsers/schema/schema.hpp"

namespace json::parsers::schema {
const ::parsers::schema::Schema parseSchema(const rapidjson::Document &document);
};
