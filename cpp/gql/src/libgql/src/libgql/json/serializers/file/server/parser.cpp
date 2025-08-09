#include "./parser.hpp"

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <magic_enum.hpp>
#include <variant>
#include <vector>

#include "../shared/shared.hpp"
#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "utils.hpp"

using namespace gql::parsers::file;
using namespace gql::parsers::file::server;

namespace gql::json::serializers::file::server {

void writeEnumValueDefinitionNode(JSONWriter &writer,
                                  const ast::EnumValueDefinitionNode &node) {
    writer.StartObject();
    writer.String("value");
    shared::writeNameNode(writer, node.value);
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeDirectiveLocationNode(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const ast::DirectiveLocationNode &node) {
    writer.StartObject();
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.String("directiveLocation");
    writer.String(magic_enum::enum_name(node.directiveLocation).data());
    writer.EndObject();
};

void writeFieldDefinitionNode(JSONWriter &writer,
                              const ast::FieldDefinitionNode &node) {
    writer.StartObject();
    writer.String("name");
    shared::writeNameNode(writer, node.name);
    writer.String("type");
    shared::writeTypeNode(writer, node.type);
    writer.String("arguments");
    shared::writeInputValueDefinitionNodes(writer, node.arguments);
    writer.String("directives");
    writer.StartObject();
    for (const auto &directive : node.directives) {
        writer.String(directive.name.name.c_str());
        writer.StartObject();
        writer.String("arguments");
        shared::writeArguments(writer, directive.arguments);
        writer.EndObject();
    };
    writer.EndObject();
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeDefinitionNode(JSONWriter &writer,
                         const ast::TypeDefinitionNode &node) {
    writer.StartObject();
    std::visit(utils::overloaded{
                   [&writer](const ast::ScalarDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("ScalarDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::EnumDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("EnumDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("values");
                       writer.StartArray();
                       for (const auto &v : node.values) {
                           writeEnumValueDefinitionNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::UnionDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("UnionDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("values");
                       writer.StartArray();
                       for (const auto &v : node.values) {
                           shared::writeNameNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::ObjectDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("ObjectDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("fields");
                       writer.StartArray();
                       for (const auto &v : node.fields) {
                           writeFieldDefinitionNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("interfaces");
                       writer.StartArray();
                       for (const auto &v : node.interfaces) {
                           shared::writeNameNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::InputObjectDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("InputObjectDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("fields");
                       writer.StartArray();
                       for (const auto &v : node.fields) {
                           writeFieldDefinitionNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::DirectiveDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("DirectiveDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("arguments");
                       shared::writeInputValueDefinitionNodes(writer,
                                                              node.arguments);
                       writer.String("targets");
                       writer.StartArray();
                       for (const auto &v : node.targets) {
                           writeDirectiveLocationNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::InterfaceDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("InterfaceDefinitionNode");
                       writer.String("name");
                       shared::writeNameNode(writer, node.name);
                       writer.String("fields");
                       writer.StartArray();
                       for (const auto &v : node.fields) {
                           writeFieldDefinitionNode(writer, v);
                       };
                       writer.EndArray();
                       writer.String("location");
                       shared::writeNodeLocation(writer, node.location);
                   } },
               node);
    writer.EndObject();
};

void writeExtensionNode(JSONWriter &writer, const ast::ExtendTypeNode &node) {
    writer.StartObject();
    writer.String("type");
    writeDefinitionNode(writer, node.typeNode);
    writer.EndObject();
};

void writeNodes(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                const std::vector<ast::ASTNode> &nodes) {
    writer.StartArray();
    for (const auto &astNode : nodes) {
        std::visit(utils::overloaded{
                       [&writer](const ast::TypeDefinitionNode &node) {
                           writeDefinitionNode(writer, node);
                       },
                       [&writer](const ast::ExtendTypeNode &node) {
                           writeExtensionNode(writer, node);
                       },
                   },
                   astNode);
    };
    writer.EndArray();
};
};  // namespace gql::json::serializers::file::server
