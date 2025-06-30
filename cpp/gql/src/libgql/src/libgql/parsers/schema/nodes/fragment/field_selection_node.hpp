
#pragma once

#include <cassert>
#include <format>
#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "../../../file/client/ast.hpp"
#include "../../../file/shared/parser_error.hpp"
#include "../../client_ast.hpp"
#include "libgql/parsers/schema/nodes/fragment/selection_argument.hpp"
#include "libgql/parsers/schema/nodes/fragment/spec.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

namespace parsers::schema::nodes {
bool isObjectFieldSpecIsTypenameField(
    const file::client::ast::ObjectFieldSpec &spec);
template <typename T>
ast::FieldSelection parseFieldSelectionNode(
    const file::client::ast::FieldSelectionNode &fNode,
    const std::shared_ptr<T> &type, const TypeRegistry &registry) {
    return std::visit<ast::FieldSelection>(
        overloaded{
            [&type, &fNode,
             &registry](const file::client::ast::ObjectLiteralFieldSpec &node)
                -> ast::FieldSelection {
                if (!(type->fields.contains(node.name.name) ||
                      node.name.name == "__typename")) {
                    throw file::shared::ParserError(
                        node.name.location.startToken,
                        std::format("Unknown field {} on type {}",
                                    node.name.name, type->name),
                        node.name.location.source);
                };
                const auto &spec = fNode.spec.transform(
                    [&registry, &node, &type](
                        const std::shared_ptr<file::client::ast::FragmentSpec>
                            &sNode) {
                        return std::make_shared<ast::FragmentSpec>(
                            parseFragmentSpec(
                                *sNode,
                                nodes::fragmentSpecFromFieldDefinition(
                                    *type->fields.at(node.name.name), *sNode),
                                registry));
                    });
                return { .name = node.name.name,
                         .alias = node.selectionName.name,
                         .selection = spec };
            },
            [&type, &registry,
             &fNode](const file::client::ast::ObjectCallableFieldSpec &node)
                -> ast::FieldSelection {
                if (!type->fields.contains(node.name.name)) {
                    throw file::shared::ParserError(
                        node.name.location.startToken,
                        std::format("Unknown field {} on type {}",
                                    node.name.name, type->name),
                        node.name.location.source);
                };
                const std::shared_ptr<
                    ast::FieldDefinition<ast::ObjectFieldSpec>> &fType =
                    type->fields.at(node.name.name);
                if (!std::holds_alternative<ast::CallableFieldSpec>(
                        fType->spec)) {
                    throw file::shared::ParserError(
                        node.location.startToken,
                        "Callable selection is forbidden on uncallable fields",
                        node.location.source);
                };
                const auto &spec =
                    std::get<ast::CallableFieldSpec>(fType->spec);
                const auto &arguments =
                    node.arguments |
                    std::views::transform(
                        [&spec](const auto &arg)
                            -> std::pair<std::string,
                                         ast::FieldSelectionArgument> {
                            return { arg.name.name,
                                     nodes::parseSelectionArgument(arg, spec) };
                        }) |
                    std::ranges::to<std::map>();
                for (const auto &[argName, argSpec] : spec.arguments) {
                    if (!argSpec->nullable &&
                        !ast::InputFieldSpec_hasDefaultValue(argSpec->spec) &&
                        !arguments.contains(argName)) {
                        assert(node.location.source != nullptr);
                        throw file::shared::ParserError(
                            node.name.location.startToken,
                            std::format("Required argument {} was not provided",
                                        argName),
                            node.name.location.source);
                    }
                }
                return { .name = node.name.name,
                         .alias = node.selectionName.name,
                         .arguments = arguments,
                         .selection = fNode.spec.transform(
                             [&registry, &fType](const auto &selectionNode) {
                                 return std::make_shared<ast::FragmentSpec>(
                                     parseFragmentSpec(
                                         *selectionNode,
                                         nodes::fragmentSpecFromFieldDefinition(
                                             *fType, *selectionNode),
                                         registry));
                             }) };
            } },
        fNode.field);
};

std::vector<ast::FieldSelection> getFieldSelectionsFromObjectSelection(
    const ast::ObjectSelection &selection);

std::vector<ast::FieldSelection> getFieldSelectionsFromUnionSelection(
    const ast::UnionSelection &selection);

std::vector<ast::FieldSelection> getFieldSelectionsFromFragmentSpec(
    const ast::FragmentSpec &spec);
};  // namespace parsers::schema::nodes
