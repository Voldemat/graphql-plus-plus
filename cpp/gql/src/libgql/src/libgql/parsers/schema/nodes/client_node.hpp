#pragma once

#include <vector>

#include "../type_registry.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"

namespace gql::parsers::schema::nodes {
ast::ClientSchemaNode parseClientDefinition(
    const file::client::ast::ASTNode &definition,
    const TypeRegistry &registry);
std::vector<ast::ClientSchemaNode> parseClientNodes(
    const std::vector<file::client::ast::ASTNode> &definitions,
    const TypeRegistry &registry);
};  // namespace parsers::schema::nodes
