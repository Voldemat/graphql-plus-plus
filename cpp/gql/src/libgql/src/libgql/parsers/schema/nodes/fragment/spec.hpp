#pragma once

#include "../../../file/client/ast.hpp"
#include "../../client_ast.hpp"
#include "../../server_ast.hpp"
#include "../../type_registry.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"

namespace gql::parsers::schema::nodes {

ast::FragmentSpec fragmentSpecFromFieldDefinition(
    const ast::FieldDefinition<ast::ObjectFieldSpec> &field,
    const file::client::ast::FragmentSpec &sNode);

ast::FragmentSpec parseFragmentSpec(
    const file::client::ast::FragmentSpec &defSpec,
    const ast::FragmentSpec &spec, const TypeRegistry &registry);
};  // namespace gql::parsers::schema::nodes
