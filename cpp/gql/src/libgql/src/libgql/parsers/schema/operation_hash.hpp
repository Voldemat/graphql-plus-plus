#pragma once

#include <cstddef>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "./shared_ast.hpp"
#include "./type_registry.hpp"
#include "libgql/parsers/schema/client_ast.hpp"

namespace gql::parsers::schema {
std::size_t getOperationParametersHash(
    const TypeRegistry &registry,
    const std::map<std::string, ast::FieldDefinition<ast::InputFieldSpec>>
        &parameters);
std::size_t getFragmentSpecHash(const TypeRegistry &registry,
                                const ast::FragmentSpec &fragmentSpec,
                                const bool &recursive);
std::vector<std::shared_ptr<ast::Fragment>> getUsedFragmentsFromFragmentSpec(
    const TypeRegistry &registry, const ast::FragmentSpec &fragmentSpec);

};  // namespace gql::parsers::schema
