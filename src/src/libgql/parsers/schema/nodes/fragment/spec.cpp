#include "./spec.hpp"

#include <memory>
#include <variant>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/nodes/fragment/object_fragment_spec.hpp"
#include "libgql/parsers/schema/nodes/fragment/union_fragment_spec.hpp"
#include "libgql/parsers/schema/nodes/object_field_spec.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

namespace parsers::schema::nodes {
ast::FragmentSpec fragmentSpecFromFieldDefinition(
    const ast::FieldDefinition<ast::ObjectFieldSpec> &field,
    const file::client::ast::FragmentSpec &sNode) {
    const ast::ObjectTypeSpec &typeSpec =
        getReturnTypeFromObjectFieldSpec(field.spec);
    if (std::holds_alternative<std::shared_ptr<ast::ObjectType>>(typeSpec)) {
        const auto &type = std::get<std::shared_ptr<ast::ObjectType>>(typeSpec);
        return (ast::ObjectFragmentSpec<ast::ObjectType>){ .type = type };
    } else if (std::holds_alternative<std::shared_ptr<ast::Interface>>(
                   typeSpec)) {
        const auto &type = std::get<std::shared_ptr<ast::Interface>>(typeSpec);
        return (ast::ObjectFragmentSpec<ast::Interface>){ .type = type };
    } else if (std::holds_alternative<std::shared_ptr<ast::Union>>(typeSpec)) {
        const auto &type = std::get<std::shared_ptr<ast::Union>>(typeSpec);
        return (ast::UnionFragmentSpec){ .type = type };
    };
    throw file::shared::ParserError(
        sNode.location.startToken,
        "Cannot have selection on literal type field", sNode.location.source);
};

ast::FragmentSpec parseFragmentSpec(
    const file::client::ast::FragmentSpec &defSpec,
    const ast::FragmentSpec &spec, const TypeRegistry &registry) {
    return std::visit<ast::FragmentSpec>(
        overloaded{ [&registry, &defSpec](
                        const ast::ObjectFragmentSpec<ast::Interface> &node)
                        -> ast::FragmentSpec {
                       return nodes::parseObjectFragmentSpec(
                           defSpec.selections, node.type, registry);
                   },
                    [&registry, &defSpec](
                        const ast::ObjectFragmentSpec<ast::ObjectType> &node)
                        -> ast::FragmentSpec {
                        return nodes::parseObjectFragmentSpec(
                            defSpec.selections, node.type, registry);
                    },
                    [&registry, &defSpec](const ast::UnionFragmentSpec &node)
                        -> ast::FragmentSpec {
                        return nodes::parseUnionFragmentSpec(
                            defSpec.selections, node.type, registry);
                    } },
        spec);
};
};  // namespace parsers::schema::nodes
