#pragma once

#include <map>
#include <memory>
#include <string>
#include <variant>
#include <vector>

#include "./client_ast.hpp"
#include "./server_ast.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

namespace gql::parsers::schema {
struct ServerSchema {
    std::map<std::string, std::shared_ptr<ast::ObjectType>> objects;
    std::map<std::string, std::shared_ptr<ast::InputType>> inputs;
    std::map<std::string, std::shared_ptr<ast::Interface>> interfaces;
    std::map<std::string, std::shared_ptr<ast::Scalar>> scalars;
    std::map<std::string, std::shared_ptr<ast::Enum>> enums;
    std::map<std::string, std::shared_ptr<ast::Union>> unions;
    std::map<std::string, std::shared_ptr<ast::ServerDirective>> directives;

    inline bool operator==(const ServerSchema &) const = default;

    static ServerSchema fromNodes(
        const std::vector<ast::ServerSchemaNode> &nodes) {
        ServerSchema schema;
        for (const auto &node : nodes) {
            schema.addNode(node);
        };
        return schema;
    };

    void addNode(const ast::ServerSchemaNode &sNode) {
        std::visit(
            utils::overloaded{
                [this](const std::shared_ptr<ast::ObjectType> &node) {
                    objects[node->name] = node;
                },
                [this](const std::shared_ptr<ast::Scalar> &node) {
                    scalars[node->name] = node;
                },
                [this](const std::shared_ptr<ast::InputType> &node) {
                    inputs[node->name] = node;
                },
                [this](const std::shared_ptr<ast::Enum> &node) {
                    enums[node->name] = node;
                },
                [this](const std::shared_ptr<ast::Union> &node) {
                    unions[node->name] = node;
                },
                [this](const std::shared_ptr<ast::Interface> &node) {
                    interfaces[node->name] = node;
                },
                [this](const std::shared_ptr<ast::ServerDirective> &node) {
                    directives[node->name] = node;
                },
            },
            sNode);
    };
};

struct ClientSchema {
    std::map<std::string, std::shared_ptr<ast::Fragment>> fragments;
    std::map<std::string, std::shared_ptr<ast::Operation>> operations;
    std::map<std::string, std::shared_ptr<ast::ClientDirective>> directives;

    static ClientSchema fromNodes(
        const std::vector<ast::ClientSchemaNode> &nodes) {
        ClientSchema schema;
        for (const auto &node : nodes) {
            schema.addNode(node);
        };
        return schema;
    };

    void addNode(const ast::ClientSchemaNode &sNode) {
        std::visit(
            utils::overloaded{
                [this](const std::shared_ptr<ast::Fragment> &node) {
                    fragments[node->name] = node;
                },
                [this](const std::shared_ptr<ast::ClientDirective> &node) {
                    directives[node->name] = node;
                },
                [this](const std::shared_ptr<ast::Operation> &node) {
                    operations[node->name] = node;
                } },
            sNode);
    };

    inline bool operator==(const ClientSchema &) const = default;
};

struct Schema {
    ServerSchema server;
    ClientSchema client;

    inline bool operator==(const Schema &) const = default;
};

const Schema parseSchema(
    const std::vector<parsers::file::server::ast::ASTNode> &serverNodes,
    const std::vector<parsers::file::client::ast::ASTNode> &clientNodes);
const ServerSchema parseServerSchema(
    parsers::schema::TypeRegistry &registry,
    const std::vector<file::server::ast::ASTNode> &astNodes);
const ClientSchema parseClientSchema(
    parsers::schema::TypeRegistry &registry,
    const std::vector<file::client::ast::ASTNode> &astNodes);

};  // namespace gql::parsers::schema
