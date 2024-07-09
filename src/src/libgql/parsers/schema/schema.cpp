#include "./schema.hpp"

#include <algorithm>
#include <format>
#include <map>
#include <memory>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "../file/client/ast.hpp"
#include "../file/server/ast.hpp"
#include "../file/shared/ast.hpp"
#include "../file/shared/parser_error.hpp"
#include "./client_ast.hpp"
#include "./server_ast.hpp"
#include "./type_registry.hpp"
#include "utils.hpp"

using namespace parsers::file;
using namespace parsers::schema;
using namespace parsers::schema::ast;

bool InputFieldSpec_hasDefaultValue(const InputFieldSpec &spec) {
    return std::visit<bool>(
        overloaded{ [](const LiteralFieldSpec<InputTypeSpec> &node) {
                       return node.hasDefaultValue();
                   },
                    [](const ArrayFieldSpec<InputTypeSpec> &node) {
                        return node.hasDefaultValue();
                    } },
        spec);
};

InputTypeSpec extractInputTypeSpec(const InputFieldSpec &spec) {
    return std::visit<InputTypeSpec>(
        overloaded{ [](const LiteralFieldSpec<InputTypeSpec> &node) {
                       return node.type;
                   },
                    [](const ArrayFieldSpec<InputTypeSpec> &node) {
                        return node.type;
                    } },
        spec);
};

std::shared_ptr<Union> parseUnion(const server::ast::UnionDefinitionNode &node,
                                  const TypeRegistry &registry) {
    const auto &obj = registry.unions.at(node.name.name);
    obj->items =
        node.values |
        std::views::transform(
            [&registry](const shared::ast::NameNode &nNode)
                -> std::pair<std::string, std::shared_ptr<ObjectType>> {
                return { nNode.name, registry.getObject(nNode.name) };
            }) |
        std::ranges::to<std::map>();
    return obj;
};

Literal parseLiteralNode(const shared::ast::LiteralNode &literal,
                         const InputTypeSpec &spec) {
    return std::visit<Literal>(
        overloaded{ [&spec](const shared::ast::LiteralEnumValueNode &node) {
                       if (!std::holds_alternative<std::shared_ptr<Enum>>(
                               spec)) {
                           throw shared::ParserError(
                               node.location.startToken,
                               "Unexpected default value identifier",
                               node.location.source);
                       };
                       const auto &type = std::get<std::shared_ptr<Enum>>(spec);
                       if (std::find(type->values.begin(), type->values.end(),
                                     node.value) == type->values.end()) {
                           throw shared::ParserError(
                               node.location.startToken,
                               std::format("Enum {} doesn`t have value {}",
                                           type->name, node.value),
                               node.location.source);
                       };
                       return node.value;
                   },
                    [](const auto &node) -> Literal { return node.value; } },
        literal);
};

std::pair<NonCallableFieldSpec<InputTypeSpec>, bool>
parseNonCallableInputTypeSpec(
    const shared::ast::TypeNode astNode,
    const std::optional<shared::ast::LiteralNode> defaultValueNode,
    const TypeRegistry &registry) {
    return std::visit<std::pair<NonCallableFieldSpec<InputTypeSpec>, bool>>(
        overloaded{
            [&registry,
             &defaultValueNode](const shared::ast::NamedTypeNode &node)
                -> std::pair<LiteralFieldSpec<InputTypeSpec>, bool> {
                const auto &type = registry.getTypeForInput(node.name);
                return { (LiteralFieldSpec<InputTypeSpec>){
                             { .defaultValue = defaultValueNode.transform(
                                   [&type](const auto &literal) {
                                       return parseLiteralNode(literal, type);
                                   }) },
                             .type = type },
                         node.nullable };
            },
            [&registry](const shared::ast::ListTypeNode &node)
                -> std::pair<ArrayFieldSpec<InputTypeSpec>, bool> {
                return { (ArrayFieldSpec<InputTypeSpec>){
                             .type = registry.getTypeForInput(node.type.name),
                             .nullable = node.type.nullable },
                         node.nullable };
            } },
        astNode);
};

std::pair<NonCallableFieldSpec<ObjectTypeSpec>, bool>
parseNonCallableObjectTypeSpec(const shared::ast::TypeNode astNode,
                               const TypeRegistry &registry) {
    return std::visit<std::pair<NonCallableFieldSpec<ObjectTypeSpec>, bool>>(
        overloaded{
            [&registry](const shared::ast::NamedTypeNode &node)
                -> std::pair<LiteralFieldSpec<ObjectTypeSpec>, bool> {
                return { (LiteralFieldSpec<ObjectTypeSpec>){
                             .type = registry.getTypeForObject(node.name) },
                         node.nullable };
            },
            [&registry](const shared::ast::ListTypeNode &node)
                -> std::pair<ArrayFieldSpec<ObjectTypeSpec>, bool> {
                return { (ArrayFieldSpec<ObjectTypeSpec>){
                             .type = registry.getTypeForObject(node.type.name),
                             .nullable = node.type.nullable },
                         node.nullable };
            } },
        astNode);
};

