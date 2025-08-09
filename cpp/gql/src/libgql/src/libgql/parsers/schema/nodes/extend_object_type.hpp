#pragma once

#include <map>
#include <memory>
#include <string>
#include <utility>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"

namespace gql::parsers::schema::nodes {
std::pair<std::shared_ptr<ast::ObjectType>,
          std::map<std::string,
                   std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>>
parseExtendObjectType(const file::server::ast::ExtendTypeNode &node,
                      const TypeRegistry &registry);
};
