#include "./fragment.hpp"

#include <memory>

#include "../../../file/client/ast.hpp"
#include "../../../file/shared/ast.hpp"
#include "../../../file/shared/parser_error.hpp"
#include "../../../file/shared/shared.hpp"
#include "../../client_ast.hpp"
#include "../../server_ast.hpp"
#include "../../type_registry.hpp"
#include "./spec.hpp"

using namespace gql::parsers::file;

namespace gql::parsers::schema::nodes {
ast::FragmentSpec fragmentSpecFromName(const shared::ast::NameNode &typeName,
                                       const TypeRegistry &registry) {
    if (registry.objects.contains(typeName.name)) {
        return (ast::ObjectFragmentSpec<ast::ObjectType>){
            .type = registry.getObject(typeName.name)
        };
    } else if (registry.unions.contains(typeName.name)) {
        return (ast::UnionFragmentSpec){ .type = registry.unions.at(
                                             typeName.name) };
    } else if (registry.interfaces.contains(typeName.name)) {
        return (ast::ObjectFragmentSpec<ast::Interface>){
            .type = registry.getInterface(typeName.name)
        };
    };
    throw shared::ParserError(
        typeName.location.startToken,
        "Object type or union with this name does not exists",
        typeName.location.source);
};

std::shared_ptr<ast::Fragment> parseFragmentFirstPass(
    const client::ast::FragmentDefinition &definition,
    const TypeRegistry &registry) {
    const auto &name = definition.name.name;
    if (registry.fragments.contains(name)) {
        throw shared::ParserError(definition.name.location.startToken,
                                  "Fragment with this name already exists",
                                  definition.name.location.source);
    };
    return std::make_shared<ast::Fragment>(
        name, fragmentSpecFromName(definition.typeName, registry));
};

std::shared_ptr<ast::Fragment> parseFragmentSecondPass(
    const client::ast::FragmentDefinition &definition,
    const TypeRegistry &registry) {
    const auto &fragment = registry.getFragment(definition.name.name);
    fragment->sourceText =
        shared::getSourceText(definition.location.source->buffer,
                              definition.location.startToken.location,
                              definition.location.endToken.location);
    fragment->spec =
        nodes::parseFragmentSpec(definition.spec, fragment->spec, registry);
    return fragment;
};
};  // namespace gql::parsers::schema::nodes
