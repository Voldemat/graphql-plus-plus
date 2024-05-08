#include "./schema.hpp"

#include <array>
#include <format>
#include <map>
#include <memory>
#include <ranges>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "libgql/parsers/server/ast.hpp"
#include "utils.hpp"

using namespace parsers::schema;
using namespace parsers::server;

ASTSchema::ASTSchema(std::vector<ast::FileNodes> astList) {
    for (const auto &ast : astList) {
        for (const auto &defNode : ast.definitions) {
            std::visit(overloaded{
                           [this](const ast::ScalarDefinitionNode &node) {
                               scalars[node.name.name] = node;
                           },
                           [this](const ast::EnumDefinitionNode &node) {
                               enums[node.name.name] = node;
                           },
                           [this](const ast::InputObjectDefinitionNode &node) {
                               inputs[node.name.name] = node;
                           },
                           [this](const ast::ObjectDefinitionNode &node) {
                               objects[node.name.name] = node;
                           },
                           [this](const ast::UnionDefinitionNode &node) {
                               unions[node.name.name] = node;
                           },
                           [this](const ast::InterfaceDefinitionNode &node) {
                               interfaces[node.name.name] = node;
                           },
                       },
                       defNode);
        };
        for (const auto &extNode : ast.extensions) {
            const std::string &objName = extNode.typeNode.name.name;
            if (!objectsExtensions.contains(objName)) {
                objectsExtensions[objName] = {};
            };
            objectsExtensions[objName].push_back(extNode);
        };
    };
};

static const std::array builtinScalars = { Scalar("ID"), Scalar("Int"),
                                           Scalar("Float"), Scalar("String"),
                                           Scalar("Boolean") };

Schema::Schema(ASTSchema astSchema) {
    for (const auto &builtinScalar : builtinScalars) {
        scalars[builtinScalar.name] = std::make_shared<Scalar>(builtinScalar);
    };
    for (const auto &[name, scalarNode] : astSchema.scalars) {
        if (scalars.contains(name)) {
            throw std::runtime_error(std::format("Duplicate type: {}", name));
        };
        scalars[name] = std::make_shared<Scalar>(scalarNode.name.name);
    };

    for (const auto &[name, enumNode] : astSchema.enums) {
        if (enums.contains(name)) {
            throw std::runtime_error(std::format("Duplicate enum: {}", name));
        };
        enums[name] = std::make_shared<EnumType>(
            enumNode.name.name,
            enumNode.values |
                std::views::transform(
                    [](const ast::EnumValueDefinitionNode &node)
                        -> std::string { return node.value.name; }) |
                std::ranges::to<std::vector>());
    };

    for (const auto &[name, interfaceNode] : astSchema.interfaces) {
        if (interfaces.contains(name)) {
            throw std::runtime_error(
                std::format("Duplicate interface: {}", name));
        };
        interfaces[name] = std::make_shared<InterfaceType>(
            interfaceNode.name.name,
            interfaceNode.fields |
                std::views::transform(
                    [this](const ast::FieldDefinitionNode &field)
                        -> std::pair<std::string, FieldSpec<ObjectTypeKind>> {
                        return { field.name.name, parseObjectFieldSpec(field) };
                    }) |
                std::ranges::to<std::map>());
    };

    for (const auto &[name, inputNode] : astSchema.inputs) {
        if (inputs.contains(name)) {
            throw std::runtime_error(std::format("Duplicate input: {}", name));
        };
        inputs[name] = std::make_shared<InputType>(
            inputNode.name.name,
            (std::map<std::string, FieldSpec<InputTypeKind>>){});
    };

    for (const auto &[name, objectNode] : astSchema.objects) {
        if (objects.contains(name)) {
            throw std::runtime_error(
                std::format("Duplicate object type: {}", name));
        };
        objects[name] = std::make_shared<ObjectType>(
            objectNode.name.name,
            objectNode.fields |
                std::views::transform(
                    [this](const ast::FieldDefinitionNode &field)
                        -> std::pair<std::string, FieldSpec<ObjectTypeKind>> {
                        return { field.name.name, parseObjectFieldSpec(field) };
                    }) |
                std::ranges::to<std::map>());
    };

    for (const auto &[name, unionNode] : astSchema.unions) {
        if (unions.contains(name)) {
            throw std::runtime_error(
                std::format("Duplicate object type: {}", name));
        };
        unions[name] = std::make_shared<UnionType>(
            unionNode.name.name,
            unionNode.values |
                std::views::transform([this](const ast::NameNode &node)
                                          -> std::shared_ptr<ObjectType> {
                    if (!objects.contains(node.name)) {
                        throw std::runtime_error(
                            std::format("Unknown object type: {}", node.name));
                    };
                    return objects[node.name];
                }) |
                std::ranges::to<std::vector>());
    };

    for (const auto &[name, extensionNodes] : astSchema.objectsExtensions) {
        if (!objects.contains(name)) {
            throw std::runtime_error(
                std::format("Unknown object extension: {}", name));
        };
        const auto &object = objects[name];
        for (const auto &node : extensionNodes) {
            for (const auto &field : node.typeNode.fields) {
                const auto &newField = parseObjectFieldSpec(field);
                object->fields[field.name.name] = newField;
            };
        };
    };
};

FieldSpec<ObjectTypeKind> Schema::parseObjectFieldSpec(
    const ast::FieldDefinitionNode &field) {
    return std::visit<FieldSpec<ObjectTypeKind>>(
        overloaded{ [this](const ast::NamedTypeNode &node)
                        -> BasicFieldSpec<ObjectTypeKind> {
                       if (enums.contains(node.name.name)) {
                           return { .spec = { .type = enums[node.name.name],
                                              .nullable = node.nullable } };
                       };
                       if (objects.contains(node.name.name)) {
                           return { .spec = { .type = objects[node.name.name],
                                              .nullable = node.nullable } };
                       };
                       if (unions.contains(node.name.name)) {
                           return { .spec = { .type = objects[node.name.name],
                                              .nullable = node.nullable } };
                       };
                       if (!scalars.contains(node.name.name)) {
                           throw std::runtime_error(
                               std::format("Unknown type: {}", node.name.name));
                       };
                       return { .spec = { .type = scalars[node.name.name],
                                          .nullable = node.nullable } };
                   },
                    [this](const ast::ListTypeNode &node)
                        -> ListFieldSpec<ObjectTypeKind> {
                        const auto &type = node.type;
                        const auto &name = type.name.name;
                        if (enums.contains(name)) {
                            return { .spec = { .type = enums[name],
                                               .nullable = node.nullable },
                                     .nullable = node.nullable };
                        };
                        if (!scalars.contains(name)) {
                            throw std::runtime_error(
                                std::format("Unknown type: {}", name));
                        };
                        return { .spec = { .type = scalars[name],
                                           .nullable = type.nullable },
                                 .nullable = node.nullable };
                    } },
        field.type);
};
Literal Schema::parseLiteral(const ast::LiteralNode &literalNode) {
    return std::visit<Literal>(
        overloaded{
            [](const ast::LiteralIntNode &node) { return node.value; },
            [](const ast::LiteralFloatNode &node) { return node.value; },
            [](const ast::LiteralStringNode &node) { return node.value; },
            [](const ast::LiteralBooleanNode &node) { return node.value; },
            [](const ast::LiteralEnumValueNode &node) { return node.value; } },
        literalNode);
};
