#pragma once

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>
#include "libgql/parsers/file/server/ast.hpp"

namespace json::serializers::parser {
void writeFileNodes(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                      const ::parsers::file::server::ast::FileNodes &nodes);
};
