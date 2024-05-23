#include "./schema.hpp"

#include <format>
#include <functional>
#include <map>
#include <memory>
#include <ranges>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/shared/shared.hpp"
#include "utils.hpp"

using namespace parsers;
using namespace parsers::schema;
using namespace parsers::server;

struct TypeRegistry {
    std::map<std::string, std::shared_ptr<ObjectType>> objects;
    std::map<std::string, std::shared_ptr<InputType>> inputs;
    std::map<std::string, std::shared_ptr<Interface>> interfaces;
    std::map<std::string, std::shared_ptr<Scalar>> scalars;
    std::map<std::string, std::shared_ptr<Enum>> enums;
    std::map<std::string, std::shared_ptr<Union>> unions;

    explicit TypeRegistry() {
        scalars["String"] = std::make_shared<Scalar>("String");
        scalars["Int"] = std::make_shared<Scalar>("Int");
        scalars["Float"] = std::make_shared<Scalar>("Float");
        scalars["Boolean"] = std::make_shared<Scalar>("Boolean");
    };

    [[nodiscard]] NodeOrLazy<std::shared_ptr<ObjectType>> getObjectOrLazy(
        const std::string &name) const {
        if (objects.contains(name)) {
            return objects.at(name);
        };
        return (LazySchemaNode){ .name = name };
    };

    [[nodiscard]] NodeOrLazy<ObjectTypeSpec> getTypeForObjectOrLazy(
        const std::string &name) const {
        if (objects.contains(name)) return objects.at(name);
        if (scalars.contains(name)) return scalars.at(name);
        if (enums.contains(name)) return enums.at(name);
        if (unions.contains(name)) return unions.at(name);
        return (LazySchemaNode){ .name = name };
    };

    [[nodiscard]] NodeOrLazy<InputTypeSpec> getTypeForInputOrLazy(
        const std::string &name) const {
        if (inputs.contains(name)) return inputs.at(name);
        if (scalars.contains(name)) return scalars.at(name);
        if (enums.contains(name)) return enums.at(name);
        return (LazySchemaNode){ .name = name };
    };

    [[nodiscard]] InputTypeSpec getTypeForInput(const std::string &name) const {
        const auto &type = getTypeForInputOrLazy(name);
        if (std::holds_alternative<LazySchemaNode>(type)) {
            throw std::runtime_error(
                std::format("Type for input with name {} is not found", name));
        };
        return std::get<InputTypeSpec>(type);
    };

    [[nodiscard]] ObjectTypeSpec getTypeForObject(
        const std::string &name) const {
        const auto &type = getTypeForObjectOrLazy(name);
        if (std::holds_alternative<LazySchemaNode>(type)) {
            throw std::runtime_error(
                std::format("Type for object with name {} is not found", name));
        };
        return std::get<ObjectTypeSpec>(type);
    };

    [[nodiscard]] std::shared_ptr<Interface> getInterface(
        const std::string &name) const {
        return interfaces.at(name);
    };

    [[nodiscard]] std::shared_ptr<ObjectType> getObject(
        const std::string &name) const {
        return objects.at(name);
    };

