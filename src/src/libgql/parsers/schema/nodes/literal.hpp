#pragma once

#include "../../file/shared/ast.hpp"
#include "../server_ast.hpp"

namespace parsers::schema::nodes {
ast::Literal parseLiteralNode(const file::shared::ast::LiteralNode &literal,
                              const ast::InputTypeSpec &spec);
};
