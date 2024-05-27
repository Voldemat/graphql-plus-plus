#include "./schema.hpp"

#include <algorithm>
#include <format>
#include <functional>
#include <iostream>
#include <map>
#include <memory>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "libgql/parsers/client/ast.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/shared/shared.hpp"
#include "utils.hpp"

using namespace parsers;
using namespace parsers::schema;
using namespace parsers::server;

struct TypeRegistry {
    std::map<std::string, const FieldDefinition<ObjectFieldSpec> *> queries;
    std::map<std::string, const FieldDefinition<ObjectFieldSpec> *> mutations;
    std::map<std::string, const FieldDefinition<ObjectFieldSpec> *>
        subscriptions;
    std::map<std::string, std::shared_ptr<ObjectType>> objects;
    std::map<std::string, std::shared_ptr<InputType>> inputs;
    std::map<std::string, std::shared_ptr<Interface>> interfaces;
    std::map<std::string, std::shared_ptr<Scalar>> scalars;
    std::map<std::string, std::shared_ptr<Enum>> enums;
    std::map<std::string, std::shared_ptr<Union>> unions;
    std::map<std::string, std::shared_ptr<Fragment>> fragments;

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

    [[nodiscard]] std::variant<std::shared_ptr<ObjectType>,
                               std::shared_ptr<Union>>
    getObjectOrUnion(const std::string &name) const {
        if (objects.contains(name)) return objects.at(name);
        if (unions.contains(name)) return unions.at(name);
        throw std::runtime_error(
            std::format("Object or Union with name {} is not found", name));
    };

    [[nodiscard]] std::shared_ptr<ObjectType> getObject(
        const std::string &name) const {
        if (objects.contains(name)) return objects.at(name);
        throw std::runtime_error(
            std::format("Object with name {} is not found", name));
    };

    [[nodiscard]] NodeOrLazy<std::shared_ptr<Fragment>> getFragmentOrLazy(
        const std::string &name) const {
        if (fragments.contains(name)) return fragments.at(name);
        return (LazySchemaNode){ .name = name };
    };

    [[nodiscard]] std::shared_ptr<Fragment> getFragment(
        const std::string &name) const {
        if (!fragments.contains(name)) {
            throw std::runtime_error("NOt fragment with name: " + name);
        };
        return fragments.at(name);
    };

    [[nodiscard]] const std::map<std::string,
                                 const FieldDefinition<ObjectFieldSpec> *> &
    getMappingForOp(client::ast::OpType type) const {
        switch (type) {
            case client::ast::OpType::QUERY:
                return queries;
            case client::ast::OpType::MUTATION:
                return mutations;
            case client::ast::OpType::SUBSCRIPTION:
                return subscriptions;
        };
    };

    [[nodiscard]] const FieldDefinition<ObjectFieldSpec> *getOp(
        client::ast::OpType type, const std::string &name) const {
        const auto &mapping = getMappingForOp(type);
        if (!mapping.contains(name)) {
            throw std::runtime_error(
                std::format("Operation \"{}\" does not exists", name));
        };
        return mapping.at(name);
    };

    void addOpIfNotExists(
        const FieldDefinition<ObjectFieldSpec> *field,
        std::map<std::string, const FieldDefinition<ObjectFieldSpec> *>
            &mapping) {
        if (mapping.contains(field->name)) {
            throw std::runtime_error(std::format(
                "Operation with name: \"{}\" already exists", field->name));
        };
        mapping[field->name] = field;
    };