FieldDefinition<InputFieldSpec> parseInputFieldDefinition(
    const shared::ast::InputValueDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &[returnType, nullable] =
        parseNonCallableInputTypeSpec(node.type, node.defaultValue, registry);
    const InputFieldSpec &returnTypeSpec = std::visit(
        [](auto &&arg) -> InputFieldSpec { return arg; }, returnType);

    return { .name = node.name.name,
             .spec = returnTypeSpec,
             .nullable = nullable };
};

std::pair<ObjectFieldSpec, bool> parseObjectTypeSpec(
    const server::ast::FieldDefinitionNode &astNode,
    const TypeRegistry &registry) {
    const auto &[returnType, nullable] =
        parseNonCallableObjectTypeSpec(astNode.type, registry);
    ObjectFieldSpec returnTypeSpec = std::visit(
        [](auto &&arg) -> ObjectFieldSpec { return arg; }, returnType);
    if (astNode.arguments.empty()) return { returnTypeSpec, nullable };
    const auto &callableSpec = (CallableFieldSpec){
        .returnType = returnType,
        .arguments =
            astNode.arguments |
            std::views::transform(
                [&registry](const shared::ast::InputValueDefinitionNode &node)
                    -> std::pair<
                        std::string,
                        std::shared_ptr<FieldDefinition<InputFieldSpec>>> {
                    return { node.name.name,
                             std::make_shared<FieldDefinition<InputFieldSpec>>(
                                 parseInputFieldDefinition(node, registry)) };
                }) |
            std::ranges::to<std::map>()
    };
    return { callableSpec, nullable };
};

std::pair<InputFieldSpec, bool> parseInputTypeSpec(
    const server::ast::FieldDefinitionNode &astNode,
    const TypeRegistry &registry) {
    const auto &[returnType, nullable] =
        parseNonCallableInputTypeSpec(astNode.type, std::nullopt, registry);
    InputFieldSpec returnTypeSpec = std::visit(
        [](auto &&arg) -> InputFieldSpec { return arg; }, returnType);

    return { returnTypeSpec, nullable };
};

std::shared_ptr<Interface> parseInterface(
    const server::ast::InterfaceDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.interfaces.at(node.name.name);
    obj->fields =
        node.fields |
        std::views::transform(
            [&registry](const server::ast::FieldDefinitionNode &defNode)
                -> std::pair<
                    std::string,
                    std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> {
                const auto &[typeSpec, nullable] =
                    parseObjectTypeSpec(defNode, registry);
                return { defNode.name.name,
                         std::make_shared<FieldDefinition<ObjectFieldSpec>>(
                             defNode.name.name, typeSpec, nullable) };
            }) |
        std::ranges::to<std::map>();
    return obj;
};

std::shared_ptr<InputType> parseInput(
    const server::ast::InputObjectDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.inputs.at(node.name.name);
    obj->fields =
        node.fields |
        std::views::transform(
            [&registry](const server::ast::FieldDefinitionNode &defNode)
                -> std::pair<std::string, FieldDefinition<InputFieldSpec>> {
                const auto &[typeSpec, nullable] =
                    parseInputTypeSpec(defNode, registry);
                return { defNode.name.name,
                         { .name = defNode.name.name,
                           .spec = typeSpec,
                           .nullable = nullable } };
            }) |
        std::ranges::to<std::map>();
    return obj;
};

std::shared_ptr<ObjectType> parseObject(
    const server::ast::ObjectDefinitionNode &node,
    const TypeRegistry &registry) {
    const auto &obj = registry.objects.at(node.name.name);
    obj->fields =
        node.fields |
        std::views::transform(
            [&registry](const server::ast::FieldDefinitionNode &defNode)
                -> std::pair<
                    std::string,
                    std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> {
                const auto &[typeSpec, nullable] =
                    parseObjectTypeSpec(defNode, registry);

                return { defNode.name.name,
                         std::make_shared<FieldDefinition<ObjectFieldSpec>>(
                             defNode.name.name, typeSpec, nullable) };
            }) |
        std::ranges::to<std::map>();
    obj->implements =
        node.interfaces |
        std::views::transform(
            [&registry](const shared::ast::NameNode &node)
                -> std::pair<std::string, std::shared_ptr<Interface>> {
                return { node.name, registry.interfaces.at(node.name) };
            }) |
        std::ranges::to<std::map>();
    return obj;
};

