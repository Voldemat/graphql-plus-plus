#pragma once

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/server/ast.hpp"

namespace gql::json::serializers::file::server {
void writeNodes(
    JSONWriter &writer,
    const std::vector<::gql::parsers::file::server::ast::ASTNode> &nodes);
};
