#include "./schema.hpp"

#include <ranges>
#include <variant>
#include <vector>

#include "../file/client/ast.hpp"
#include "../file/server/ast.hpp"
#include "./nodes/fragment/fragment.hpp"
#include "./nodes/server_node.hpp"
#include "./type_registry.hpp"
#include "libgql/parsers/schema/nodes/client_node.hpp"
#include "libgql/parsers/schema/nodes/extend_nodes.hpp"

using namespace parsers::file;
using namespace parsers::schema;
using namespace parsers::schema::ast;


auto filterFragmentDefinitions(
    const std::vector<client::ast::ClientDefinition> &definitions) {
    return definitions | std::views::filter([](const auto &def) {
               return std::holds_alternative<client::ast::FragmentDefinition>(
                   def);
           }) |
           std::views::transform([](const auto &def) {
               return std::get<client::ast::FragmentDefinition>(def);
           });
};

const Schema parsers::schema::parseSchema(
    std::vector<server::ast::FileNodes> astArray,
    std::vector<client::ast::ClientDefinition> clientDefinitions) {
    TypeRegistry registry;
    for (const auto &ast : astArray) {
        for (const auto &node : ast.definitions) {
            registry.addNode(nodes::parseServerNodeFirstPass(node));
        };
    };
    const auto &serverNodes =
        nodes::parseServerNodesSecondPass(astArray, registry);
    for (const auto &[typeNode, newFields] :
         nodes::parseServerExtendNodes(astArray, registry)) {
        registry.patchObject(typeNode, newFields);
    };
    for (const auto &fragmentDefinition :
         filterFragmentDefinitions(clientDefinitions)) {
        registry.addFragment(
            nodes::parseFragmentFirstPass(fragmentDefinition, registry));
    };
    const auto &clientNodes = nodes::parseClientNodes(clientDefinitions, registry);
    return { .server = ServerSchema::fromNodes(serverNodes),
             .client = ClientSchema::fromNodes(clientNodes) };
};
