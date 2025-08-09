#pragma once

#include <memory>

#include "../client_ast.hpp"
#include "../type_registry.hpp"
#include "libgql/parsers/file/client/ast.hpp"

namespace gql::parsers::schema::nodes {
std::shared_ptr<ast::ClientDirective> parseClientDirective(
    const file::client::ast::DirectiveDefinition &astNode,
    const TypeRegistry &registry);

}  // namespace parsers::schema::nodes