ServerSchemaNode parseServerNode(const server::ast::TypeDefinitionNode &astNode,
                                 const TypeRegistry &registry) {
    return std::visit<ServerSchemaNode>(
        overloaded{
            [&registry](const server::ast::ScalarDefinitionNode &node)
                -> std::shared_ptr<Scalar> {
                return registry.scalars.at(node.name.name);
            },
            [&registry](const server::ast::EnumDefinitionNode &node)
                -> std::shared_ptr<Enum> {
                return registry.enums.at(node.name.name);
            },
            [&registry](const server::ast::UnionDefinitionNode &node)
                -> std::shared_ptr<Union> {
                return parseUnion(node, registry);
            },
            [&registry](const server::ast::InterfaceDefinitionNode &node)
                -> std::shared_ptr<Interface> {
                return parseInterface(node, registry);
            },
            [&registry](const server::ast::InputObjectDefinitionNode &node)
                -> std::shared_ptr<InputType> {
                return parseInput(node, registry);
            },
            [&registry](const server::ast::ObjectDefinitionNode &node)
                -> std::shared_ptr<ObjectType> {
                return parseObject(node, registry);
            },
        },
        astNode);
};

std::pair<
    std::shared_ptr<ObjectType>,
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>>
parseExtendObjectType(const server::ast::ExtendTypeNode &node,
                      const TypeRegistry &registry) {
    if (!registry.objects.contains(node.typeNode.name.name)) {
        throw shared::ParserError(node.typeNode.name.location.startToken,
                                  "Type with this name does not exists",
                                  node.typeNode.name.location.source);
    };
    return {
        registry.getObject(node.typeNode.name.name),
        node.typeNode.fields |
            std::views::transform(
                [&registry](const auto &field)
                    -> std::pair<
                        std::string,
                        std::shared_ptr<FieldDefinition<ObjectFieldSpec>>> {
                    const auto &[typeSpec, nullable] =
                        parseObjectTypeSpec(field, registry);

                    return { field.name.name,
                             std::make_shared<FieldDefinition<ObjectFieldSpec>>(
                                 field.name.name, typeSpec, nullable) };
                }) |
            std::ranges::to<std::map>()
    };
};

FragmentSpec fragmentSpecFromName(const shared::ast::NameNode &typeName,
                                  const TypeRegistry &registry) {
    if (registry.objects.contains(typeName.name)) {
        return (ObjectFragmentSpec<ObjectType>){ .type = registry.getObject(
                                                     typeName.name) };
    } else if (registry.unions.contains(typeName.name)) {
        return (UnionFragmentSpec){ .type = registry.unions.at(typeName.name) };
    } else if (registry.interfaces.contains(typeName.name)) {
        return (ObjectFragmentSpec<Interface>){ .type = registry.getInterface(
                                                    typeName.name) };
    };
    throw shared::ParserError(
        typeName.location.startToken,
        "Object type or union with this name does not exists",
        typeName.location.source);
};

std::shared_ptr<Fragment> parseFragmentFirstPass(
    const client::ast::FragmentDefinition &definition,
    const TypeRegistry &registry) {
    const auto &name = definition.name.name;
    if (registry.fragments.contains(name)) {
        throw shared::ParserError(definition.name.location.startToken,
                                  "Fragment with this name already exists",
                                  definition.name.location.source);
    };
    return std::make_shared<Fragment>(
        name, fragmentSpecFromName(definition.typeName, registry));
};

FragmentSpec parseFragmentSpec(const client::ast::FragmentSpec &defSpec,
                               const FragmentSpec &spec,
                               const TypeRegistry &registry);

ObjectTypeSpec getReturnTypeFromNonCallableFieldSpec(
    const NonCallableFieldSpec<ObjectTypeSpec> &fSpec) {
    return std::visit<ObjectTypeSpec>(
        overloaded{
            [](const LiteralFieldSpec<ObjectTypeSpec> &node) -> ObjectTypeSpec {
                return node.type;
            },
            [](const ArrayFieldSpec<ObjectTypeSpec> &node) -> ObjectTypeSpec {
                return node.type;
            } },
        fSpec);
};

ObjectTypeSpec getReturnTypeFromObjectFieldSpec(const ObjectFieldSpec &spec) {
    return std::visit<ObjectTypeSpec>(
        overloaded{ [](const LiteralFieldSpec<ObjectTypeSpec> &node) {
                       return node.type;
                   },
                    [](const CallableFieldSpec &node) {
                        return getReturnTypeFromNonCallableFieldSpec(
                            node.returnType);
                    },
                    [](const ArrayFieldSpec<ObjectTypeSpec> &node) {
                        return node.type;
                    } },
        spec);
};

