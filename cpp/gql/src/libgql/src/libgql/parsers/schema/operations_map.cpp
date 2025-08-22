#include "./operations_map.hpp"

#include <memory>
#include <ranges>
#include <vector>

#include "./schema.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"

namespace gql::parsers::schema::operations_map {

OperationsMap &getOperationsMapByType(
    OperationsMapContainer &container,
    const gql::parsers::file::client::ast::OpType &type) {
    switch (type) {
        case gql::parsers::file::client::ast::OpType::QUERY:
            return container.queries;
        case gql::parsers::file::client::ast::OpType::MUTATION:
            return container.mutations;
        case gql::parsers::file::client::ast::OpType::SUBSCRIPTION:
            return container.subscriptions;
    };
};

void OperationsMapContainer::addFragments(
    const std::vector<std::shared_ptr<ast::Fragment>> &array) {
    for (const auto &fragment : array) {
        if (!fragments.contains(fragment->name)) {
            fragments[fragment->name] = {};
        };
        auto &fragmentMap = fragments[fragment->name];
        fragmentMap[fragment->hash] = fragment;
    };
};

OperationsMapContainer OperationsMapContainer::from(
    const std::vector<ClientSchema> &schemas) {
    OperationsMapContainer container;
    for (const auto &schema : schemas) {
        for (const auto &operation : schema.operations | std::views::values) {
            auto &operationsMap =
                getOperationsMapByType(container, operation->type);
            if (!operationsMap.contains(operation->name)) {
                operationsMap[operation->name] = {};
            };
            auto &signaturesMap = operationsMap[operation->name];
            if (!signaturesMap.contains(operation->parametersHash)) {
                signaturesMap[operation->parametersHash] = {
                    .parameters = operation->parameters, .implementationMap = {}
                };
            };
            auto &implementationMap =
                signaturesMap[operation->parametersHash].implementationMap;
            if (implementationMap.contains(operation->fragmentSpecHash))
                continue;
            implementationMap[operation->fragmentSpecHash] =
                operation->fragmentSpec;
            container.addFragments(operation->usedFragments);
        };
    };
    return container;
};
};  // namespace gql::parsers::schema::operations_map
