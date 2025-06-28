#pragma once

#include <memory>

#include "../server_ast.hpp"
#include "../type_registry.hpp"
#include "../../file/server/ast.hpp"

namespace parsers::schema::nodes {

std::shared_ptr<ast::Union> parseUnion(const file::server::ast::UnionDefinitionNode &node,
                                  const TypeRegistry &registry);
};