FragmentSpec fragmentSpecFromFieldDefinition(
    const FieldDefinition<ObjectFieldSpec> &field,
    const client::ast::FragmentSpec &sNode) {
    const ObjectTypeSpec &typeSpec =
        getReturnTypeFromObjectFieldSpec(field.spec);
    if (std::holds_alternative<std::shared_ptr<ObjectType>>(typeSpec)) {
        const auto &type = std::get<std::shared_ptr<ObjectType>>(typeSpec);
        return (ObjectFragmentSpec<ObjectType>){ .type = type };
    } else if (std::holds_alternative<std::shared_ptr<Interface>>(typeSpec)) {
        const auto &type = std::get<std::shared_ptr<Interface>>(typeSpec);
        return (ObjectFragmentSpec<Interface>){ .type = type };
    } else if (std::holds_alternative<std::shared_ptr<Union>>(typeSpec)) {
        const auto &type = std::get<std::shared_ptr<Union>>(typeSpec);
        return (UnionFragmentSpec){ .type = type };
    };
    throw shared::ParserError(sNode.location.startToken,
                              "Cannot have selection on literal type field",
                              sNode.location.source);
};

FieldSelectionArgument parseSelectionArgument(const client::ast::Argument &node,
                                              const CallableFieldSpec &spec) {
    if (!spec.arguments.contains(node.name.name)) {
        throw shared::ParserError(node.name.location.startToken,
                                  "Argument with this name does not exists",
                                  node.name.location.source);
    };
    const FieldSelectionArgument &arg = { .name = node.name.name,
                                          .parameterName = node.paramName.name,
                                          .type = spec.arguments.at(
                                              node.name.name) };
    return arg;
};

template <typename T>
FieldSelection parseFieldSelectionNode(
    const client::ast::FieldSelectionNode &fNode,
    const std::shared_ptr<T> &type, const TypeRegistry &registry) {
    return std::visit<FieldSelection>(
        overloaded{
            [&type, &fNode,
             &registry](const client::ast::ObjectLiteralFieldSpec &node)
                -> FieldSelection {
                if (!(type->fields.contains(node.name.name) ||
                      node.name.name == "__typename")) {
                    throw shared::ParserError(
                        node.name.location.startToken,
                        std::format("Unknown field {} on type {}",
                                    node.name.name, type->name),
                        node.name.location.source);
                };
                const auto &spec = fNode.spec.transform(
                    [&registry, &node,
                     &type](const std::shared_ptr<client::ast::FragmentSpec>
                                &sNode) {
                        return std::make_shared<FragmentSpec>(parseFragmentSpec(
                            *sNode,
                            fragmentSpecFromFieldDefinition(
                                *type->fields.at(node.name.name), *sNode),
                            registry));
                    });
                return { .name = node.name.name,
                         .alias = node.selectionName.name,
                         .selection = spec };
            },
            [&type, &registry,
             &fNode](const client::ast::ObjectCallableFieldSpec &node)
                -> FieldSelection {
                if (!type->fields.contains(node.name.name)) {
                    throw shared::ParserError(
                        node.name.location.startToken,
                        std::format("Unknown field {} on type {}",
                                    node.name.name, type->name),
                        node.name.location.source);
                };
                const std::shared_ptr<FieldDefinition<ObjectFieldSpec>> &fType =
                    type->fields.at(node.name.name);
                if (!std::holds_alternative<CallableFieldSpec>(fType->spec)) {
                    throw shared::ParserError(
                        node.location.startToken,
                        "Callable selection is forbidden on uncallable fields",
                        node.location.source);
                };
                const auto &spec = std::get<CallableFieldSpec>(fType->spec);
                return { .name = node.name.name,
                         .alias = node.selectionName.name,
                         .arguments =
                             node.arguments |
                             std::views::transform(
                                 [&spec](const auto &arg)
                                     -> std::pair<std::string,
                                                  FieldSelectionArgument> {
                                     return { arg.name.name,
                                              parseSelectionArgument(arg,
                                                                     spec) };
                                 }) |
                             std::ranges::to<std::map>(),
                         .selection = fNode.spec.transform(
                             [&registry, &fType](const auto &selectionNode) {
                                 return std::make_shared<FragmentSpec>(
                                     parseFragmentSpec(
                                         *selectionNode,
                                         fragmentSpecFromFieldDefinition(
                                             *fType, *selectionNode),
                                         registry));
                             }) };
            } },
        fNode.field);
};

SpreadSelection parseSpreadSelectionNode(
    const client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<Union> &type, const TypeRegistry &registry) {
    if (!registry.fragments.contains(node.fragmentName.name)) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment with this name does not exists",
                                  node.fragmentName.location.source);
    };
    const auto &fragment = registry.fragments.at(node.fragmentName.name);
    const bool isUnion =
        std::holds_alternative<UnionFragmentSpec>(fragment->spec);
    const bool isInterface =
        std::holds_alternative<ObjectFragmentSpec<Interface>>(fragment->spec);
    if (!isUnion && !isInterface) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment has invalid type",
                                  node.fragmentName.location.source);
    };
    if (isUnion) {
        const auto &unionFragment = std::get<UnionFragmentSpec>(fragment->spec);
        const auto &fType = unionFragment.type;
        if (fType != type) {
            throw shared::ParserError(
                node.fragmentName.location.startToken,
                std::format("Fragment has type {} while expected {}",
                            fType->name, type->name),
                node.fragmentName.location.source);
        };
    } else {
        const auto &objectFragment =
            std::get<ObjectFragmentSpec<Interface>>(fragment->spec);
        const auto &fType = objectFragment.type;
        for (const auto &itemType : type->items | std::views::values) {
            if (!itemType->implements.contains(fType->name)) {
                throw shared::ParserError(
                    node.fragmentName.location.startToken,
                    std::format(
                        "Fragment on interface {} cannot be used here, as not "
                        "all types in union implement this interface",
                        fType->name),
                    node.fragmentName.location.source);
            };
        };
    };
    return (SpreadSelection){ .fragment = fragment };
};