    void addNode(const SchemaNode &schemaNode) {
        std::visit(overloaded{ [this](const std::shared_ptr<ObjectType> &node) {
                                  appendOpsIfSpecialObject(*node);
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

    void addFragment(const std::shared_ptr<Fragment> &fragment) {
        fragments[fragment->name] = fragment;
    };

    void appendOpsIfSpecialObject(const ObjectType &obj) {
        if (obj.name == "Query") {
            for (auto &field : obj.fields) {
                std::cout << "Appending query op: " << field.name << std::endl;
                addOpIfNotExists(&field, queries);
            };
        } else if (obj.name == "Mutation") {
            for (auto &field : obj.fields) {
                addOpIfNotExists(&field, mutations);
            };
        } else if (obj.name == "Subscription") {
            for (auto &field : obj.fields) {
                addOpIfNotExists(&field, subscriptions);
            };
        };
    };

    void patchObject(const ExtendObjectType &node) {
        if (!objects.contains(node.type.name)) {
            throw std::runtime_error(std::format(
                "Type with name \"{}\" does not exists", node.type.name));
        };
        auto &object = objects[node.type.name];
        for (auto &newField : node.type.fields) {
            if (std::find_if(object->fields.begin(), object->fields.end(),
                             [&newField](const auto &field) {
                                 return field.name == newField.name;
                             }) != object->fields.end()) {
                throw std::runtime_error(std::format(
                    "Field with name \"{}\" already exists", newField.name));
            };
            object->fields.push_back(newField);
        };
        appendOpsIfSpecialObject(node.type);
    };
};

std::shared_ptr<Union> parseUnion(const ast::UnionDefinitionNode &node,
                                  const TypeRegistry &registry) {
    return std::make_shared<Union>(
        node.name.name,
        node.values |
            std::views::transform(
                [&registry](const shared::ast::NameNode &nNode) {
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

FieldDefinition<InputFieldSpec> parseInputFieldDefinition(
    const shared::ast::InputValueDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &[returnType, nullable] = parseNonCallableTypeSpec(
        node.type,
        (std::function<NodeOrLazy<InputTypeSpec>(
             const std::string &)>)[&registry](const std::string &name) {
            return registry.getTypeForInputOrLazy(name);
        });
    const InputFieldSpec &returnTypeSpec = std::visit(
        [](auto &&arg) -> InputFieldSpec { return arg; }, returnType);

    return { .name = node.name.name,
             .spec = returnTypeSpec,
             .nullable = nullable };
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
            std::views::transform(
                [&registry](const shared::ast::InputValueDefinitionNode &node)
                    -> FieldDefinition<InputFieldSpec> {
                    return parseInputFieldDefinition(node, registry);
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

ObjectType parseObject(const ast::ObjectDefinitionNode &node,
                       const TypeRegistry &registry) {
    return { .name = node.name.name,
             .fields = node.fields |
                       std::views::transform(
                           [&registry](const ast::FieldDefinitionNode &defNode)
                               -> FieldDefinition<ObjectFieldSpec> {
                               const auto &[typeSpec, nullable] =
                                   parseObjectTypeSpec(defNode, registry);
                               return { .name = defNode.name.name,
                                        .spec = typeSpec,
                                        .nullable = nullable };
                           }) |
                       std::ranges::to<std::vector>() };
};

SchemaNode parseServerNode(const ast::TypeDefinitionNode &astNode,
                           const TypeRegistry &registry) {
    return std::visit<SchemaNode>(
        overloaded{
            [&registry](const ast::ScalarDefinitionNode &node)
                -> std::shared_ptr<Scalar> {
                if (registry.scalars.contains(node.name.name)) {
                    throw std::runtime_error(std::format(
                        "Scalar with name {} already exists", node.name.name));
                };
                return std::make_shared<Scalar>(node.name.name);
            },
            [&registry](
                const ast::EnumDefinitionNode &node) -> std::shared_ptr<Enum> {
                if (registry.enums.contains(node.name.name)) {
                    throw std::runtime_error(std::format(
                        "Enum with name {} already exists", node.name.name));
                };
                return std::make_shared<Enum>(
                    node.name.name,
                    node.values |
                        std::views::transform(
                            [](const ast::EnumValueDefinitionNode &vNode) {
                                return vNode.value.name;
                            }) |
                        std::ranges::to<std::vector>());
            },
            [&registry](const ast::UnionDefinitionNode &node)
                -> std::shared_ptr<Union> {
                return parseUnion(node, registry);
            },
            [&registry](const ast::InterfaceDefinitionNode &node)
                -> std::shared_ptr<Interface> {
                return parseInterface(node, registry);
            },
            [&registry](const ast::InputObjectDefinitionNode &node)
                -> std::shared_ptr<InputType> {
                return parseInput(node, registry);
            },
            [&registry](const ast::ObjectDefinitionNode &node)
                -> std::shared_ptr<ObjectType> {
                return std::make_shared<ObjectType>(
                    parseObject(node, registry));
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

Selection parseSelectionNode(const client::ast::SelectionNode &sNode,
                             const TypeRegistry &registry);

FragmentSpec parseFragmentSpec(const client::ast::FragmentSpec &spec,
                               const TypeRegistry &registry) {
    return { .selections =
                 spec.selections |
                 std::views::transform([&registry](const auto &selection) {
                     return parseSelectionNode(selection, registry);
                 }) |
                 std::ranges::to<std::vector>() };
};

Selection parseSelectionNode(const client::ast::SelectionNode &sNode,
                             const TypeRegistry &registry) {
    return std::visit<Selection>(
        overloaded{
            [&registry](
                const client::ast::FieldSelectionNode &node) -> FieldSelection {
                return (FieldSelection){
                    .name = node.field.selectionName.name,
                    .alias = node.field.name.name,
                    .selection = node.spec.transform(
                        [&registry](const auto &fragmentSpec) {
                            return std::make_shared<FragmentSpec>(
                                parseFragmentSpec(*fragmentSpec.get(),
                                                  registry));
                        })
                };
            },
            [&registry](const client::ast::SpreadSelectionNode &node)
                -> SpreadSelection {
                return (SpreadSelection){ .fragment =
                                              registry.getFragmentOrLazy(
                                                  node.fragmentName.name) };
            },
            [&registry](const client::ast::ConditionalSpreadSelectionNode &node)
                -> ConditionalSpreadSelection {
                return (ConditionalSpreadSelection){
                    .type = registry.getObjectOrUnion(node.typeName.name),
                    .selection = std::make_shared<FragmentSpec>(
                        parseFragmentSpec(*node.fragment.get(), registry))
                };
            } },
        sNode);
};

ClientSchemaNode parseClientDefinition(
    const client::ast::ClientDefinition &definition,
    const TypeRegistry &registry) {
    return std::visit<ClientSchemaNode>(
        overloaded{
            [&registry](const client::ast::FragmentDefinition &node) {
                return std::make_shared<Fragment>(
                    node.name.name, node.typeName.name,
                    std::make_shared<FragmentSpec>(
                        node.spec.selections |
                        std::views::transform([&registry](const auto &sNode) {
                            return parseSelectionNode(sNode, registry);
                        }) |
                        std::ranges::to<std::vector>()));
            },
            [&registry](const client::ast::OperationDefinition &node) {
                return std::make_shared<Operation>(
                    node.type, node.name.name,
                    node.parameters |
                        std::views::transform([&registry](const auto &arg) {
                            return parseInputFieldDefinition(arg, registry);
                        }) |
                        std::ranges::to<std::vector>(),
                    registry.getOp(node.type, node.spec.name.name),
                    node.spec.selectionName.name,
                    node.spec.args |
                        std::views::transform(
                            [](const auto &arg)
                                -> std::pair<std::string, std::string> {
                                return { arg.paramName.name, arg.name.name };
                            }) |
                        std::ranges::to<std::map>(),
                    std::make_shared<FragmentSpec>(
                        parseFragmentSpec(node.fragment, registry)));
            } },
        definition);
};

std::shared_ptr<FragmentSpec> replaceFragmentSpecLazyNodes(
    const std::shared_ptr<FragmentSpec> &fSpec, const TypeRegistry &registry);

Selection replaceSelectionNodeLazyNodes(const Selection &sNode,
                                        const TypeRegistry &registry) {
    return std::visit<Selection>(
        overloaded{
            [&registry](const FieldSelection &node) {
                return (FieldSelection){
                    .name = node.name,
                    .alias = node.alias,
                    .selection = node.selection.transform(
                        [&registry](const auto &fSpec) {
                            return replaceFragmentSpecLazyNodes(fSpec,
                                                                registry);
                        })
                };
            },
            [&registry](const SpreadSelection &node) {
                if (std::holds_alternative<LazySchemaNode>(node.fragment)) {
                    return (SpreadSelection){
                        .fragment = registry.getFragment(
                            std::get<LazySchemaNode>(node.fragment).name)
                    };
                };
                return node;
            },
            [&registry](const ConditionalSpreadSelection &node) {
                return (ConditionalSpreadSelection){
                    .type = node.type,
                    .selection =
                        replaceFragmentSpecLazyNodes(node.selection, registry)
                };
            } },
        sNode);
}

std::shared_ptr<FragmentSpec> replaceFragmentSpecLazyNodes(
    const std::shared_ptr<FragmentSpec> &fSpec, const TypeRegistry &registry) {
    return std::make_shared<FragmentSpec>(
        fSpec->selections |
        std::views::transform([&registry](const auto &sNode) {
            return replaceSelectionNodeLazyNodes(sNode, registry);
        }) |
        std::ranges::to<std::vector>());
};

ClientSchemaNode replaceClientLazyNodes(const ClientSchemaNode &sNode,
                                        const TypeRegistry &registry) {
    return std::visit<ClientSchemaNode>(
        overloaded{
            [&registry](const std::shared_ptr<Fragment> &node) {
                return std::make_shared<Fragment>(
                    node->name, node->typeName,
                    replaceFragmentSpecLazyNodes(node->spec, registry));
            },
            [&registry](const std::shared_ptr<Operation> &node) {
                return std::make_shared<Operation>(
                    node->type, node->name,
                    node->arguments |
                        std::views::transform([&registry](const auto &arg) {
                            return replaceInputFieldLazyNodes(arg, registry);
                        }) |
                        std::ranges::to<std::vector>(),
                    node->operation, node->returnFieldName,
                    node->argumentsMapping,
                    replaceFragmentSpecLazyNodes(node->fragmentSpec, registry));
            } },
        sNode);
};

ExtendObjectType parseExtendObjectType(const ast::ExtendTypeNode &node,
                                       const TypeRegistry &registry) {
    return { .type = parseObject(node.typeNode, registry) };
};

const Schema parsers::schema::parseSchema(
    std::vector<ast::FileNodes> astArray,
    std::vector<client::ast::ClientDefinition> clientDefinitions) {
    Schema schema;
    TypeRegistry registry;
    schema.serverNodes =
        astArray |
        std::views::transform([](const auto &ast) { return ast.definitions; }) |
        std::views::join |
        std::views::transform([&registry](const auto &astNode) {
            const auto &node = parseServerNode(astNode, registry);
            registry.addNode(node);
            return node;
        }) |
        std::ranges::to<std::vector>() |
        std::views::transform([&registry](auto &sNode) {
            return replaceLazyNodes(sNode, registry);
        }) |
        std::ranges::to<std::vector>();
    for (const auto &ast : astArray) {
        for (const auto &node : ast.extensions) {
            registry.patchObject(parseExtendObjectType(node, registry));
        };
    };
    schema.clientNodes =
        clientDefinitions |
        std::views::transform([&registry](const auto &definition) {
            const auto &node = parseClientDefinition(definition, registry);
            if (std::holds_alternative<std::shared_ptr<Fragment>>(node)) {
                registry.addFragment(std::get<std::shared_ptr<Fragment>>(node));
            };
            return node;
        }) |
        std::ranges::to<std::vector>() |
        std::views::transform([&registry](const auto &node) {
            return replaceClientLazyNodes(node, registry);
        }) |
        std::ranges::to<std::vector>();
    return schema;
};
