#pragma once

#include "../../file/shared/ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {
ast::FieldDefinition<ast::InputFieldSpec> parseInputFieldDefinition(
    const file::shared::ast::InputFieldDefinitionNode &node,
    const TypeRegistry &registry);
};
