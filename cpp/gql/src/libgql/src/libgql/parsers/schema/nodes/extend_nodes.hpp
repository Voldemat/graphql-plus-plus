#pragma once

#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace gql::parsers::schema::nodes {

std::vector<std::pair<
    std::shared_ptr<ast::ObjectType>,
    std::map<std::string,
             std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>>>
parseServerExtendNodes(
    const std::vector<file::server::ast::ASTNode> &astArray,
    const TypeRegistry &registry);
};
