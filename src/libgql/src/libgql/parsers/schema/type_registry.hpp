#pragma once

#include <map>
#include <memory>
#include <string>
#include <variant>

#include "../file/client/ast.hpp"
#include "../file/shared/ast.hpp"
#include "./client_ast.hpp"
#include "./server_ast.hpp"

namespace parsers {
namespace schema {

struct TypeRegistry {
    std::map<std::string,
             std::shared_ptr<ast::ServerDirective>>
        serverDirectives;
    std::map<std::string,
             std::shared_ptr<ast::ClientDirective>>
        clientDirectives;
    std::map<std::string,
             std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>
        queries;
    std::map<std::string,
             std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>
        mutations;
    std::map<std::string,
             std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>
        subscriptions;
    std::map<std::string, std::shared_ptr<ast::ObjectType>> objects;
    std::map<std::string, std::shared_ptr<ast::InputType>> inputs;
    std::map<std::string, std::shared_ptr<ast::Interface>> interfaces;
    std::map<std::string, std::shared_ptr<ast::Scalar>> scalars;
    std::map<std::string, std::shared_ptr<ast::Enum>> enums;
    std::map<std::string, std::shared_ptr<ast::Union>> unions;
    std::map<std::string, std::shared_ptr<ast::Fragment>> fragments;

    explicit TypeRegistry();

    [[nodiscard]] std::shared_ptr<ast::ServerDirective> getServerDirective(const std::string& name) const;
    [[nodiscard]] std::shared_ptr<ast::ClientDirective> getClientDirective(const std::string& name) const;
    [[nodiscard]] std::shared_ptr<ast::ObjectType> getQueryObject() const;
    [[nodiscard]] std::shared_ptr<ast::ObjectType> getMutationObject() const;
    [[nodiscard]] std::shared_ptr<ast::ObjectType> getSubscriptionObject()
        const;
    [[nodiscard]] ast::InputTypeSpec getTypeForInput(
        const file::shared::ast::NameNode &node) const;
    [[nodiscard]] ast::ObjectTypeSpec getTypeForObject(
        const file::shared::ast::NameNode &node) const;
    [[nodiscard]] std::shared_ptr<ast::Interface> getInterface(
        const std::string &name) const;
    [[nodiscard]] std::variant<std::shared_ptr<ast::ObjectType>,
                               std::shared_ptr<ast::Union>>
    getObjectOrUnion(const std::string &name) const;
    [[nodiscard]] std::shared_ptr<ast::ObjectType> getObject(
        const std::string &name) const;
    [[nodiscard]] std::shared_ptr<ast::Fragment> getFragment(
        const std::string &name) const;
    [[nodiscard]] std::map<
        std::string,
        std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>> &
    getMappingForOp(file::client::ast::OpType type);
    [[nodiscard]] const std::map<
        std::string,
        std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>> &
    getMappingForOp(file::client::ast::OpType type) const;
    [[nodiscard]] std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>
    getOp(file::client::ast::OpType type, const std::string &name) const;
    void addOpIfNotExists(
        const std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>
            &field,
        std::map<std::string,
                 std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>
            &mapping);
    void addNode(const ast::ServerSchemaNode &schemaNode);
    void addFragment(const std::shared_ptr<ast::Fragment> &fragment);
    void appendOpsIfSpecialObject(
        const std::string &objName,
        const std::map<std::string, std::shared_ptr<ast::FieldDefinition<
                                        ast::ObjectFieldSpec>>> &newFields);
    void patchObject(
        const std::shared_ptr<ast::ObjectType> &type,
        const std::map<std::string, std::shared_ptr<ast::FieldDefinition<
                                        ast::ObjectFieldSpec>>> &newFields);

    ast::FragmentSpec fragmentSpecFromOpType(file::client::ast::OpType type) const;
};
};  // namespace schema
};  // namespace parsers
