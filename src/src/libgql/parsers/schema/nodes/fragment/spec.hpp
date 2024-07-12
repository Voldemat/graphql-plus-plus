#pragma once

#include "../../../file/client/ast.hpp"
#include "../../client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace parsers::schema::nodes {

ast::FragmentSpec fragmentSpecFromFieldDefinition(
    const ast::FieldDefinition<ast::ObjectFieldSpec> &field,
    const file::client::ast::FragmentSpec &sNode);

ast::FragmentSpec parseFragmentSpec(
    const file::client::ast::FragmentSpec &defSpec,
    const ast::FragmentSpec &spec, const TypeRegistry &registry);
};  // namespace parsers::schema::nodes
