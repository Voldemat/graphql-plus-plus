#pragma once

#include <memory>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {
std::shared_ptr<ast::ObjectType> parseObject(
    const file::server::ast::ObjectDefinitionNode &node,
    const TypeRegistry &registry);
};