SpreadSelection parseSpreadSelectionNode(
    const client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<Interface> &type, const TypeRegistry &registry) {
    if (!registry.fragments.contains(node.fragmentName.name)) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment with this name does not exists",
                                  node.fragmentName.location.source);
    };
    const auto &fragment = registry.fragments.at(node.fragmentName.name);
    if (!std::holds_alternative<ObjectFragmentSpec<Interface>>(
            fragment->spec) ||
        std::get<ObjectFragmentSpec<Interface>>(fragment->spec).type != type) {
        throw shared::ParserError(
            node.fragmentName.location.startToken,
            std::format("Fragment has type {} while expected {}",
                        fragment->name, type->name),
            node.fragmentName.location.source);
    };
    return (SpreadSelection){ .fragment = fragment };
};

struct GetFieldSelection {};
const auto getFieldSelection = GetFieldSelection();

auto operator|(const std::vector<ObjectSelection> &items,
               const GetFieldSelection &) {
    return items | std::views::filter([](const auto &node) {
               return std::holds_alternative<FieldSelection>(node);
           }) |
           std::views::transform(
               [](const auto &node) { return std::get<FieldSelection>(node); });
};

SpreadSelection parseSpreadSelectionNode(
    const client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ObjectType> &type, const TypeRegistry &registry) {
    if (!registry.fragments.contains(node.fragmentName.name)) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment with this name does not exists",
                                  node.fragmentName.location.source);
    };
    const auto &fragment = registry.fragments.at(node.fragmentName.name);
    bool isObjectType =
        std::holds_alternative<ObjectFragmentSpec<ObjectType>>(fragment->spec);
    bool isInterfaceType =
        std::holds_alternative<ObjectFragmentSpec<Interface>>(fragment->spec);
    if (!isObjectType && !isInterfaceType) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment has invalid type",
                                  node.fragmentName.location.source);
    };
    if (isObjectType) {
        const auto &fType =
            std::get<ObjectFragmentSpec<ObjectType>>(fragment->spec).type;
        if (fType != type) {
            throw shared::ParserError(
                node.fragmentName.location.startToken,
                std::format("Fragment has type {} while expected {}",
                            fType->name, type->name),
                node.fragmentName.location.source);
        };
    } else {
        const auto &fType =
            std::get<ObjectFragmentSpec<Interface>>(fragment->spec).type;
        if (!type->implements.contains(fType->name)) {
            throw shared::ParserError(
                node.fragmentName.location.startToken,
                std::format(
                    "Fragment has incompatible interface {} for type {}",
                    fType->name, type->name),
                node.fragmentName.location.source);
        };
    };
    return (SpreadSelection){ .fragment = fragment };
};

template <typename T>
ObjectSelection parseObjectSelectionNode(
    const client::ast::SelectionNode &sNode, const std::shared_ptr<T> &type,
    const TypeRegistry &registry) {
    return std::visit<ObjectSelection>(
        overloaded{
            [&registry, &type](const client::ast::SpreadSelectionNode &node) {
                const auto &check =
                    parseSpreadSelectionNode(node, type, registry);
                return check;
            },
            [](const client::ast::ConditionalSpreadSelectionNode &node)
                -> ObjectSelection {
                throw shared::ParserError(node.location.startToken,
                                          "Conditional spread selection is "
                                          "disallowed on object fragments",
                                          node.location.source);
            },
            [&registry, &type](const client::ast::FieldSelectionNode &node) {
                return parseFieldSelectionNode(node, type, registry);
            } },
        sNode);
};

template <typename T>
ObjectFragmentSpec<T> parseObjectFragmentSpec(
    const std::vector<client::ast::SelectionNode> &selections,
    const std::shared_ptr<T> &type, const TypeRegistry &registry) {
    const auto &selectionNodes =
        selections |
        std::views::transform([&registry, &type](const auto &sNode) {
            return parseObjectSelectionNode(sNode, type, registry);
        }) |
        std::ranges::to<std::vector>();
    return { .type = type, .selections = selectionNodes };
};

bool isObjectFieldSpecIsTypenameField(
    const client::ast::ObjectFieldSpec &spec) {
    return std::holds_alternative<client::ast::ObjectLiteralFieldSpec>(spec) &&
           std::get<client::ast::ObjectLiteralFieldSpec>(spec).name.name ==
               "__typename";
};

