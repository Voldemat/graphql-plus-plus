#pragma once

#include <memory>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace parsers::schema::nodes {

std::shared_ptr<ast::Interface> parseInterface(
    const file::server::ast::InterfaceDefinitionNode &node,
    const TypeRegistry &registry);
std::shared_ptr<ast::InputType> parseInput(
    const file::server::ast::InputObjectDefinitionNode &node,
    const TypeRegistry &registry);
};  // namespace parsers::schema::nodes
