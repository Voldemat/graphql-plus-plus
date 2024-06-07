#include "./serializer.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/rapidjson.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <format>
#include <memory>
#include <ranges>
#include <variant>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "utils.hpp"

using namespace rapidjson;
using namespace parsers;

void json::serializer::writeTokenAsJSON(rapidjson::PrettyWriter<StringBuffer> &writer,
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

void writeTypeSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                   const schema::ObjectTypeSpec &field) {

    writer.StartObject();
    std::visit(overloaded{
        [&writer](const std::shared_ptr<schema::ObjectType>& node){
            writer.String("_type");
            writer.String("ObjectType");
            writer.String("$ref");
            writer.String(std::format("#/server/objects/{}", node->name).c_str());
        },
        [&writer](const std::shared_ptr<schema::Interface>& node){
            writer.String("_type");
            writer.String("InterfaceType");
            writer.String("$ref");
            writer.String(std::format("#/server/interfaces/{}", node->name).c_str());
        },
        [&writer](const std::shared_ptr<schema::Scalar>& node){
            writer.String("_type");
            writer.String("Scalar");
            writer.String("name");
            writer.String(node->name.c_str());
        },
        [&writer](const std::shared_ptr<schema::Enum>& node){
            writer.String("_type");
            writer.String("Enum");
            writer.String("$ref");
            writer.String(std::format("#/server/enums/{}", node->name).c_str());
        },
        [&writer](const std::shared_ptr<schema::Union>& node){
            writer.String("_type");
            writer.String("Union");
            writer.String("$ref");
            writer.String(std::format("#/server/unions/{}", node->name).c_str());
        },
    }, field);
    writer.EndObject();
};

void writeTypeSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                   const schema::InputTypeSpec &field) {

    writer.StartObject();
    std::visit(overloaded{
        [&writer](const std::shared_ptr<schema::InputType>& node){
            writer.String("_type");
            writer.String("InputType");
            writer.String("$ref");
            writer.String(std::format("#/server/inputs/{}", node->name).c_str());
        },
        [&writer](const std::shared_ptr<schema::Interface>& node){
            writer.String("_type");
            writer.String("Interface");
            writer.String("$ref");
            writer.String(std::format("#/server/interfaces/{}", node->name).c_str());
        },
        [&writer](const std::shared_ptr<schema::Scalar>& node){
            writer.String("_type");
            writer.String("Scalar");
            writer.String("name");
            writer.String(node->name.c_str());
        },
        [&writer](const std::shared_ptr<schema::Enum>& node){
            writer.String("_type");
            writer.String("Enum");
            writer.String("$ref");
            writer.String(std::format("#/server/enums/{}", node->name).c_str());
        },
        [&writer](const std::shared_ptr<schema::Union>& node){
            writer.String("_type");
            writer.String("Union");
            writer.String("$ref");
            writer.String(std::format("#/server/unions/{}", node->name).c_str());
        },
    }, field);
    writer.EndObject();
};

template <typename T>
void writeFieldSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                    const schema::NonCallableFieldSpec<T> &field) {
    writer.StartObject();
    std::visit(
        overloaded{
            [&writer](
                const schema::LiteralFieldSpec<T> &node) {
                writer.String("_type");
                writer.String("literal");
                writer.String("type");
                writeTypeSpec(writer, node.type);
            },
            [&writer](
                const schema::ArrayFieldSpec<T> &node) {
                writer.String("_type");
                writer.String("array");
                writer.String("nullable");
                writer.Bool(node.nullable);
                writer.String("type");
                writeTypeSpec(writer, node.type);
            },
        },
        field);
    writer.EndObject();
};



template <typename T>
void writeFieldDefinition(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::FieldDefinition<T> &field);

void writeFieldSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                    const schema::ObjectFieldSpec &field) {
    writer.StartObject();
    std::visit(
        overloaded{
            [&writer](
                const schema::LiteralFieldSpec<schema::ObjectTypeSpec> &node) {
                writer.String("_type");
                writer.String("literal");
                writer.String("type");
                writeTypeSpec(writer, node.type);
            },
            [&writer](
                const schema::ArrayFieldSpec<schema::ObjectTypeSpec> &node) {
                writer.String("_type");
                writer.String("array");
                writer.String("nullable");
                writer.Bool(node.nullable);
                writer.String("type");
                writeTypeSpec(writer, node.type);
            },
            [&writer](const schema::CallableFieldSpec &node) {
                writer.String("_type");
                writer.String("callable");
                writer.String("returnType");
                writeFieldSpec(writer, node.returnType);
                writer.String("arguments");
                writer.StartObject();
                for (const auto& arg : node.arguments | std::views::values) {
                    writer.String(arg->name.c_str());
                    writeFieldDefinition(writer, *arg.get());
                };
                writer.EndObject();
            },
        },
        field);
    writer.EndObject();
};

