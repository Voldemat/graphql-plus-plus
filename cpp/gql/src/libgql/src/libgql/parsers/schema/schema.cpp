#include "./schema.hpp"

#include <memory>
#include <ranges>
#include <variant>
#include <vector>

#include "../file/client/ast.hpp"
#include "../file/server/ast.hpp"
#include "./client_ast.hpp"
#include "./nodes/client_node.hpp"
#include "./nodes/extend_nodes.hpp"
#include "./nodes/fragment/fragment.hpp"
#include "./nodes/server_node.hpp"
#include "./type_registry.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/operation_hash.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema {
auto filterFragmentDefinitions(
    const std::vector<client::ast::ASTNode> &definitions) {
    return definitions | std::views::filter([](const auto &def) {
               return std::holds_alternative<client::ast::FragmentDefinition>(
                   def);
           }) |
           std::views::transform([](const auto &def) {
               return std::get<client::ast::FragmentDefinition>(def);
           });
};

const ServerSchema parseServerSchema(
    TypeRegistry &registry, const std::vector<server::ast::ASTNode> &astNodes) {
    for (const auto &node :
         astNodes | std::views::filter([](const auto &node) {
             return std::holds_alternative<server::ast::TypeDefinitionNode>(
                 node);
         }) | std::views::transform([](const auto &node) {
             return std::get<server::ast::TypeDefinitionNode>(node);
         })) {
        registry.addNode(nodes::parseServerNodeFirstPass(node));
    };
    const auto &serverNodes =
        nodes::parseServerNodesSecondPass(astNodes, registry);
    for (const auto &[typeNode, newFields] :
         nodes::parseServerExtendNodes(astNodes, registry)) {
        registry.patchObject(typeNode, newFields);
    };
    return ServerSchema::fromNodes(serverNodes);
}

const ClientSchema parseClientSchema(
    parsers::schema::TypeRegistry &registry,
    const std::vector<file::client::ast::ASTNode> &astNodes) {
    for (const auto &astNode : astNodes) {
        std::visit(
            utils::overloaded{
                [&registry](
                    const file::client::ast::OperationDefinition &operation) {
                    if (registry.operations.contains(operation.name.name)) {
                        throw shared::ParserError(
                            operation.name.location.startToken,
                            "Operation with this name already exists",
                            operation.name.location.source);
                    };
                    registry.operations[operation.name.name] =
                        std::make_shared<ast::Operation>(operation.type,
                                                         operation.name.name);
                },
                [&registry](
                    const file::client::ast::FragmentDefinition &fragment) {
                    if (registry.fragments.contains(fragment.name.name)) {
                        throw shared::ParserError(
                            fragment.name.location.startToken,
                            "Fragment with this name already exists",
                            fragment.name.location.source);
                    };
                    registry.fragments[fragment.name.name] =
                        std::make_shared<ast::Fragment>(
                            fragment.name.name,
                            nodes::fragmentSpecFromName(fragment.typeName,
                                                        registry));
                },
                [&registry](
                    const file::client::ast::DirectiveDefinition &directive) {
                    if (registry.clientDirectives.contains(directive.name.name)) {
                        throw shared::ParserError(
                            directive.name.location.startToken,
                            "Client directive with this name already exists",
                            directive.name.location.source);
                    };
                },
            },
            astNode);
    };
    const auto &clientNodes = nodes::parseClientNodes(astNodes, registry);
    for (const auto &operation :
         clientNodes | std::views::filter([](const auto &node) {
             return std::holds_alternative<std::shared_ptr<ast::Operation>>(
                 node);
         }) | std::views::transform([](const auto &node) {
             return std::get<std::shared_ptr<ast::Operation>>(node);
         })) {
        operation->parametersHash =
            getOperationParametersHash(registry, operation->parameters);
        operation->fragmentSpecHash =
            getFragmentSpecHash(registry, operation->fragmentSpec, true);
        operation->usedFragments =
            getUsedFragmentsFromFragmentSpec(registry, operation->fragmentSpec);
    };
    return ClientSchema::fromNodes(clientNodes);
};

const Schema parseSchema(const std::vector<server::ast::ASTNode> &serverNodes,
                         const std::vector<client::ast::ASTNode> &clientNodes) {
    TypeRegistry registry;
    return { .server = parseServerSchema(registry, serverNodes),
             .client = parseClientSchema(registry, clientNodes) };
};
};  // namespace gql::parsers::schema
