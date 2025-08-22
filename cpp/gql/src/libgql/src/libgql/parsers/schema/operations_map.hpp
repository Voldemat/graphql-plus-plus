#pragma once

#include <cstddef>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "./client_ast.hpp"
#include "./schema.hpp"
#include "./shared_ast.hpp"

namespace gql::parsers::schema::operations_map {
using OperationImplementationMap = std::map<std::size_t, ast::FragmentSpec>;

struct OperationSignature {
    std::map<std::string, ast::FieldDefinition<ast::InputFieldSpec>> parameters;
    OperationImplementationMap implementationMap;
};
using OperationSignatureMap = std::map<std::size_t, OperationSignature>;

using OperationsMap = std::map<std::string, OperationSignatureMap>;
using FragmentMap = std::map<std::size_t, std::shared_ptr<ast::Fragment>>;

struct OperationsMapContainer {
    OperationsMap queries;
    OperationsMap mutations;
    OperationsMap subscriptions;
    std::map<std::string, FragmentMap> fragments;

    void addFragments(const std::vector<std::shared_ptr<ast::Fragment>>&);

    OperationsMapContainer static from(
        const std::vector<ClientSchema> &schemas);
};
};  // namespace gql::parsers::schema::operations_map
