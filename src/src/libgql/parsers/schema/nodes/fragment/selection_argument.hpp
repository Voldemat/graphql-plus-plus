#pragma once

#include "../../../file/client/ast.hpp"
#include "../../client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"

namespace parsers::schema::nodes {

ast::FieldSelectionArgument parseSelectionArgument(
    const file::client::ast::Argument &node,
    const ast::CallableFieldSpec &spec);
};  // namespace parsers::schema::nodes
