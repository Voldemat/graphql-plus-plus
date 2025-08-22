#pragma once

#include <memory>

#include "../../../file/client/ast.hpp"
#include "../../../file/shared/ast.hpp"
#include "../../client_ast.hpp"
#include "../../type_registry.hpp"

namespace gql::parsers::schema::nodes {

ast::FragmentSpec fragmentSpecFromName(
    const file::shared::ast::NameNode &typeName, const TypeRegistry &registry);

std::shared_ptr<ast::Fragment> parseFragmentSecondPass(
    const file::client::ast::FragmentDefinition &definition,
    const TypeRegistry &registry);
};  // namespace parsers::schema::nodes
