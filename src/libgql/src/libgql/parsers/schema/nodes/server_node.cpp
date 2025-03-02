#include "./server_node.hpp"

#include <memory>
#include <ranges>
#include <variant>
#include <vector>

#include "../../file/server/ast.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"
#include "./input.hpp"
#include "./interface.hpp"
#include "./object.hpp"
#include "./union.hpp"
#include "libgql/parsers/schema/nodes/server_directive.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "utils.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
ast::ServerSchemaNode parseServerNodeFirstPass(
    const server::ast::TypeDefinitionNode &astNode) {
    return std::visit<ast::ServerSchemaNode>(
        overloaded{
            [](const server::ast::ScalarDefinitionNode &node) {
                return std::make_shared<ast::Scalar>(node.name.name);
            },
            [](const server::ast::EnumDefinitionNode &node) {
                return std::make_shared<ast::Enum>(
                    node.name.name,
                    node.values | std::views::transform([](const auto &v) {
                        return v.value.name;
                    }) | std::ranges::to<std::vector>());
            },
            [](const server::ast::UnionDefinitionNode &node) {
                return std::make_shared<ast::Union>(node.name.name);
            },
            [](const server::ast::ObjectDefinitionNode &node) {
                return std::make_shared<ast::ObjectType>(node.name.name);
            },
            [](const server::ast::InputObjectDefinitionNode &node) {
                return std::make_shared<ast::InputType>(node.name.name);
            },
            [](const server::ast::DirectiveDefinitionNode &node) {
                return std::make_shared<ast::ServerDirective>(node.name.name);
            },
            [](const server::ast::InterfaceDefinitionNode &node) {
                return std::make_shared<ast::Interface>(node.name.name);
            } },
        astNode);
};

ast::ServerSchemaNode parseServerNodeSecondPass(
    const file::server::ast::TypeDefinitionNode &astNode,
    const TypeRegistry &registry) {
    return std::visit<ast::ServerSchemaNode>(
        overloaded{
            [&registry](const server::ast::ScalarDefinitionNode &node)
                -> std::shared_ptr<ast::Scalar> {
                return registry.scalars.at(node.name.name);
            },
            [&registry](const server::ast::EnumDefinitionNode &node)
                -> std::shared_ptr<ast::Enum> {
                return registry.enums.at(node.name.name);
            },
            [&registry](const server::ast::UnionDefinitionNode &node)
                -> std::shared_ptr<ast::Union> {
                return nodes::parseUnion(node, registry);
            },
            [&registry](const server::ast::InterfaceDefinitionNode &node)
                -> std::shared_ptr<ast::Interface> {
                return nodes::parseInterface(node, registry);
            },
            [&registry](const server::ast::InputObjectDefinitionNode &node)
                -> std::shared_ptr<ast::InputType> {
                return nodes::parseInput(node, registry);
            },
            [&registry](const server::ast::ObjectDefinitionNode &node)
                -> std::shared_ptr<ast::ObjectType> {
                return nodes::parseObject(node, registry);
            },
            [&registry](const server::ast::DirectiveDefinitionNode &node)
                -> std::shared_ptr<ast::ServerDirective> {
                return nodes::parseServerDirective(node, registry);
            },
        },
        astNode);
};

std::vector<ast::ServerSchemaNode> parseServerNodesSecondPass(
    const std::vector<server::ast::FileNodes> &astArray,
    const TypeRegistry &registry) {
    return astArray | std::views::transform([](const auto &ast) {
               return ast.definitions;
           }) |
           std::views::join | std::views::transform([&registry](auto &sNode) {
               return nodes::parseServerNodeSecondPass(sNode, registry);
           }) |
           std::ranges::to<std::vector>();
};

};  // namespace parsers::schema::nodes