std::optional<std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Union>>>
getTypeForUnionConditionalSelection(
    const client::ast::ConditionalSpreadSelectionNode &node,
    const std::shared_ptr<Union> &type, const TypeRegistry &registry) {
    if (type->items.contains(node.typeName.name))
        return type->items.at(node.typeName.name);
    if (!registry.unions.contains(node.typeName.name)) return std::nullopt;
    const auto &unionNode = registry.unions.at(node.typeName.name);
    for (const auto &item : unionNode->items | std::views::keys) {
        if (!type->items.contains(item)) return std::nullopt;
    };
    return unionNode;
};

UnionFragmentSpec parseUnionFragmentSpec(
    const std::vector<client::ast::SelectionNode> &selections,
    const std::shared_ptr<Union> &type, const TypeRegistry &registry);

UnionSelection parseUnionSelectionNode(const client::ast::SelectionNode &sNode,
                                       const std::shared_ptr<Union> &type,
                                       const TypeRegistry &registry) {
    return std::visit<UnionSelection>(
        overloaded{
            [&registry, &type](const client::ast::SpreadSelectionNode &node) {
                return parseSpreadSelectionNode(node, type, registry);
            },
            [&registry, &type](const client::ast::FieldSelectionNode &node) {
                if (isObjectFieldSpecIsTypenameField(node.field))
                    return (TypenameField){};
                throw shared::ParserError(
                    node.location.startToken,
                    "No fields selections are allowed on union fragments",
                    node.location.source);
            },
            [&registry,
             &type](const client::ast::ConditionalSpreadSelectionNode &node)
                -> UnionSelection {
                const auto &itemTypeOptional =
                    getTypeForUnionConditionalSelection(node, type, registry);
                if (!itemTypeOptional.has_value()) {
                    throw shared::ParserError(
                        node.typeName.location.startToken,
                        std::format("No suitable union or type inside "
                                    "union {} with name {} was found",
                                    type->name, node.typeName.name),
                        node.typeName.location.source);
                };
                const auto &itemTypeVariant = itemTypeOptional.value();
                if (std::holds_alternative<std::shared_ptr<ObjectType>>(
                        itemTypeVariant)) {
                    const auto &itemType =
                        std::get<std::shared_ptr<ObjectType>>(itemTypeVariant);
                    const auto &spec = parseObjectFragmentSpec(
                        node.fragment->selections, itemType, registry);
                    return (UnionSelection)(ObjectConditionalSpreadSelection){
                        .type = itemType,
                        .selection =
                            std::make_shared<ObjectFragmentSpec<ObjectType>>(
                                spec)
                    };
                };
                const auto &itemType =
                    std::get<std::shared_ptr<Union>>(itemTypeVariant);
                const auto &spec = parseUnionFragmentSpec(
                    node.fragment->selections, itemType, registry);
                return (UnionSelection)(UnionConditionalSpreadSelection){
                    .type = itemType,
                    .selection = std::make_shared<UnionFragmentSpec>(spec)
                };
            } },
        sNode);
};

UnionFragmentSpec parseUnionFragmentSpec(
    const std::vector<client::ast::SelectionNode> &selections,
    const std::shared_ptr<Union> &type, const TypeRegistry &registry) {
    return { .type = type,
             .selections =
                 selections |
                 std::views::transform([&registry, &type](const auto &sNode) {
                     return parseUnionSelectionNode(sNode, type, registry);
                 }) |
                 std::ranges::to<std::vector>() };
};

FragmentSpec parseFragmentSpec(const client::ast::FragmentSpec &defSpec,
                               const FragmentSpec &spec,
                               const TypeRegistry &registry) {
    return std::visit<FragmentSpec>(
        overloaded{
            [&registry, &defSpec](
                const ObjectFragmentSpec<Interface> &node) -> FragmentSpec {
                return parseObjectFragmentSpec(defSpec.selections, node.type,
                                               registry);
            },
            [&registry, &defSpec](
                const ObjectFragmentSpec<ObjectType> &node) -> FragmentSpec {
                return parseObjectFragmentSpec(defSpec.selections, node.type,
                                               registry);
            },
            [&registry,
             &defSpec](const UnionFragmentSpec &node) -> FragmentSpec {
                return parseUnionFragmentSpec(defSpec.selections, node.type,
                                              registry);
            } },
        spec);
};

std::shared_ptr<Fragment> parseFragmentSecondPass(
    const client::ast::FragmentDefinition &definition,
    const TypeRegistry &registry) {
    const auto &fragment = registry.getFragment(definition.name.name);
    fragment->spec =
        parseFragmentSpec(definition.spec, fragment->spec, registry);
    return fragment;
};

