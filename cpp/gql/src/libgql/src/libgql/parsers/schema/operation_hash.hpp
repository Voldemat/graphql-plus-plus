#pragma once

#include <cstddef>
#include <map>
#include <string>

#include "./shared_ast.hpp"
#include "./type_registry.hpp"
#include "libgql/parsers/schema/client_ast.hpp"

namespace gql::parsers::schema {
std::size_t getOperationParametersHash(
    const TypeRegistry &registry,
    const std::map<std::string, ast::FieldDefinition<ast::InputFieldSpec>>
        &parameters);
std::size_t getOperationFragmentSpecHash(const TypeRegistry &registry,
                                         const ast::FragmentSpec &fragmentSpec);
};  // namespace gql::parsers::schema
