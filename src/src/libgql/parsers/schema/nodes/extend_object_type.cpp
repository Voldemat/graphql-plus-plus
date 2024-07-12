#include "./extend_object_type.hpp"

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <utility>

#include "../../file/server/ast.hpp"
#include "../../file/shared/parser_error.hpp"
#include "../server_ast.hpp"
#include "../type_registry.hpp"
#include "./object_field_spec.hpp"

using namespace parsers::file;

namespace parsers::schema::nodes {
std::pair<std::shared_ptr<ast::ObjectType>,
          std::map<std::string,
                   std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>>>>
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
                    -> std::pair<std::string,
                                 std::shared_ptr<ast::FieldDefinition<
                                     ast::ObjectFieldSpec>>> {
                    const auto &[typeSpec, nullable] =
                        nodes::parseObjectFieldSpec(field, registry);

                    return { field.name.name,
                             std::make_shared<
                                 ast::FieldDefinition<ast::ObjectFieldSpec>>(
                                 field.name.name, typeSpec, nullable) };
                }) |
            std::ranges::to<std::map>()
    };
};
};  // namespace parsers::schema::nodes
