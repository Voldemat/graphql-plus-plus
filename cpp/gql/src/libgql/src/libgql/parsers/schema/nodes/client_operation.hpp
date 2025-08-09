#pragma once

#include <memory>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace gql::parsers::schema::nodes {
std::shared_ptr<ast::Operation> parseClientOperationDefinition(
    const file::client::ast::OperationDefinition &definition,
    const TypeRegistry &registry);
};