template <typename T>
void writeFieldDefinition(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::FieldDefinition<T> &field) {
    writer.StartObject();
    writer.String("nullable");
    writer.Bool(field.nullable);
    writer.String("spec");
    writeFieldSpec(writer, field.spec);
    writer.EndObject();
};

void writeSchemaNode(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                     const parsers::schema::ServerSchemaNode &sNode) {
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
                for (const auto &field : node->fields | std::views::values) {
                    writeFieldDefinition(writer, *field.get());
                };
                writer.EndObject();
            },
            [&writer](const std::shared_ptr<parsers::schema::Union> &node) {
                writer.String("Union");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("items");
                writer.StartArray();
                for (const auto &[name, _] : node->items) {
                    writer.String(name.c_str());
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
                for (const auto &[name, _] : node->implements) {
                    writer.String(name.c_str());
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
                for (const auto &field : node->fields) {

                };
                writer.EndObject();
            },
        },
        sNode);
    writer.EndObject();
};

void writeServerType(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::ObjectType &node
) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("implements");
    writer.StartArray();
    for (const auto& interface : node.implements | std::views::values) {
        writer.StartObject();
        writer.String("name");
        writer.String(interface->name.c_str());
        writer.String("$ref");
        writer.String(std::format("#/server/interfaces/{}", interface->name).c_str());
        writer.EndObject();
    };
    writer.EndArray();
    writer.String("fields");
    writer.StartObject();
    for (const auto& field : node.fields | std::views::values) {
        writer.String(field->name.c_str());
        writeFieldDefinition(writer, *field.get());
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerType(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::Interface &node
) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("fields");
    writer.StartObject();
    for (const auto& field : node.fields | std::views::values) {
        writer.String(field->name.c_str());
        writeFieldDefinition(writer, *field.get());
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerType(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::InputType &node
) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("fields");
    writer.StartObject();
    for (const auto& field : node.fields | std::views::values) {
        writer.String(field.name.c_str());
        writeFieldDefinition(writer, field);
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerType(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::Enum &node
) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("values");
    writer.StartArray();
    for (const auto& value : node.values) {
        writer.String(value.c_str());
    };
    writer.EndArray();
    writer.EndObject();
};


void writeServerType(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::Union &node
) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("items");
    writer.StartObject();
    for (const auto& value : node.items | std::views::values) {
        writer.String(value->name.c_str());
        writer.String(std::format("#/server/objects/{}", value->name).c_str());
    };
    writer.EndObject();
    writer.EndObject();
};



void writeServerSchema(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::ServerSchema &schema
) {
    writer.StartObject();

    writer.String("objects");
    writer.StartObject();
    for (const auto& obj : schema.objects | std::views::values) {
        writer.String(obj->name.c_str());
        writeServerType(writer, *obj.get());
    };
    writer.EndObject();

    writer.String("interfaces");
    writer.StartObject();
    for (const auto& interface : schema.interfaces | std::views::values) {
        writer.String(interface->name.c_str());
        writeServerType(writer, *interface.get());
    };
    writer.EndObject();

    writer.String("inputs");
    writer.StartObject();
    for (const auto& input : schema.inputs | std::views::values) {
        writer.String(input->name.c_str());
        writeServerType(writer, *input.get());
    };
    writer.EndObject();

    writer.String("scalars");
    writer.StartArray();
    for (const auto& name : schema.scalars | std::views::keys) {
        writer.String(name.c_str());
    };
    writer.EndArray();

    writer.String("enums");
    writer.StartObject();
    for (const auto& node : schema.enums | std::views::values) {
        writer.String(node->name.c_str());
        writeServerType(writer, *node.get());
    };
    writer.EndObject();

    writer.String("unions");
    writer.StartObject();
    for (const auto& node : schema.unions | std::views::values) {
        writer.String(node->name.c_str());
        writeServerType(writer, *node.get());
    };
    writer.EndObject();

    writer.EndObject();
};

void json::serializer::writeSchemaNodes(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const schema::Schema &schema) {
    writer.StartObject();
    writer.String("server");
    writeServerSchema(writer, schema.server);
    writer.EndObject();
};
