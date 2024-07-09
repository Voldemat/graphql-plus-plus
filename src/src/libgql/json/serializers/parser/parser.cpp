#include "./parser.hpp"

#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <variant>

#include "libgql/json/serializers/lexer/lexer.hpp"
#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "utils.hpp"

using namespace parsers::file;
using namespace parsers::file::server;

void writeNodeLocation(JSONWriter &writer,
                       const shared::ast::NodeLocation &location) {
    writer.StartObject();
    writer.String("startToken");
    json::serializers::lexer::writeToken(writer, location.startToken);
    writer.String("endToken");
    json::serializers::lexer::writeToken(writer, location.endToken);
    writer.EndObject();
};

void writeNameNode(JSONWriter &writer, const shared::ast::NameNode &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("location");
    writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeEnumValueDefinitionNode(JSONWriter &writer,
                                  const ast::EnumValueDefinitionNode &node) {
    writer.StartObject();
    writer.String("value");
    writeNameNode(writer, node.value);
    writer.String("location");
    writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeNamedTypeNodeContent(JSONWriter &writer,
                               const shared::ast::NamedTypeNode &node) {
    writer.String("_type");
    writer.String("NamedTypeNode");
    writer.String("name");
    writeNameNode(writer, node.name);
    writer.String("nullable");
    writer.Bool(node.nullable);
    writer.String("location");
    writeNodeLocation(writer, node.location);
};

void writeTypeNode(JSONWriter &writer, const shared::ast::TypeNode &typeNode) {
    writer.StartObject();
    std::visit(overloaded{ [&writer](const shared::ast::NamedTypeNode &node) {
                              writeNamedTypeNodeContent(writer, node);
                          },
                           [&writer](const shared::ast::ListTypeNode &node) {
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
                           } },
               typeNode);
    writer.EndObject();
};

void writeLiteralNode(JSONWriter &writer,
                      const shared::ast::LiteralNode &literalNode) {
    writer.StartObject();
    std::visit(overloaded{
                   [&writer](const shared::ast::LiteralIntNode &node) {
                       writer.String("value");
                       writer.Int(node.value);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const shared::ast::LiteralFloatNode &node) {
                       writer.String("value");
                       writer.Double(node.value);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const shared::ast::LiteralStringNode &node) {
                       writer.String("value");
                       writer.String(node.value.c_str());
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const shared::ast::LiteralBooleanNode &node) {
                       writer.String("value");
                       writer.Bool(node.value);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                   [&writer](const shared::ast::LiteralEnumValueNode &node) {
                       writer.String("value");
                       writer.String(node.value.c_str());
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
               },
               literalNode);
    writer.EndObject();
};

void writeInputValueDefinitionNode(
    JSONWriter &writer, const shared::ast::InputValueDefinitionNode &node) {
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

void writeFieldDefinitionNode(JSONWriter &writer,
                              const ast::FieldDefinitionNode &node) {
    writer.StartObject();
    writer.String("name");
    writeNameNode(writer, node.name);
    writer.String("type");
    writeTypeNode(writer, node.type);
    writer.String("arguments");
    writer.StartArray();
    for (const auto &arg : node.arguments) {
        writeInputValueDefinitionNode(writer, arg);
    };
    writer.EndArray();
    writer.String("location");
    writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeDefinitionNode(JSONWriter &writer,
                         const server::ast::TypeDefinitionNode &node) {
    writer.StartObject();
    std::visit(
        overloaded{ [&writer](const ast::ScalarDefinitionNode &node) {
                       writer.String("_type");
                       writer.String("ScalarDefinitionNode");
                       writer.String("name");
                       writeNameNode(writer, node.name);
                       writer.String("location");
                       writeNodeLocation(writer, node.location);
                   },
                    [&writer](const ast::EnumDefinitionNode &node) {
                        writer.String("_type");
                        writer.String("EnumDefinitionNode");
                        writer.String("name");
                        writeNameNode(writer, node.name);
                        writer.String("values");
                        writer.StartArray();
                        for (const auto &v : node.values) {
                            writeEnumValueDefinitionNode(writer, v);
                        };
                        writer.EndArray();
                        writer.String("location");
                        writeNodeLocation(writer, node.location);
                    },
                    [&writer](const ast::UnionDefinitionNode &node) {
                        writer.String("_type");
                        writer.String("UnionDefinitionNode");
                        writer.String("name");
                        writeNameNode(writer, node.name);
                        writer.String("values");
                        writer.StartArray();
                        for (const auto &v : node.values) {
                            writeNameNode(writer, v);
                        };
                        writer.EndArray();
                        writer.String("location");
                        writeNodeLocation(writer, node.location);
                    },
                    [&writer](const ast::ObjectDefinitionNode &node) {
                        writer.String("_type");
                        writer.String("ObjectDefinitionNode");
                        writer.String("name");
                        writeNameNode(writer, node.name);
                        writer.String("fields");
                        writer.StartArray();
                        for (const auto &v : node.fields) {
                            writeFieldDefinitionNode(writer, v);
                        };
                        writer.EndArray();
                        writer.String("interfaces");
                        writer.StartArray();
                        for (const auto &v : node.interfaces) {
                            writeNameNode(writer, v);
                        };
                        writer.EndArray();
                        writer.String("location");
                        writeNodeLocation(writer, node.location);
                    },
                    [&writer](const ast::InputObjectDefinitionNode &node) {
                        writer.String("_type");
                        writer.String("InputObjectDefinitionNode");
                        writer.String("name");
                        writeNameNode(writer, node.name);
                        writer.String("fields");
                        writer.StartArray();
                        for (const auto &v : node.fields) {
                            writeFieldDefinitionNode(writer, v);
                        };
                        writer.EndArray();
                        writer.String("location");
                        writeNodeLocation(writer, node.location);
                    },
                    [&writer](const ast::InterfaceDefinitionNode &node) {
                        writer.String("_type");
                        writer.String("InterfaceDefinitionNode");
                        writer.String("name");
                        writeNameNode(writer, node.name);
                        writer.String("fields");
                        writer.StartArray();
                        for (const auto &v : node.fields) {
                            writeFieldDefinitionNode(writer, v);
                        };
                        writer.EndArray();
                        writer.String("location");
                        writeNodeLocation(writer, node.location);
                    } },
        node);
    writer.EndObject();
};

void json::serializers::parser::writeFileNodes(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const server::ast::FileNodes &nodes) {
    writer.StartObject();
    writer.String("sourceFile");
    writer.String(nodes.source->filepath.string().c_str());
    writer.String("definitions");
    writer.StartArray();
    for (const auto &node : nodes.definitions) {
        writeDefinitionNode(writer, node);
    };
    writer.EndArray();
    writer.String("extensions");
    writer.StartArray();
    for (const auto &node : nodes.extensions) {
    };
    writer.EndArray();
    writer.EndObject();
};
