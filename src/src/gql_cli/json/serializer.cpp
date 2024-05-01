#include "./serializer.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <optional>
#include <variant>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/server/ast.hpp"

using namespace parsers::server::ast;
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
    const parsers::server::ast::ASTProgram &program) noexcept {
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
    std::visit(
        overloaded{
            [this](const ASTTypeDefinition &node) {
                writeTypeDefinitionNode(node);
            },
            [this](const ASTTrivialTypeSpec &node) { writeTypeSpecNode(node); },
            [this](const ASTUnionNode &node) { writeUnionNode(node); },
            [this](const ASTEnumNode &node) { writeEnumNode(node); },
            [this](const ASTExtendNode &node) { writeExtendNode(node); },
        },
        node);
};
void json::serializer::ASTJSONWriter::writeUnionNode(
    const ASTUnionNode &node) noexcept {
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

void json::serializer::ASTJSONWriter::writeTrivialTypeSpecNode(
    const ASTTrivialTypeSpec &node) noexcept {
    writer.StartObject();
    writer.String("type");
    writeASTGQLType(node.type);
    writer.String("nullable");
    writer.Bool(node.nullable);
    writer.String("defaultValue");
    writeMaybeASTLiteralType(node.defaultValue);
    writer.EndObject();
};

void json::serializer::ASTJSONWriter::writeArrayTypeSpecNode(
    const ASTArrayTypeSpec &node) noexcept {
    writer.StartObject();
    writer.String("type");
    writer.String("array");
    writer.String("items");
    writeTrivialTypeSpecNode(node.type);
    writer.String("nullable");
    writer.Bool(node.nullable);
    writer.String("defaultType");
    writeMaybeASTArrayLiteralType(node.defaultValue);
    writer.EndObject();
};

void json::serializer::ASTJSONWriter::writeMaybeASTArrayLiteralType(
    const std::optional<ASTArrayLiteral>& node
) noexcept {
    if (!node.has_value()) {
        writer.Null();
        return;
    };
    writeASTArrayLiteralType(node.value());
};

void json::serializer::ASTJSONWriter::writeASTArrayLiteralType(
    const ASTArrayLiteral& node
) noexcept {
    writer.StartArray();
    std::visit(overloaded{
        [this](const ASTStringArrayLiteral& v){
            for (const auto& item : v) {
                writer.String(item.value.c_str());
            };
        },
        [this](const ASTIntArrayLiteral& v){
            for (const auto& item : v) {
                writer.Int(item.value);
            };
        },
        [this](const ASTFloatArrayLiteral& v){
            for (const auto& item : v) {
                writer.Double(item.value);
            };
        },
        [this](const ASTBooleanArrayLiteral& v){
            for (const auto& item : v) {
                writer.Bool(item.value);
            };
        }
    }, node);
    writer.EndArray();
};
void json::serializer::ASTJSONWriter::writeMaybeASTLiteralType(
    const std::optional<ASTLiteral>& node
) noexcept {
    if (!node.has_value()) {
        writer.Null();
        return;
    };
    writeASTLiteralType(node.value());
};

void json::serializer::ASTJSONWriter::writeASTLiteralType(
    const ASTLiteral& node
) noexcept {
    std::visit(overloaded{
        [this](const ASTStringLiteral& v){ writer.String(v.value.c_str()); },
        [this](const ASTIntLiteral& v){ writer.Int(v.value); },
        [this](const ASTFloatLiteral& v){ writer.Double(v.value); },
        [this](const ASTBooleanLiteral& v){ writer.Bool(v.value); },
    }, node);
};

void json::serializer::ASTJSONWriter::writeTypeSpecNode(
    const ASTTypeSpec &node) noexcept {
    if (std::holds_alternative<ASTLiteralTypeSpec>(node)) {
        return writeLiteralTypeSpecNode(std::get<ASTLiteralTypeSpec>(node));
    } else {
        return writeCallableTypeSpecNode(std::get<ASTCallableTypeSpec>(node));
    };
};

void json::serializer::ASTJSONWriter::writeCallableTypeSpecNode(
    const ASTCallableTypeSpec &node) noexcept {
    writer.StartObject();
    writer.String("arguments");
    writer.StartObject();
    for (const auto& [name, arg] : node.arguments) {
        writer.String(name.c_str());
        writeLiteralTypeSpecNode(arg);
    };
    writer.EndObject();
    writer.String("returnType");
    writeLiteralTypeSpecNode(node.returnType);
    writer.EndObject();
};

void json::serializer::ASTJSONWriter::writeLiteralTypeSpecNode(
    const ASTLiteralTypeSpec &node) noexcept {
    if (std::holds_alternative<ASTArrayTypeSpec>(node)) {
        return writeArrayTypeSpecNode(std::get<ASTArrayTypeSpec>(node));
    } else {
        return writeTrivialTypeSpecNode(std::get<ASTTrivialTypeSpec>(node));
    };
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
    for (const auto &[name, field] : node.fields) {
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
