#pragma once

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/client/ast.hpp"

namespace gql::json::serializers::file::client {
void writeNodes(
    JSONWriter &writer,
    const std::vector<::gql::parsers::file::client::ast::ASTNode> &nodes);
};
