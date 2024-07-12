#include "./client_node.hpp"

#include <format>
#include <memory>
#include <ranges>
#include <stdexcept>
#include <variant>
#include <vector>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/nodes/client_operation.hpp"
#include "libgql/parsers/schema/nodes/fragment/field_selection_node.hpp"
#include "libgql/parsers/schema/nodes/fragment/fragment.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
ast::ClientSchemaNode parseClientDefinition(
    const client::ast::ClientDefinition &definition,
    const TypeRegistry &registry) {
    return std::visit<ast::ClientSchemaNode>(
        overloaded{ [&registry](const client::ast::FragmentDefinition &node) {
                       return nodes::parseFragmentSecondPass(node, registry);
                   },
                    [&registry](const client::ast::OperationDefinition &node) {
                        return nodes::parseClientOperationDefinition(node,
                                                                     registry);
                    } },
        definition);
};

void assertOperationArgumentIsValid(const ast::FieldSelectionArgument &arg,
                                    const std::shared_ptr<ast::Operation> &op) {
    if (!op->parameters.contains(arg.parameterName)) {
        throw std::runtime_error(
            std::format("Operation {} doesn`t define required parameter {}",
                        op->name, arg.parameterName));
    };
    const ast::FieldDefinition<ast::InputFieldSpec> &paramField =
        op->parameters.at(arg.parameterName);
    const ast::FieldDefinition<ast::InputFieldSpec> &argField = *arg.type;
    if (extractInputTypeSpec(paramField.spec) !=
        extractInputTypeSpec(argField.spec)) {
        throw std::runtime_error(
            std::format("Operation {} parameter {} has wrong type", op->name,
                        arg.parameterName));
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

void assertOperationIsValid(const std::shared_ptr<ast::Operation> &op) {
    const auto &selections =
        nodes::getFieldSelectionsFromFragmentSpec(op->fragmentSpec);
    for (const auto &field :
         selections | std::views::filter([](const auto &field) {
             return field.arguments.has_value();
         })) {
        for (const auto &arg : field.arguments.value() | std::views::values) {
            assertOperationArgumentIsValid(arg, op);
        };
    };
};

void assertClientNodesAreValid(
    const std::vector<ast::ClientSchemaNode> &clientNodes) {
    for (const auto &opNode :
         clientNodes | std::views::filter([](const auto &n) {
             return std::holds_alternative<std::shared_ptr<ast::Operation>>(n);
         }) | std::views::transform([](const auto &n) {
             return std::get<std::shared_ptr<ast::Operation>>(n);
         })) {
        assertOperationIsValid(opNode);
    };
};

std::vector<ast::ClientSchemaNode> parseClientNodes(
    const std::vector<client::ast::ClientDefinition> &definitions,
    const TypeRegistry &registry) {
    const auto &nodes = definitions |
                        std::views::transform([&registry](const auto &node) {
                            return nodes::parseClientDefinition(node, registry);
                        }) |
                        std::ranges::to<std::vector>();
    assertClientNodesAreValid(nodes);
    return nodes;
};
};  // namespace parsers::schema::nodes
