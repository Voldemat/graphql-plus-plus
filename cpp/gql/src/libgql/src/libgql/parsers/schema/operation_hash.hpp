#pragma once

#include <cstddef>
#include <memory>

#include "./client_ast.hpp"
#include "./type_registry.hpp"

namespace gql::parsers::schema {
std::size_t getClientOperationHash(
    const TypeRegistry &registry,
    const std::shared_ptr<const ast::Operation> &operation);
};
