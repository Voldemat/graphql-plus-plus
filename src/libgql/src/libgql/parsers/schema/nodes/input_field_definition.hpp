#pragma once

#include "../../file/shared/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"

namespace parsers::schema::nodes {
ast::FieldDefinition<ast::InputFieldSpec> parseInputFieldDefinition(
    const file::shared::ast::InputValueDefinitionNode &node,
    const TypeRegistry &registry);
};
