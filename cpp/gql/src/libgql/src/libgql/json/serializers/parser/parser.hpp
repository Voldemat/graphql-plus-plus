#pragma once

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <vector>

#include "libgql/parsers/file/server/ast.hpp"

namespace json::serializers::parser {
void writeServerNodes(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const std::vector<::parsers::file::server::ast::ASTNode> &nodes);
};
