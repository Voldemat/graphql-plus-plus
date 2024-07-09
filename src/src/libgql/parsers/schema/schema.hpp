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
#include "utils.hpp"

namespace parsers::schema {
struct ServerSchema {
    std::map<std::string, std::shared_ptr<ast::ObjectType>> objects;
    std::map<std::string, std::shared_ptr<ast::InputType>> inputs;
    std::map<std::string, std::shared_ptr<ast::Interface>> interfaces;
    std::map<std::string, std::shared_ptr<ast::Scalar>> scalars;
    std::map<std::string, std::shared_ptr<ast::Enum>> enums;
    std::map<std::string, std::shared_ptr<ast::Union>> unions;

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
        std::visit(overloaded{
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
                   },
                   sNode);
    };
};

struct ClientSchema {
    std::map<std::string, std::shared_ptr<ast::Fragment>> fragments;
    std::map<std::string, std::shared_ptr<ast::Operation>> operations;

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
            overloaded{ [this](const std::shared_ptr<ast::Fragment> &node) {
                           fragments[node->name] = node;
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
    std::vector<parsers::file::server::ast::FileNodes> astArray,
    std::vector<parsers::file::client::ast::ClientDefinition> definitions);

};  // namespace parsers::schema
