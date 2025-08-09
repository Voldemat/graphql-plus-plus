#pragma once

#include <memory>
#include <vector>

#include "../../../file/client/ast.hpp"
#include "../../client_ast.hpp"
#include "../../server_ast.hpp"
#include "../../type_registry.hpp"

namespace gql::parsers::schema::nodes {
ast::UnionFragmentSpec parseUnionFragmentSpec(
    const std::vector<file::client::ast::SelectionNode> &selections,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry);
};
