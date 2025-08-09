#pragma once

#include <utility>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {
std::pair<ast::ObjectFieldSpec, bool> parseObjectFieldSpec(
    const file::server::ast::FieldDefinitionNode &astNode,
    const TypeRegistry &registry);

ast::ObjectTypeSpec getReturnTypeFromNonCallableFieldSpec(
    const ast::NonCallableFieldSpec<ast::ObjectTypeSpec> &fSpec);

ast::ObjectTypeSpec getReturnTypeFromObjectFieldSpec(
    const ast::ObjectFieldSpec &spec);
}  // namespace gql::parsers::schema::nodes
