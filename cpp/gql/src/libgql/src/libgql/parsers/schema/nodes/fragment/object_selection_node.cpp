#include "./object_selection_node.hpp"

#include <format>
#include <memory>
#include <variant>

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {

ast::SpreadSelection parseSpreadSelectionNode(
    const client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ast::Interface> &type, const TypeRegistry &registry) {
    if (!registry.fragments.contains(node.fragmentName.name)) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment with this name does not exists",
                                  node.fragmentName.location.source);
    };
    const auto &fragment = registry.fragments.at(node.fragmentName.name);
    if (!std::holds_alternative<ast::ObjectFragmentSpec<ast::Interface>>(
            fragment->spec) ||
        std::get<ast::ObjectFragmentSpec<ast::Interface>>(fragment->spec)
                .type != type) {
        throw shared::ParserError(
            node.fragmentName.location.startToken,
            std::format("Fragment has type {} while expected {}",
                        fragment->name, type->name),
            node.fragmentName.location.source);
    };
    return (ast::SpreadSelection){ .fragment = fragment };
};

ast::SpreadSelection parseSpreadSelectionNode(
    const client::ast::SpreadSelectionNode &node,
    const std::shared_ptr<ast::ObjectType> &type,
    const TypeRegistry &registry) {
    if (!registry.fragments.contains(node.fragmentName.name)) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment with this name does not exists",
                                  node.fragmentName.location.source);
    };
    const auto &fragment = registry.fragments.at(node.fragmentName.name);
    bool isObjectType =
        std::holds_alternative<ast::ObjectFragmentSpec<ast::ObjectType>>(
            fragment->spec);
    bool isInterfaceType =
        std::holds_alternative<ast::ObjectFragmentSpec<ast::Interface>>(
            fragment->spec);
    if (!isObjectType && !isInterfaceType) {
        throw shared::ParserError(node.fragmentName.location.startToken,
                                  "Fragment has invalid type",
                                  node.fragmentName.location.source);
    };
    if (isObjectType) {
        const auto &fType =
            std::get<ast::ObjectFragmentSpec<ast::ObjectType>>(fragment->spec)
                .type;
        if (fType != type) {
            throw shared::ParserError(
                node.fragmentName.location.startToken,
                std::format("Fragment has type {} while expected {}",
                            fType->name, type->name),
                node.fragmentName.location.source);
        };
    } else {
        const auto &fType =
            std::get<ast::ObjectFragmentSpec<ast::Interface>>(fragment->spec)
                .type;
        if (!type->implements.contains(fType->name)) {
            throw shared::ParserError(
                node.fragmentName.location.startToken,
                std::format(
                    "Fragment has incompatible interface {} for type {}",
                    fType->name, type->name),
                node.fragmentName.location.source);
        };
    };
    return (ast::SpreadSelection){ .fragment = fragment };
};
};  // namespace gql::parsers::schema::nodes
