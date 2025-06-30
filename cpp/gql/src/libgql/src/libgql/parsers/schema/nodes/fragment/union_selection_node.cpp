
#include <format>
#include <memory>
#include <optional>
#include <ranges>
#include <variant>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/nodes/fragment/field_selection_node.hpp"
#include "libgql/parsers/schema/nodes/fragment/object_fragment_spec.hpp"
#include "libgql/parsers/schema/nodes/fragment/union_fragment_spec.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
ast::SpreadSelection parseSpreadSelectionNode(
    const client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry) {
    if (!registry.fragments.contains(node.fragmentName.name)) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment with this name does not exists",
                                  node.fragmentName.location.source);
    };
    const auto &fragment = registry.fragments.at(node.fragmentName.name);
    const bool isUnion =
        std::holds_alternative<ast::UnionFragmentSpec>(fragment->spec);
    const bool isInterface =
        std::holds_alternative<ast::ObjectFragmentSpec<ast::Interface>>(
            fragment->spec);
    if (!isUnion && !isInterface) {
        const auto &objectSpec =
            std::get<ast::ObjectFragmentSpec<ast::ObjectType>>(fragment->spec);
        throw shared::ParserError(
            node.fragmentName.location.startToken,
            std::format(
                "Fragment has object type, while union or interface was "
                "expected. Fragment type: {}, Expected type: {}",
                objectSpec.type->name, type->name),
            node.fragmentName.location.source);
    };
    if (isUnion) {
        const auto &unionFragment =
            std::get<ast::UnionFragmentSpec>(fragment->spec);
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
            std::get<ast::ObjectFragmentSpec<ast::Interface>>(fragment->spec);
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
    return (ast::SpreadSelection){ .fragment = fragment };
};

std::optional<
    std::variant<std::shared_ptr<ast::ObjectType>, std::shared_ptr<ast::Union>>>
getTypeForUnionConditionalSelection(
    const client::ast::ConditionalSpreadSelectionNode &node,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry) {
    if (type->items.contains(node.typeName.name))
        return type->items.at(node.typeName.name);
    if (!registry.unions.contains(node.typeName.name)) return std::nullopt;
    const auto &unionNode = registry.unions.at(node.typeName.name);
    for (const auto &item : unionNode->items | std::views::keys) {
        if (!type->items.contains(item)) return std::nullopt;
    };
    return unionNode;
};

ast::UnionSelection parseUnionSelectionNode(
    const client::ast::SelectionNode &sNode,
    const std::shared_ptr<ast::Union> &type, const TypeRegistry &registry) {
    return std::visit<ast::UnionSelection>(
        overloaded{
            [&registry, &type](const client::ast::SpreadSelectionNode &node) {
                return nodes::parseSpreadSelectionNode(node, type, registry);
            },
            [&registry, &type](const client::ast::FieldSelectionNode &node) {
                if (isObjectFieldSpecIsTypenameField(node.field))
                    return (ast::TypenameField){
                        .alias =
                            file::client::ast::extractSelectionName(node.field)
                    };
                throw shared::ParserError(
                    node.location.startToken,
                    "No fields selections are allowed on union fragments",
                    node.location.source);
            },
            [&registry,
             &type](const client::ast::ConditionalSpreadSelectionNode &node)
                -> ast::UnionSelection {
                const auto &itemTypeOptional =
                    nodes::getTypeForUnionConditionalSelection(node, type,
                                                               registry);
                if (!itemTypeOptional.has_value()) {
                    throw shared::ParserError(
                        node.typeName.location.startToken,
                        std::format("No suitable union or type inside "
                                    "union {} with name {} was found",
                                    type->name, node.typeName.name),
                        node.typeName.location.source);
                };
                const auto &itemTypeVariant = itemTypeOptional.value();
                if (std::holds_alternative<std::shared_ptr<ast::ObjectType>>(
                        itemTypeVariant)) {
                    const auto &itemType =
                        std::get<std::shared_ptr<ast::ObjectType>>(
                            itemTypeVariant);
                    const auto &spec = nodes::parseObjectFragmentSpec(
                        node.fragment->selections, itemType, registry);
                    return (ast::UnionSelection)(
                        ast::ObjectConditionalSpreadSelection){
                        .type = itemType,
                        .selection = std::make_shared<
                            ast::ObjectFragmentSpec<ast::ObjectType>>(spec)
                    };
                };
                const auto &itemType =
                    std::get<std::shared_ptr<ast::Union>>(itemTypeVariant);
                const auto &spec = nodes::parseUnionFragmentSpec(
                    node.fragment->selections, itemType, registry);
                return (ast::UnionSelection)(
                    ast::UnionConditionalSpreadSelection){
                    .type = itemType,
                    .selection = std::make_shared<ast::UnionFragmentSpec>(spec)
                };
            } },
        sNode);
};
};  // namespace parsers::schema::nodes
