#include "./server_directive.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <utility>
#include <vector>

#include "../nodes/input_field_definition.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

namespace gql::parsers::schema::nodes {
std::shared_ptr<ast::ServerDirective> parseServerDirective(
    const file::server::ast::DirectiveDefinitionNode &astNode,
    const TypeRegistry &registry) {
    return std::make_shared<ast::ServerDirective>((ast::ServerDirective){
        .name = astNode.name.name,
        .arguments =
            astNode.arguments |
            std::views::transform(
                [&registry](
                    const file::shared::ast::InputValueDefinitionNode &node) {
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
                [](const file::server::ast::DirectiveLocationNode &node) {
                    return node.directiveLocation;
                }) |
            std::ranges::to<std::vector>(),
    });
};

}  // namespace gql::parsers::schema::nodes
