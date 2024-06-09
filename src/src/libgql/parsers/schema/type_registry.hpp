#pragma once

#include <map>
#include <memory>
#include <string>
#include <variant>

#include "./schema.hpp"
#include "libgql/parsers/client/ast.hpp"
#include "libgql/parsers/shared/shared.hpp"

namespace parsers {
namespace schema {

struct TypeRegistry {
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        queries;
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        mutations;
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        subscriptions;
    std::map<std::string, std::shared_ptr<ObjectType>> objects;
    std::map<std::string, std::shared_ptr<InputType>> inputs;
    std::map<std::string, std::shared_ptr<Interface>> interfaces;
    std::map<std::string, std::shared_ptr<Scalar>> scalars;
    std::map<std::string, std::shared_ptr<Enum>> enums;
    std::map<std::string, std::shared_ptr<Union>> unions;
    std::map<std::string, std::shared_ptr<Fragment>> fragments;

    explicit TypeRegistry();

    [[nodiscard]] std::shared_ptr<ObjectType> getQueryObject() const;
    [[nodiscard]] std::shared_ptr<ObjectType> getMutationObject() const;
    [[nodiscard]] std::shared_ptr<ObjectType> getSubscriptionObject() const;
    [[nodiscard]] InputTypeSpec getTypeForInput(
        const shared::ast::NameNode &node) const;
    [[nodiscard]] ObjectTypeSpec getTypeForObject(
        const shared::ast::NameNode &node) const;
    [[nodiscard]] std::shared_ptr<Interface> getInterface(
        const std::string &name) const;
    [[nodiscard]] std::variant<std::shared_ptr<ObjectType>,
                               std::shared_ptr<Union>>
    getObjectOrUnion(const std::string &name) const;
    [[nodiscard]] std::shared_ptr<ObjectType> getObject(
        const std::string &name) const;
    [[nodiscard]] std::shared_ptr<Fragment> getFragment(
        const std::string &name) const;
    [[nodiscard]] std::map<std::string,
                           std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> &
    getMappingForOp(client::ast::OpType type);
    [[nodiscard]] const std::map<
        std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> &
    getMappingForOp(client::ast::OpType type) const;
    [[nodiscard]] std::shared_ptr<FieldDefinition<ObjectFieldSpec>> getOp(
        client::ast::OpType type, const std::string &name) const;
    void addOpIfNotExists(
        const std::shared_ptr<FieldDefinition<ObjectFieldSpec>> &field,
        std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
            &mapping);
    void addNode(const ServerSchemaNode &schemaNode);
    void addFragment(const std::shared_ptr<Fragment> &fragment);
    void appendOpsIfSpecialObject(
        const std::string &objName,
        const std::map<std::string,
                       std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
            &newFields);
    void patchObject(
        const std::shared_ptr<ObjectType> &type,
        const std::map<std::string,
                       std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
            &newFields);
};
};  // namespace schema
};  // namespace parsers
