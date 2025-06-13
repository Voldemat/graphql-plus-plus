#pragma once

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

namespace parsers::schema::nodes {

std::map<std::string, ast::FieldSelectionArgument> parseArguments(
    const std::vector<file::shared::ast::Argument> &arguments,
    const std::shared_ptr<ast::ServerDirective>& directive,
    const TypeRegistry &registry);
};
