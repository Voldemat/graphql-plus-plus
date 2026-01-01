#include "./shared.hpp"

#include <variant>
#include <vector>

#include "libgql/json/serializers/lexer/lexer.hpp"
#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "utils.hpp"

using namespace gql::parsers::file::shared;

namespace gql::json::serializers::shared {

void writeNodeLocation(JSONWriter &writer, const ast::NodeLocation &location) {
    writer.StartObject();
    writer.String("startToken");
    lexer::writeToken(writer, location.startToken);
    writer.String("endToken");
    lexer::writeToken(writer, location.endToken);
    writer.EndObject();
};

void writeNameNode(JSONWriter &writer, const ast::NameNode &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("location");
    writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeLiteralNode(JSONWriter &writer, const ast::LiteralNode &literalNode) {
    writer.StartObject();
    std::visit(utils::overloaded{
                   [&writer](const ast::LiteralIntNode &node) {
                       writer.String("value");
                       writer.Int(node.value);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::LiteralFloatNode &node) {
                       writer.String("value");
                       writer.Double(node.value);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::LiteralStringNode &node) {
                       writer.String("value");
                       writer.String(node.value.c_str());
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::LiteralBooleanNode &node) {
                       writer.String("value");
                       writer.Bool(node.value);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const ast::LiteralEnumValueNode &node) {
                       writer.String("value");
                       writer.String(node.value.c_str());
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
               },
               literalNode);
    writer.EndObject();
};

void writeNamedTypeNodeContent(JSONWriter &writer,
                               const ast::NamedTypeNode &node) {
    writer.String("_type");
    writer.String("NamedTypeNode");
    writer.String("name");
    writeNameNode(writer, node.name);
    writer.String("nullable");
    writer.Bool(node.nullable);
    writer.String("location");
    writeNodeLocation(writer, node.location);
};

void writeTypeNode(JSONWriter &writer, const ast::TypeNode &typeNode) {
    writer.StartObject();
    std::visit(utils::overloaded{
                   [&writer](const ast::NamedTypeNode &node) -> void {
                       writeNamedTypeNodeContent(writer, node);
                   },
                   [&writer](const ast::ListTypeNode &node) -> void {
                       writer.String("_type");
                       writer.String("ListTypeNode");
                       writer.String("type");
                       writer.StartObject();
                       writeNamedTypeNodeContent(writer, node.type);
                       writer.EndObject();
                       writer.String("nullable");
                       writer.Bool(node.nullable);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
               },
               typeNode);
    writer.EndObject();
};

void writeInputFieldDefinitionNode(JSONWriter &writer,
                                   const ast::InputFieldDefinitionNode &node) {
    writer.StartObject();
    writer.String("name");
    writeNameNode(writer, node.name);
    writer.String("type");
    writeTypeNode(writer, node.type);
    writer.String("defaultValue");
    if (node.defaultValue.has_value()) {
        writeLiteralNode(writer, node.defaultValue.value());
    } else {
        writer.Null();
    };
    writer.String("location");
    writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeArgumentValue(JSONWriter &writer, const ast::ArgumentValue &value) {
    std::visit(utils::overloaded{ [&writer](const ast::NameNode &node) {
                                     writer.String(node.name.c_str());
                                 },
                                  [&writer](const ast::LiteralNode &node) {
                                      writeLiteralNode(writer, node);
                                  } },
               value);
};

void writeInputFieldDefinitionNodes(
    JSONWriter &writer,
    const std::vector<ast::InputFieldDefinitionNode> &nodes) {
    writer.StartArray();
    for (const auto &arg : nodes) {
        shared::writeInputFieldDefinitionNode(writer, arg);
    };
    writer.EndArray();
};

void writeArguments(JSONWriter &writer,
                    const std::vector<ast::Argument> &arguments) {
    writer.StartObject();
    for (const auto &arg : arguments) {
        writer.String(arg.name.name.c_str());
        shared::writeArgumentValue(writer, arg.value);
    };
    writer.EndObject();
};

};  // namespace gql::json::serializers::shared
