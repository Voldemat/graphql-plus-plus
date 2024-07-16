#pragma once

#include <memory>
#include <vector>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace parsers::schema::nodes {
ast::UnionFragmentSpec parseUnionFragmentSpec(
    const std::vector<file::client::ast::SelectionNode> &selections,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry);
};
