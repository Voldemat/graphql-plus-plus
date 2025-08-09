#pragma once

#include "../../file/shared/ast.hpp"
#include "../shared_ast.hpp"

namespace gql::parsers::schema::nodes {
ast::Literal parseLiteralNode(const file::shared::ast::LiteralNode &literal,
                              const ast::InputTypeSpec &spec);
};
