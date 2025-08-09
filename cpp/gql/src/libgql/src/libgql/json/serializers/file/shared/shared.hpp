#pragma once

#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

namespace gql::json::serializers::shared {
void writeNodeLocation(
    JSONWriter &writer,
    const ::gql::parsers::file::shared::ast::NodeLocation &location);

void writeNameNode(JSONWriter &writer,
                   const ::gql::parsers::file::shared::ast::NameNode &node);

void writeLiteralNode(
    JSONWriter &writer,
    const ::gql::parsers::file::shared::ast::LiteralNode &literalNode);

void writeNamedTypeNodeContent(
    JSONWriter &writer,
    const ::gql::parsers::file::shared::ast::NamedTypeNode &node);

void writeTypeNode(JSONWriter &writer,
                   const ::gql::parsers::file::shared::ast::TypeNode &typeNode);

void writeInputValueDefinitionNode(
    JSONWriter &writer,
    const ::gql::parsers::file::shared::ast::InputValueDefinitionNode &node);

void writeArgumentValue(
    JSONWriter &writer,
    const ::gql::parsers::file::shared::ast::ArgumentValue &value);
void writeArguments(
    JSONWriter &writer,
    const std::vector<::gql::parsers::file::shared::ast::Argument> &arguments);

void writeInputValueDefinitionNodes(
    JSONWriter &writer,
    const std::vector<
        ::gql::parsers::file::shared::ast::InputValueDefinitionNode> &nodes);
};  // namespace gql::json::serializers::shared
