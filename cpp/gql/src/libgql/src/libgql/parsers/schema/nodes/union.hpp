#pragma once

#include <memory>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {

std::shared_ptr<ast::Union> parseUnion(
    const file::server::ast::UnionDefinitionNode &node,
    const TypeRegistry &registry);
};