FragmentSpec fragmentSpecFromOpType(client::ast::OpType type,
                                    const TypeRegistry &registry) {
    switch (type) {
        case client::ast::OpType::QUERY: {
            return (ObjectFragmentSpec<ObjectType>){
                .type = registry.getQueryObject()
            };
        }
        case client::ast::OpType::MUTATION: {
            return (ObjectFragmentSpec<ObjectType>){
                .type = registry.getMutationObject()
            };
        }

        case client::ast::OpType::SUBSCRIPTION: {
            return (ObjectFragmentSpec<ObjectType>){
                .type = registry.getSubscriptionObject()
            };
        }
    };
};

std::shared_ptr<Operation> parseClientOperationDefinition(
    const client::ast::OperationDefinition &definition,
    const TypeRegistry &registry) {
    const auto &fragment = fragmentSpecFromOpType(definition.type, registry);
    const auto &parameters =
        definition.parameters | std::views::values |
        std::views::transform(
            [&registry](const auto &param)
                -> std::pair<std::string, FieldDefinition<InputFieldSpec>> {
                return { param.name.name,
                         parseInputFieldDefinition(param, registry) };
            }) |
        std::ranges::to<std::map>();
    const auto &node =
        parseFragmentSpec(definition.fragment, fragment, registry);
    const auto &objectNode = std::get<ObjectFragmentSpec<ObjectType>>(node);
    const auto &sNode = std::get<FieldSelection>(objectNode.selections[0]);

    return std::make_shared<Operation>(definition.type, definition.name.name,
                                       parameters, node);
};

ClientSchemaNode parseClientDefinition(
    const client::ast::ClientDefinition &definition,
    const TypeRegistry &registry) {
    return std::visit<ClientSchemaNode>(
        overloaded{ [&registry](const client::ast::FragmentDefinition &node) {
                       return parseFragmentSecondPass(node, registry);
                   },
                    [&registry](const client::ast::OperationDefinition &node) {
                        return parseClientOperationDefinition(node, registry);
                    } },
        definition);
};

ServerSchemaNode parseServerNodeFirstPass(
    const server::ast::TypeDefinitionNode &astNode) {
    return std::visit<ServerSchemaNode>(
        overloaded{
            [](const server::ast::ScalarDefinitionNode &node) {
                return std::make_shared<Scalar>(node.name.name);
            },
            [](const server::ast::EnumDefinitionNode &node) {
                return std::make_shared<Enum>(
                    node.name.name,
                    node.values | std::views::transform([](const auto &v) {
                        return v.value.name;
                    }) | std::ranges::to<std::vector>());
            },
            [](const server::ast::UnionDefinitionNode &node) {
                return std::make_shared<Union>(node.name.name);
            },
            [](const server::ast::ObjectDefinitionNode &node) {
                return std::make_shared<ObjectType>(node.name.name);
            },
            [](const server::ast::InputObjectDefinitionNode &node) {
                return std::make_shared<InputType>(node.name.name);
            },
            [](const server::ast::InterfaceDefinitionNode &node) {
                return std::make_shared<Interface>(node.name.name);
            } },
        astNode);
};

std::vector<FieldSelection> getFieldSelectionsFromFragmentSpec(
    const FragmentSpec &spec);
std::vector<FieldSelection> getFieldSelectionsFromObjectSelection(
    const ObjectSelection &selection) {
    return std::visit<std::vector<FieldSelection>>(
        overloaded{
            [](const TypenameField &) -> std::vector<FieldSelection> {
                return {};
            },
            [](const FieldSelection &node) -> std::vector<FieldSelection> {
                std::vector<FieldSelection> selections = { node };
                if (node.selection.has_value()) {
                    const auto &nestedSelections =
                        getFieldSelectionsFromFragmentSpec(
                            *node.selection.value().get());
                    selections.resize(selections.size() +
                                      nestedSelections.size());
                    selections.insert(std::end(selections),
                                      std::begin(nestedSelections),
                                      std::end(nestedSelections));
                };
                return selections;
            },
            [](const SpreadSelection &node) -> std::vector<FieldSelection> {
                return getFieldSelectionsFromFragmentSpec(node.fragment->spec);
            } },
        selection);
};

std::vector<FieldSelection> getFieldSelectionsFromUnionSelection(
    const UnionSelection &selection) {
    return std::visit<std::vector<FieldSelection>>(
        overloaded{
            [](const TypenameField &) -> std::vector<FieldSelection> {
                return {};
            },
            [](const ObjectConditionalSpreadSelection &node)
                -> std::vector<FieldSelection> {
                return node.selection->selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromObjectSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const UnionConditionalSpreadSelection &node)
                -> std::vector<FieldSelection> {
                return node.selection->selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromUnionSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const SpreadSelection &node) -> std::vector<FieldSelection> {
                return getFieldSelectionsFromFragmentSpec(node.fragment->spec);
            } },
        selection);
};

