
#include "json/serializer.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <iostream>
#include <variant>

#include "lexer/token.hpp"
#include "parsers/server/parser.hpp"

using namespace parsers::server;
using namespace rapidjson;
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;
void json::serializer::writeTokenAsJSON(
    Writer<GenericStringBuffer<UTF8<>>> &writer, const GQLToken &token) {
    writer.StartObject();
    writer.String("type");
    writer.String(gqlTokenTypeToString(token.type).c_str());
    writer.String("lexeme");
    writer.String(token.lexeme.c_str());
    writer.String("location");
    {
        writer.StartObject();
        writer.String("line");
        writer.Int(token.location.line);
        writer.String("start");
        writer.Int(token.location.start);
        writer.String("end");
        writer.Int(token.location.end);
        writer.EndObject();
    };
    writer.EndObject();
};

json::serializer::ASTJSONWriter::ASTJSONWriter(
    Writer<GenericStringBuffer<UTF8<>>> &writer)
    : writer{ writer } {};
void json::serializer::ASTJSONWriter::writeProgram(
    const ASTProgram &program) noexcept {
    writer.StartObject();
    writer.String("nodes");
    writer.StartArray();
    int index = 0;
    for (const auto &node : program.nodes) {
        writeNode(node);
        index++;
    };
    writer.EndArray();
    writer.EndObject();
};

void json::serializer::ASTJSONWriter::writeNode(const ASTNode &node) noexcept {
    std::visit(overloaded{
                   [this](const ASTTypeDefinition &node) {
                       writeTypeDefinitionNode(node);
                   },
                   [this](const ASTTypeSpec &node) { writeTypeSpecNode(node); },
                   [this](const ASTUnionNode &node) { writeUnionNode(node); },
                   [this](const ASTEnumNode &node) { writeEnumNode(node); },
                   [this](const ASTExtendNode &node) { writeExtendNode(node); },
               },
               node);
};
void json::serializer::ASTJSONWriter::writeUnionNode(
    const ASTUnionNode &node) noexcept {
    std::cout << "writeUnionNode" << std::endl;
    writer.StartObject();
    writer.String("_nodeType");
    writer.String("union");
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("items");
    writer.StartArray();
    for (const auto &item : node.items) {
        writeReferenceTypeNode(item);
    };
    writer.EndArray();
    writer.EndObject();
};
void json::serializer::ASTJSONWriter::writeEnumNode(
    const ASTEnumNode &node) noexcept {
    writer.StartObject();
    writer.String("_nodeType");
    writer.String("enum");
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("items");
    writer.StartArray();
    for (const auto &item : node.items) {
        writer.String(item.c_str());
    };
    writer.EndArray();
    writer.EndObject();
};
void json::serializer::ASTJSONWriter::writeTypeSpecNode(
    const ASTTypeSpec &node) noexcept {
    writer.StartObject();
    writer.String("type");
    writeASTGQLType(node.type);
    writer.EndObject();
};
void json::serializer::ASTJSONWriter::writeASTGQLType(
    const ASTGQLType &type) noexcept {
    std::visit(
        overloaded{ [this](const ASTGQLSimpleType &sType) {
                       writer.String(astGQLSimpleTypeToString(sType).c_str());
                   },
                    [this](const ASTGQLReferenceType &rType) {
                        writeReferenceTypeNode(rType);
                    } },
        type);
};
void json::serializer::ASTJSONWriter::writeTypeDefinitionNode(
    const ASTTypeDefinition &node) noexcept {
    writer.StartObject();
    writer.String("_nodeType");
    writer.String("typeDefinition");
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("isInput");
    writer.Bool(node.isInput);
    writer.String("fields");
    writer.StartObject();
    for (const auto& [name, field] : node.fields) {
        writer.String(name.c_str());
        writeTypeSpecNode(field);
    };
    writer.EndObject();
    writer.EndObject();
};
void json::serializer::ASTJSONWriter::writeReferenceTypeNode(
    const ASTGQLReferenceType &node) noexcept {
    writer.StartObject();
    writer.String("_nodeType");
    writer.String("referenceType");
    writer.String("name");
    writer.String(node.name.c_str());
    writer.EndObject();
};

void json::serializer::ASTJSONWriter::writeExtendNode(
    const ASTExtendNode &node) noexcept {
    writer.StartObject();
    writer.String("type");
    writeTypeDefinitionNode(node.type);
    writer.EndObject();
};
