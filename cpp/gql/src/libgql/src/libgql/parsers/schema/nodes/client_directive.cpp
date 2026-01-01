#include "./client_directive.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <utility>
#include <vector>

#include "../client_ast.hpp"
#include "../type_registry.hpp"
#include "./input_field_definition.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"

namespace gql::parsers::schema::nodes {
std::shared_ptr<ast::ClientDirective> parseClientDirective(
    const file::client::ast::DirectiveDefinition &astNode,
    const TypeRegistry &registry) {
    return std::make_shared<ast::ClientDirective>((ast::ClientDirective){
        .name = astNode.name.name,
        .arguments =
            astNode.arguments |
            std::views::transform(
                [&registry](
                    const file::shared::ast::InputFieldDefinitionNode &node) {
                    return std::make_pair(
                        node.name.name,
                        std::make_shared<
                            ast::FieldDefinition<ast::InputFieldSpec>>(
                            nodes::parseInputFieldDefinition(node, registry)));
                }) |
            std::ranges::to<std::map>(),
        .locations =
            astNode.targets |
            std::views::transform(
                [](const file::client::ast::DirectiveLocationNode &node) {
                    return node.directiveLocation;
                }) |
            std::ranges::to<std::vector>(),
    });
};

}  // namespace gql::parsers::schema::nodes
