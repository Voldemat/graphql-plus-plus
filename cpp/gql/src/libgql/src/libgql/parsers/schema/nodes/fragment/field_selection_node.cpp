
#include "./field_selection_node.hpp"

#include <ranges>
#include <variant>
#include <vector>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "utils.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
bool isObjectFieldSpecIsTypenameField(
    const client::ast::ObjectFieldSpec &spec) {
    return std::holds_alternative<client::ast::ObjectLiteralFieldSpec>(spec) &&
           std::get<client::ast::ObjectLiteralFieldSpec>(spec).name.name ==
               "__typename";
};

std::vector<ast::FieldSelection> getFieldSelectionsFromFragmentSpec(
    const ast::FragmentSpec &spec);
std::vector<ast::FieldSelection> getFieldSelectionsFromObjectSelection(
    const ast::ObjectSelection &selection) {
    return std::visit<std::vector<ast::FieldSelection>>(
        overloaded{
            [](const ast::TypenameField &) -> std::vector<ast::FieldSelection> {
                return {};
            },
            [](const ast::FieldSelection &node)
                -> std::vector<ast::FieldSelection> {
                std::vector<ast::FieldSelection> selections = { node };
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
            [](const ast::SpreadSelection &node)
                -> std::vector<ast::FieldSelection> {
                return getFieldSelectionsFromFragmentSpec(node.fragment->spec);
            } },
        selection);
};

std::vector<ast::FieldSelection> getFieldSelectionsFromUnionSelection(
    const ast::UnionSelection &selection) {
    return std::visit<std::vector<ast::FieldSelection>>(
        overloaded{
            [](const ast::TypenameField &) -> std::vector<ast::FieldSelection> {
                return {};
            },
            [](const ast::ObjectConditionalSpreadSelection &node)
                -> std::vector<ast::FieldSelection> {
                return node.selection->selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromObjectSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const ast::UnionConditionalSpreadSelection &node)
                -> std::vector<ast::FieldSelection> {
                return node.selection->selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromUnionSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const ast::SpreadSelection &node)
                -> std::vector<ast::FieldSelection> {
                return getFieldSelectionsFromFragmentSpec(node.fragment->spec);
            } },
        selection);
};

std::vector<ast::FieldSelection> getFieldSelectionsFromFragmentSpec(
    const ast::FragmentSpec &spec) {
    return std::visit<std::vector<ast::FieldSelection>>(
        overloaded{
            [](const ast::UnionFragmentSpec &node)
                -> std::vector<ast::FieldSelection> {
                return node.selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromUnionSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const ast::ObjectFragmentSpec<ast::ObjectType> &node)
                -> std::vector<ast::FieldSelection> {
                return node.selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromObjectSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
            [](const ast::ObjectFragmentSpec<ast::Interface> &node)
                -> std::vector<ast::FieldSelection> {
                return node.selections |
                       std::views::transform([](const auto &sNode) {
                           return getFieldSelectionsFromObjectSelection(sNode);
                       }) |
                       std::views::join | std::ranges::to<std::vector>();
            },
        },
        spec);
};
};  // namespace parsers::schema::nodes
