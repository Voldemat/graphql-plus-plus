#pragma once

#include <memory>
#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {

std::shared_ptr<ast::Interface> parseInterface(
    const file::server::ast::InterfaceDefinitionNode &node,
    const TypeRegistry &registry);
};
