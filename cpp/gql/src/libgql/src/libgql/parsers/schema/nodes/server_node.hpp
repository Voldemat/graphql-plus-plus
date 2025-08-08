#pragma once

#include <vector>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace parsers::schema::nodes {

ast::ServerSchemaNode parseServerNodeFirstPass(
    const file::server::ast::TypeDefinitionNode &astNode);
ast::ServerSchemaNode parseServerNodeSecondPass(
    const file::server::ast::TypeDefinitionNode &astNode,
    const TypeRegistry &registry);

std::vector<ast::ServerSchemaNode> parseServerNodesSecondPass(
    const std::vector<file::server::ast::ASTNode> &astArray,
    const TypeRegistry &registry);
};  // namespace parsers::schema::nodes
