#pragma once

#include <memory>

#include "../../file/server/ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"

namespace parsers::schema::nodes {
std::shared_ptr<ast::ServerDirective> parseServerDirective(
    const file::server::ast::DirectiveDefinitionNode &astNode,
    const TypeRegistry &registry);

}  // namespace parsers::schema::nodes
