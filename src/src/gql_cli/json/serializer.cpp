#include "./serializer.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/rapidjson.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <memory>
#include <variant>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "utils.hpp"

using namespace rapidjson;
void json::serializer::writeTokenAsJSON(PrettyWriter<StringBuffer> &writer,
                                        const GQLToken &token) {
    writer.StartObject();
    writer.String("type");
    writer.String(gqlTokenTypeToString(token.type).c_str());
    writer.String("lexeme");
    writer.String(token.lexeme.c_str());
    writer.String("location");
    {
        writer.StartObject();
        writer.String("line");
        writer.Uint(token.location.line);
        writer.String("start");
        writer.Uint(token.location.start);
        writer.String("end");
        writer.Uint(token.location.end);
        writer.EndObject();
    };
    writer.EndObject();
};

json::serializer::ASTJSONWriter::ASTJSONWriter(
    Writer<GenericStringBuffer<UTF8<>>> *writer)
    : writer{ writer } {};

void writeSchemaNode(rapidjson::PrettyWriter<rapidjson::StringBuffer> &writer,
                     const parsers::schema::SchemaNode &sNode) {
    writer.StartObject();
    writer.String("_type");
    std::visit(
        overloaded{
            [&writer](const std::shared_ptr<parsers::schema::Scalar> &node) {
                writer.String("Scalar");
                writer.String("name");
                writer.String(node->name.c_str());
            },
            [&writer](const std::shared_ptr<parsers::schema::Enum> &node) {
                writer.String("Enum");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("values");
                writer.StartArray();
                for (const auto &value : node->values) {
                    writer.String(value.c_str());
                };
                writer.EndArray();
            },
            [&writer](const std::shared_ptr<parsers::schema::Interface> &node) {
                writer.String("Interface");
                writer.String("name");
                writer.String(node->name.c_str());

                writer.String("fields");
                writer.StartObject();
                writer.EndObject();
            },
            [&writer](const std::shared_ptr<parsers::schema::Union> &node) {
                writer.String("Union");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("items");
                writer.StartArray();
                for (const auto &item : node->items) {
                    writer.String(
                        std::get<std::shared_ptr<parsers::schema::ObjectType>>(
                            item)
                            ->name.c_str());
                };
                writer.EndArray();
            },
            [&writer](
                const std::shared_ptr<parsers::schema::ObjectType> &node) {
                writer.String("ObjectType");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("implements");
                writer.StartArray();
                for (const auto &interface : node->implements) {
                    writer.String(
                        std::get<std::shared_ptr<parsers::schema::Interface>>(
                            interface)
                            ->name.c_str());
                };
                writer.EndArray();
                writer.String("fields");
                writer.StartObject();
                for (const auto &field : node->fields) {
                };
                writer.EndObject();
            },
            [&writer](const std::shared_ptr<parsers::schema::InputType> &node) {
                writer.String("InputType");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("fields");
                writer.StartObject();
                for (const auto& field : node -> fields) {};
                writer.EndObject();
            },
        },
        sNode);
    writer.EndObject();
};
void json::serializer::writeSchemaNodes(
    rapidjson::PrettyWriter<rapidjson::StringBuffer> &writer,
    const std::vector<parsers::schema::SchemaNode> &nodes) {
    writer.StartArray();
    for (const auto &sNode : nodes) {
        writeSchemaNode(writer, sNode);
    };
    writer.EndArray();
};
