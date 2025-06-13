#pragma once

#include "libgql/parsers/file/shared/ast.hpp"
#include "../../server_ast.hpp"
#include "../../shared_ast.hpp"

namespace parsers::schema::nodes {

ast::FieldSelectionArgument parseSelectionArgument(
    const file::shared::ast::Argument &node,
    const ast::CallableFieldSpec &spec);
};  // namespace parsers::schema::nodes