std::vector<FieldSelection> getFieldSelectionsFromFragmentSpec(
    const FragmentSpec &spec) {
    return std::visit<std::vector<FieldSelection>>(
        overloaded{
            [](const UnionFragmentSpec &node) -> std::vector<FieldSelection> {
                return node.selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromUnionSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const ObjectFragmentSpec<ObjectType> &node)
                -> std::vector<FieldSelection> {
                return node.selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromObjectSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const ObjectFragmentSpec<Interface> &node)
                -> std::vector<FieldSelection> {
                return node.selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromObjectSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
        },
        spec);
};
void assertOperationIsValid(const std::shared_ptr<Operation> &op) {
    const auto &selections =
        getFieldSelectionsFromFragmentSpec(op->fragmentSpec);
    for (const auto &field : selections) {
        if (!field.arguments.has_value()) continue;
        const auto &arguments = field.arguments.value();
        for (const auto &arg : arguments | std::views::values) {
            if (!op->parameters.contains(arg.parameterName)) {
                throw std::runtime_error(std::format(
                    "Operation {} doesn`t define required parameter {}",
                    op->name, arg.parameterName));
            };
            const FieldDefinition<InputFieldSpec> &paramField =
                op->parameters.at(arg.parameterName);
            const FieldDefinition<InputFieldSpec> &argField = *arg.type.get();
            if (extractInputTypeSpec(paramField.spec) !=
                extractInputTypeSpec(argField.spec)) {
                throw std::runtime_error(
                    std::format("Operation {} parameter {} has wrong type",
                                op->name, arg.parameterName));
            };
            const auto &isArgNullable = argField.nullable;
            const auto &argHasDefaultValue =
                InputFieldSpec_hasDefaultValue(argField.spec);
            const auto &isParamNullable = paramField.nullable;
            const auto &paramHasDefaultValue =
                InputFieldSpec_hasDefaultValue(paramField.spec);
            if (!isArgNullable && !argHasDefaultValue && isParamNullable &&
                !paramHasDefaultValue) {
                throw std::runtime_error(
                    std::format("Operation {} parameter {} must not be "
                                "nullable or need to have default value",
                                op->name, arg.parameterName));
            };
        };
    };
};

std::vector<ast::ServerSchemaNode> parseServerNodes(
    const std::vector<server::ast::FileNodes> &astArray,
    const TypeRegistry &registry) {
    return astArray | std::views::transform([](const auto &ast) {
               return ast.definitions;
           }) |
           std::views::join | std::views::transform([&registry](auto &sNode) {
               return parseServerNode(sNode, registry);
           }) |
           std::ranges::to<std::vector>();
};

auto parseServerExtendNodes(const std::vector<server::ast::FileNodes> &astArray,
                            const TypeRegistry &registry) {
    return astArray | std::views::transform([](const auto &ast) {
               return ast.extensions;
           }) |
           std::views::join |
           std::views::transform([&registry](const auto &node) {
               return parseExtendObjectType(node, registry);
           });
};

void assertClientNodesAreValid(
    const std::vector<ast::ClientSchemaNode> &clientNodes) {
    for (const auto &opNode :
         clientNodes | std::views::filter([](const auto &n) {
             return std::holds_alternative<std::shared_ptr<Operation>>(n);
         }) | std::views::transform([](const auto &n) {
             return std::get<std::shared_ptr<Operation>>(n);
         })) {
        assertOperationIsValid(opNode);
    };
};

std::vector<ast::ClientSchemaNode> parseClientNodes(
    const std::vector<client::ast::ClientDefinition> &definitions,
    const TypeRegistry &registry) {
    return definitions | std::views::transform([&registry](const auto &node) {
               return parseClientDefinition(node, registry);
           }) |
           std::ranges::to<std::vector>();
};

const Schema parsers::schema::parseSchema(
    std::vector<server::ast::FileNodes> astArray,
    std::vector<client::ast::ClientDefinition> clientDefinitions) {
    TypeRegistry registry;
    for (const auto &ast : astArray) {
        for (const auto &node : ast.definitions) {
            registry.addNode(parseServerNodeFirstPass(node));
        };
    };
    const auto &serverNodes = parseServerNodes(astArray, registry);
    for (const auto &[typeNode, newFields] :
         parseServerExtendNodes(astArray, registry)) {
        registry.patchObject(typeNode, newFields);
    };
    for (const auto &fragmentDefinition :
         clientDefinitions | std::views::filter([](const auto &def) {
             return std::holds_alternative<client::ast::FragmentDefinition>(
                 def);
         }) | std::views::transform([](const auto &def) {
             return std::get<client::ast::FragmentDefinition>(def);
         })) {
        registry.addFragment(
            parseFragmentFirstPass(fragmentDefinition, registry));
    };
    const auto &clientNodes = parseClientNodes(clientDefinitions, registry);
    assertClientNodesAreValid(clientNodes);
    return { .server = ServerSchema::fromNodes(serverNodes),
             .client = ClientSchema::fromNodes(clientNodes) };
};