    void addNode(const SchemaNode &schemaNode) {
        std::visit(overloaded{ [this](const std::shared_ptr<ObjectType> &node) {
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
};

std::shared_ptr<Union> parseUnion(const ast::UnionDefinitionNode &node,
                                  const TypeRegistry &registry) {
    return std::make_shared<Union>(
        node.name.name,
        node.values |
            std::views::transform([&registry](const shared::ast::NameNode &nNode) {
                return registry.getObjectOrLazy(nNode.name);
            }) |
            std::ranges::to<std::vector>());
};

template <typename T>
std::pair<NonCallableFieldSpec<T>, bool> parseNonCallableTypeSpec(
    const shared::ast::TypeNode astNode,
    std::function<NodeOrLazy<T>(const std::string &)> typeGetter) {
    return std::visit<std::pair<NonCallableFieldSpec<T>, bool>>(
        overloaded{ [&typeGetter](const shared::ast::NamedTypeNode &node)
                        -> std::pair<LiteralFieldSpec<T>, bool> {
                       return { (LiteralFieldSpec<T>){
                                    .type = typeGetter(node.name.name) },
                                node.nullable };
                   },
                    [&typeGetter](const shared::ast::ListTypeNode &node)
                        -> std::pair<ArrayFieldSpec<T>, bool> {
                        return { (ArrayFieldSpec<T>){
                                     .type = typeGetter(node.type.name.name),
                                     .nullable = node.type.nullable },
                                 node.nullable };
                    } },
        astNode);
};

std::pair<ObjectFieldSpec, bool> parseObjectTypeSpec(
    const ast::FieldDefinitionNode &astNode, const TypeRegistry &registry) {
    const auto &[returnType, nullable] = parseNonCallableTypeSpec(
        astNode.type,
        (std::function<NodeOrLazy<ObjectTypeSpec>(
             const std::string &)>)[&registry](const std::string &name) {
            return registry.getTypeForObjectOrLazy(name);
        });
    ObjectFieldSpec returnTypeSpec = std::visit(
        [](auto &&arg) -> ObjectFieldSpec { return arg; }, returnType);
    if (astNode.arguments.empty()) return { returnTypeSpec, nullable };
    const auto &callableSpec = (CallableFieldSpec){
        .returnType = returnType,
        .arguments =
            astNode.arguments |
            std::views::transform([&registry](
                                      const shared::ast::InputValueDefinitionNode &node)
                                      -> FieldDefinition<InputFieldSpec> {
                const auto &[returnType, nullable] = parseNonCallableTypeSpec(
                    node.type, (std::function<NodeOrLazy<InputTypeSpec>(
                                    const std::string &)>)[&registry](
                                   const std::string &name) {
                        return registry.getTypeForInputOrLazy(name);
                    });
                const InputFieldSpec &returnTypeSpec =
                    std::visit([](auto &&arg) -> InputFieldSpec { return arg; },
                               returnType);

                return { .name = node.name.name,
                         .spec = returnTypeSpec,
                         .nullable = nullable };
            }) |
            std::ranges::to<std::vector>()
    };
    return { callableSpec, nullable };
};

std::pair<InputFieldSpec, bool> parseInputTypeSpec(
    const ast::FieldDefinitionNode &astNode, const TypeRegistry &registry) {
    const auto &[returnType, nullable] = parseNonCallableTypeSpec(
        astNode.type,
        (std::function<NodeOrLazy<InputTypeSpec>(
             const std::string &)>)[&registry](const std::string &name) {
            return registry.getTypeForInputOrLazy(name);
        });
    InputFieldSpec returnTypeSpec = std::visit(
        [](auto &&arg) -> InputFieldSpec { return arg; }, returnType);

    return { returnTypeSpec, nullable };
};

std::shared_ptr<Interface> parseInterface(
    const ast::InterfaceDefinitionNode &node, const TypeRegistry &registry) {
    return std::make_shared<Interface>(
        node.name.name,
        node.fields |
            std::views::transform(
                [&registry](const ast::FieldDefinitionNode &defNode)
                    -> FieldDefinition<ObjectFieldSpec> {
                    const auto &[typeSpec, nullable] =
                        parseObjectTypeSpec(defNode, registry);
                    return { .name = defNode.name.name,
                             .spec = typeSpec,
                             .nullable = nullable };
                }) |
            std::ranges::to<std::vector>());
};

std::shared_ptr<InputType> parseInput(
    const ast::InputObjectDefinitionNode &node, const TypeRegistry &registry) {
    return std::make_shared<InputType>(
        node.name.name,
        node.fields |
            std::views::transform(
                [&registry](const ast::FieldDefinitionNode &defNode)
                    -> FieldDefinition<InputFieldSpec> {
                    const auto &[typeSpec, nullable] =
                        parseInputTypeSpec(defNode, registry);
                    return { .name = defNode.name.name,
                             .spec = typeSpec,
                             .nullable = nullable };
                }) |
            std::ranges::to<std::vector>());
};

std::shared_ptr<ObjectType> parseObject(const ast::ObjectDefinitionNode &node,
                                        const TypeRegistry &registry) {
    return std::make_shared<ObjectType>(
        node.name.name,
        node.fields |
            std::views::transform(
                [&registry](const ast::FieldDefinitionNode &defNode)
                    -> FieldDefinition<ObjectFieldSpec> {
                    const auto &[typeSpec, nullable] =
                        parseObjectTypeSpec(defNode, registry);
                    return { .name = defNode.name.name,
                             .spec = typeSpec,
                             .nullable = nullable };
                }) |
            std::ranges::to<std::vector>());
};

std::pair<std::string, SchemaNode> parseServerNode(
    const ast::TypeDefinitionNode &astNode, const TypeRegistry &registry) {
    return std::visit<std::pair<std::string, SchemaNode>>(
        overloaded{
            [&registry](const ast::ScalarDefinitionNode &node)
                -> std::pair<std::string, std::shared_ptr<Scalar>> {
                if (registry.scalars.contains(node.name.name)) {
                    throw std::runtime_error(std::format(
                        "Scalar with name {} already exists", node.name.name));
                };
                return { node.name.name,
                         std::make_shared<Scalar>(node.name.name) };
            },
            [&registry](const ast::EnumDefinitionNode &node)
                -> std::pair<std::string, std::shared_ptr<Enum>> {
                if (registry.enums.contains(node.name.name)) {
                    throw std::runtime_error(std::format(
                        "Enum with name {} already exists", node.name.name));
                };
                return {
                    node.name.name,
                    std::make_shared<Enum>(
                        node.name.name,
                        node.values |
                            std::views::transform(
                                [](const ast::EnumValueDefinitionNode &vNode) {
                                    return vNode.value.name;
                                }) |
                            std::ranges::to<std::vector>())
                };
            },
            [&registry](const ast::UnionDefinitionNode &node)
                -> std::pair<std::string, std::shared_ptr<Union>> {
                return { node.name.name, parseUnion(node, registry) };
            },
            [&registry](const ast::InterfaceDefinitionNode &node)
                -> std::pair<std::string, std::shared_ptr<Interface>> {
                return { node.name.name, parseInterface(node, registry) };
            },
            [&registry](const ast::InputObjectDefinitionNode &node)
                -> std::pair<std::string, std::shared_ptr<InputType>> {
                return { node.name.name, parseInput(node, registry) };
            },
            [&registry](const ast::ObjectDefinitionNode &node)
                -> std::pair<std::string, std::shared_ptr<ObjectType>> {
                return { node.name.name, parseObject(node, registry) };
            },
        },
        astNode);
};

template <typename T>
NonCallableFieldSpec<T> replaceNonCallableFieldSpecLazyNodes(
    const NonCallableFieldSpec<T> &fSpec,
    const std::function<T(const std::string &)> typeGetter) {
    return std::visit<NonCallableFieldSpec<T>>(
        overloaded{
            [&typeGetter](
                const LiteralFieldSpec<T> &spec) -> LiteralFieldSpec<T> {
                if (std::holds_alternative<LazySchemaNode>(spec.type)) {
                    return { .type = typeGetter(
                                 std::get<LazySchemaNode>(spec.type).name) };
                };
                return spec;
            },
            [&typeGetter](const ArrayFieldSpec<T> &spec) -> ArrayFieldSpec<T> {
                if (std::holds_alternative<LazySchemaNode>(spec.type)) {
                    return { .type = typeGetter(
                                 std::get<LazySchemaNode>(spec.type).name),
                             .nullable = spec.nullable };
                };
                return spec;
            } },
        fSpec);
};

InputFieldSpec replaceInputSpecLazyNodes(const InputFieldSpec &iSpec,
                                         const TypeRegistry &registry) {
    return replaceNonCallableFieldSpecLazyNodes<InputTypeSpec>(
        iSpec, [&registry](const std::string &name) {
            return registry.getTypeForInput(name);
        });
};

FieldDefinition<InputFieldSpec> replaceInputFieldLazyNodes(
    const FieldDefinition<InputFieldSpec> &definition,
    const TypeRegistry &registry) {
    return {
        .name = definition.name,
        .spec = replaceInputSpecLazyNodes(definition.spec, registry),
        .nullable = definition.nullable,
    };
};

ObjectFieldSpec replaceObjectSpecLazyNodes(const ObjectFieldSpec &iSpec,
                                           const TypeRegistry &registry) {
    return std::visit<ObjectFieldSpec>(
        overloaded{
            [&registry](const LiteralFieldSpec<ObjectTypeSpec> &spec)
                -> LiteralFieldSpec<ObjectTypeSpec> {
                if (std::holds_alternative<LazySchemaNode>(spec.type)) {
                    return { .type = registry.getTypeForObject(
                                 std::get<LazySchemaNode>(spec.type).name) };
                };
                return spec;
            },
            [&registry](const ArrayFieldSpec<ObjectTypeSpec> &spec)
                -> ArrayFieldSpec<ObjectTypeSpec> {
                if (std::holds_alternative<LazySchemaNode>(spec.type)) {
                    return { .type = registry.getTypeForObject(
                                 std::get<LazySchemaNode>(spec.type).name),
                             .nullable = spec.nullable };
                };
                return spec;
            },
            [&registry](const CallableFieldSpec &spec) -> CallableFieldSpec {
                NodeOrLazy<NonCallableFieldSpec<ObjectTypeSpec>> returnType =
                    spec.returnType;
                if (std::holds_alternative<LazySchemaNode>(returnType)) {
                    returnType =
                        replaceNonCallableFieldSpecLazyNodes<ObjectTypeSpec>(
                            std::get<NonCallableFieldSpec<ObjectTypeSpec>>(
                                returnType),
                            [&registry](const std::string &name) {
                                return registry.getTypeForObject(name);
                            });
                };
                const auto &arguments =
                    spec.arguments |
                    std::views::transform([&registry](const auto &arg) {
                        return replaceInputFieldLazyNodes(arg, registry);
                    }) |
                    std::ranges::to<std::vector>();
                return { .returnType = returnType, .arguments = arguments };
            } },
        iSpec);
};

FieldDefinition<ObjectFieldSpec> replaceObjectFieldLazyNodes(
    const FieldDefinition<ObjectFieldSpec> &definition,
    const TypeRegistry &registry) {
    return {
        .name = definition.name,
        .spec = replaceObjectSpecLazyNodes(definition.spec, registry),
        .nullable = definition.nullable,
    };
};

SchemaNode replaceLazyNodes(const SchemaNode &sNode,
                            const TypeRegistry &registry) {
    return std::visit<SchemaNode>(
        overloaded{
            [&registry](const std::shared_ptr<InputType> &node) {
                return std::make_shared<InputType>(
                    node->name,
                    node->fields |
                        std::views::transform([&registry](const auto &field) {
                            return replaceInputFieldLazyNodes(field, registry);
                        }) |
                        std::ranges::to<std::vector>());
            },
            [&registry](const std::shared_ptr<ObjectType> &node) {
                return std::make_shared<ObjectType>(
                    node->name,
                    node->fields |
                        std::views::transform([&registry](const auto &field) {
                            return replaceObjectFieldLazyNodes(field, registry);
                        }) |
                        std::ranges::to<std::vector>(),
                    node->implements |
                        std::views::transform(
                            [&registry](const auto &interfaceNode)
                                -> NodeOrLazy<std::shared_ptr<Interface>> {
                                if (std::holds_alternative<LazySchemaNode>(
                                        interfaceNode)) {
                                    return registry.getInterface(
                                        std::get<LazySchemaNode>(interfaceNode)
                                            .name);
                                };
                                return interfaceNode;
                            }) |
                        std::ranges::to<std::vector>());
            },
            [](const std::shared_ptr<Enum> &node) { return node; },
            [](const std::shared_ptr<Scalar> &node) { return node; },
            [&registry](const std::shared_ptr<Interface> &node) {
                return std::make_shared<Interface>(
                    node->name,
                    node->fields |
                        std::views::transform([&registry](const auto &field) {
                            return replaceObjectFieldLazyNodes(field, registry);
                        }) |
                        std::ranges::to<std::vector>());
            },
            [&registry](const std::shared_ptr<Union> &node) {
                return std::make_shared<Union>(
                    node->name,
                    node->items |
                        std::views::transform(
                            [&registry](const auto &item)
                                -> NodeOrLazy<std::shared_ptr<ObjectType>> {
                                if (std::holds_alternative<LazySchemaNode>(
                                        item)) {
                                    return registry.getObject(
                                        std::get<LazySchemaNode>(item).name);
                                };
                                return item;
                            }) |
                        std::ranges::to<std::vector>());
            } },
        sNode);
};

std::vector<SchemaNode> parsers::schema::parseSchema(
    std::vector<ast::FileNodes> astArray) {
    std::vector<SchemaNode> nodes;
    TypeRegistry registry;
    for (const auto &ast : astArray) {
        for (const auto &astNode : ast.definitions) {
            const auto &[name, node] = parseServerNode(astNode, registry);
            nodes.emplace_back(node);
            registry.addNode(node);
        };
    };
    std::vector<SchemaNode> finalNodes =
        nodes | std::views::transform([&registry](auto &sNode) {
            return replaceLazyNodes(sNode, registry);
        }) |
        std::ranges::to<std::vector>();
    return finalNodes;
};
