#pragma once

#include <vector>

#include "../shared_ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace gql::parsers::schema::nodes {
std::vector<ast::ServerDirectiveInvocation> parseServerDirectiveInvocations(
    const std::vector<file::shared::ast::DirectiveInvocationNode>& invocations,
    const TypeRegistry& registry
);
};
