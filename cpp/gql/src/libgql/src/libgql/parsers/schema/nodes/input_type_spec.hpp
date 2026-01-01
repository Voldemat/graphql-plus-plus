#pragma once

#include <optional>
#include <utility>

#include "../../file/shared/ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {
std::pair<ast::NonCallableFieldSpec<ast::InputTypeSpec>, bool>
parseNonCallableInputTypeSpec(
    const file::shared::ast::TypeNode astNode,
    const std::optional<file::shared::ast::LiteralNode> defaultValueNode,
    const TypeRegistry &registry);

std::pair<ast::InputFieldSpec, bool> parseInputTypeSpec(
    const file::shared::ast::InputFieldDefinitionNode &astNode,
    const TypeRegistry &registry);
};  // namespace gql::parsers::schema::nodes
