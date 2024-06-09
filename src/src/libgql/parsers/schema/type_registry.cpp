#include "./type_registry.hpp"

#include <format>
#include <map>
#include <memory>
#include <stdexcept>
#include <string>
#include <variant>

#include "libgql/parsers/client/ast.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/shared/shared.hpp"
#include "utils.hpp"

using namespace parsers::schema;

TypeRegistry::TypeRegistry() {
    scalars["String"] = std::make_shared<Scalar>("String");
    scalars["Int"] = std::make_shared<Scalar>("Int");
    scalars["Float"] = std::make_shared<Scalar>("Float");
    scalars["Boolean"] = std::make_shared<Scalar>("Boolean");
};

[[nodiscard]] std::shared_ptr<ObjectType> TypeRegistry::getQueryObject() const {
    return objects.at("Query");
};

[[nodiscard]] std::shared_ptr<ObjectType> TypeRegistry::getMutationObject()
    const {
    return objects.at("Mutation");
};

[[nodiscard]] std::shared_ptr<ObjectType> TypeRegistry::getSubscriptionObject()
    const {
    return objects.at("Subscription");
};

[[nodiscard]] InputTypeSpec TypeRegistry::getTypeForInput(
    const shared::ast::NameNode &node) const {
    const auto &name = node.name;
    if (inputs.contains(name)) return inputs.at(name);
    if (scalars.contains(name)) return scalars.at(name);
    if (enums.contains(name)) return enums.at(name);
    throw shared::ParserError(node.location.startToken,
                              "Type with this name does not exists",
                              node.location.source);
};

[[nodiscard]] ObjectTypeSpec TypeRegistry::getTypeForObject(
    const shared::ast::NameNode &node) const {
    const auto &name = node.name;
    if (objects.contains(name)) return objects.at(name);
    if (interfaces.contains(name)) return interfaces.at(name);
    if (scalars.contains(name)) return scalars.at(name);
    if (enums.contains(name)) return enums.at(name);
    if (unions.contains(name)) return unions.at(name);
    throw shared::ParserError(node.location.startToken,
                              "Type with this name does not exists",
                              node.location.source);
};

[[nodiscard]] std::shared_ptr<Interface> TypeRegistry::getInterface(
    const std::string &name) const {
    return interfaces.at(name);
};

[[nodiscard]] std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Union>>
TypeRegistry::getObjectOrUnion(const std::string &name) const {
    if (objects.contains(name)) return objects.at(name);
    if (unions.contains(name)) return unions.at(name);
    throw std::runtime_error(
        std::format("Object or Union with name {} is not found", name));
};

[[nodiscard]] std::shared_ptr<ObjectType> TypeRegistry::getObject(
    const std::string &name) const {
    if (objects.contains(name)) return objects.at(name);
    throw std::runtime_error(
        std::format("Object with name {} is not found", name));
};

[[nodiscard]] std::shared_ptr<Fragment> TypeRegistry::getFragment(
    const std::string &name) const {
    if (fragments.contains(name)) return fragments.at(name);
    throw std::runtime_error("Not fragment with name: " + name);
};

[[nodiscard]] std::map<std::string,
                       std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> &
TypeRegistry::getMappingForOp(client::ast::OpType type) {
    switch (type) {
        case client::ast::OpType::QUERY:
            return queries;
        case client::ast::OpType::MUTATION:
            return mutations;
        case client::ast::OpType::SUBSCRIPTION:
            return subscriptions;
    };
};

[[nodiscard]] const std::map<
    std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> &
TypeRegistry::getMappingForOp(client::ast::OpType type) const {
    switch (type) {
        case client::ast::OpType::QUERY:
            return queries;
        case client::ast::OpType::MUTATION:
            return mutations;
        case client::ast::OpType::SUBSCRIPTION:
            return subscriptions;
    };
};

[[nodiscard]] std::shared_ptr<FieldDefinition<ObjectFieldSpec>>
TypeRegistry::getOp(client::ast::OpType type, const std::string &name) const {
    const auto &mapping = getMappingForOp(type);
    if (!mapping.contains(name)) {
        throw std::runtime_error(
            std::format("Operation \"{}\" does not exists", name));
    };
    return mapping.at(name);
};

void TypeRegistry::addOpIfNotExists(
    const std::shared_ptr<FieldDefinition<ObjectFieldSpec>> &field,
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        &mapping) {
    if (mapping.contains(field->name)) {
        throw std::runtime_error(std::format(
            "Operation with name: \"{}\" already exists", field->name));
    };
    mapping[field->name] = field;
};

void TypeRegistry::addNode(const ServerSchemaNode &schemaNode) {
    std::visit(overloaded{ [this](const std::shared_ptr<ObjectType> &node) {
                              appendOpsIfSpecialObject(node->name,
                                                       node->fields);
                              objects[node->name] = node;
                          },
                           [this](const std::shared_ptr<Interface> &node) {
                               interfaces[node->name] = node;
                           },
                           [this](const std::shared_ptr<Union> &node) {
                               unions[node->name] = node;
                           },
                           [this](const std::shared_ptr<InputType> &node) {
                               inputs[node->name] = node;
                           },
                           [this](const std::shared_ptr<Enum> &node) {
                               enums[node->name] = node;
                           },
                           [this](const std::shared_ptr<Scalar> &node) {
                               scalars[node->name] = node;
                           } },
               schemaNode);
};

void TypeRegistry::addFragment(const std::shared_ptr<Fragment> &fragment) {
    fragments[fragment->name] = fragment;
};

void TypeRegistry::appendOpsIfSpecialObject(
    const std::string &objName,
    const std::map<std::string,
                   std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        &newFields) {
    const auto &opType = client::ast::opTypeFromObjectName(objName);
    if (!opType.has_value()) return;
    auto &mapping = getMappingForOp(opType.value());
    for (auto &[_, field] : newFields) {
        addOpIfNotExists(field, mapping);
    };
};

void TypeRegistry::patchObject(
    const std::shared_ptr<ObjectType> &type,
    const std::map<std::string,
                   std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        &newFields) {
    for (auto &[name, newField] : newFields) {
        if (type->fields.contains(name)) {
            throw std::runtime_error(
                std::format("Field with name \"{}\" already exists", name));
        };
        type->fields[name] = newField;
    };
    TypeRegistry::appendOpsIfSpecialObject(type->name, newFields);
};
