#include "./schema.hpp"

#include <rapidjson/document.h>

#include <functional>
#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <vector>

#include "../../utils.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace parsers::schema;
using namespace parsers::schema::ast;

template <typename T>
std::map<std::string, T> parseNodesFromArray(
    const JSONArray &array,
    const std::function<T(const JSONValue &)> &parserFunc,
    TypeRegistry &registry) {
    return array |
           std::views::transform([&registry, &parserFunc](const auto &el) {
               const auto &node = parserFunc(el);
               registry.addNode(node);
               return std::make_pair(node->name, node);
           }) |
           std::ranges::to<std::map>();
};

template <typename T>
std::map<std::string, T> parseNodesFromObject(
    const JSONObject &array,
    const std::function<T(const JSONObjectEntry &)> &parserFunc,
    TypeRegistry &registry) {
    return array |
           std::views::transform([&registry, &parserFunc](const auto &el) {
               const auto &node = parserFunc(el);
               registry.addNode(node);
               return std::make_pair(node->name, node);
           }) |
           std::ranges::to<std::map>();
};

template <typename T>
void addNodesToRegistry(const JSONObject &object, TypeRegistry &registry) {
    for (const auto &obj : object) {
        registry.addNode(std::make_shared<T>(obj.name.GetString()));
    };
};

namespace json::parsers::schema {
::parsers::schema::Schema parseSchema(const rapidjson::Document &document) {
    const auto &serverSchema = document["server"];
    TypeRegistry registry;
    Schema schema;
    schema.server.scalars = parseNodesFromArray<std::shared_ptr<Scalar>>(
        serverSchema["scalars"].GetArray(), parseScalar, registry);
    schema.server.enums = parseNodesFromObject<std::shared_ptr<Enum>>(
        serverSchema["enums"].GetObject(), parseEnum, registry);
    addNodesToRegistry<InputType>(serverSchema["inputs"].GetObject(), registry);
    addNodesToRegistry<ObjectType>(serverSchema["inputs"].GetObject(),
                                   registry);
    addNodesToRegistry<Interface>(serverSchema["inputs"].GetObject(), registry);
    addNodesToRegistry<Union>(serverSchema["inputs"].GetObject(), registry);
    schema.server.inputs =
        serverSchema["inputs"].GetObject() |
        std::views::transform([&registry](const auto &obj) {
            const auto &input = registry.inputs.at(obj.name.GetString());
            input->fields =
                obj.value["fields"].GetObject() |
                std::views::transform([&registry](const auto &entry) {
                    const auto &field =
                        parseInputFieldDefinition(entry, registry);
                    return std::make_pair(field.name, field);
                }) |
                std::ranges::to<std::map>();
            return std::make_pair(input->name, input);
        }) |
        std::ranges::to<std::map>();
    schema.server.objects =
        serverSchema["objects"].GetObject() |
        std::views::transform([&registry](const auto &entry) {
            const auto &obj = registry.objects.at(entry.name.GetString());
            obj->implements =
                entry.value["implements"].GetObject() |
                std::views::transform([&registry](const auto &entry) {
                    const auto &interface =
                        registry.interfaces.at(entry.name.GetString());
                    return std::make_pair(interface->name, interface);
                }) |
                std::ranges::to<std::map>();
            obj->fields = entry.value["fields"].GetObject() |
                          std::views::transform([&registry](const auto &entry) {
                              const auto &field =
                                  parseObjectFieldDefinition(entry, registry);
                              return std::make_pair(field->name, field);
                          }) |
                          std::ranges::to<std::map>();
            return std::make_pair(obj->name, obj);
        }) |
        std::ranges::to<std::map>();
    schema.server.interfaces =
        serverSchema["interfaces"].GetObject() |
        std::views::transform([&registry](const auto &entry) {
            const auto &interface =
                registry.interfaces.at(entry.name.GetString());
            interface->fields =
                entry.value["fields"].GetObject() |
                std::views::transform([&registry](const auto &entry) {
                    const auto &field =
                        parseObjectFieldDefinition(entry, registry);
                    return std::make_pair(field->name, field);
                }) |
                std::ranges::to<std::map>();
            return std::make_pair(interface->name, interface);
        }) |
        std::ranges::to<std::map>();
    schema.server.unions =
        serverSchema["unions"].GetObject() |
        std::views::transform([&registry](const auto &entry) {
            const auto &node = registry.unions.at(entry.name.GetString());
            node->items = entry.value["items"].GetObject() |
                          std::views::transform([&registry](const auto &entry) {
                              const auto &object =
                                  registry.objects.at(entry.name.GetString());
                              return std::make_pair(object->name, object);
                          }) |
                          std::ranges::to<std::map>();
            return std::make_pair(node->name, node);
        }) |
        std::ranges::to<std::map>();
    return schema;
};

std::shared_ptr<Scalar> parseScalar(const JSONValue &value) {
    return std::make_shared<Scalar>(value.GetString());
};

std::shared_ptr<Enum> parseEnum(const JSONObjectEntry &entry) {
    return std::make_shared<Enum>(
        entry.name.GetString(),
        entry.value["values"].GetArray() |
            std::views::transform([](const auto &item) -> std::string {
                return item.GetString();
            }) |
            std::ranges::to<std::vector>());
};

InputTypeSpec parseInputTypeSpec(const JSONValue &value,
                                 const TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    const std::string &name = value["name"].GetString();
    if (_type == "Scalar") {
        return registry.scalars.at(name);
    } else if (_type == "Enum") {
        return registry.enums.at(name);
    }
    return registry.inputs.at(name);
};

InputFieldSpec parseInputFieldSpec(const JSONValue &value,
                                   const TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        return (LiteralFieldSpec<InputTypeSpec>){
            .type = parseInputTypeSpec(value["type"], registry)
        };
    }
    return (ArrayFieldSpec<InputTypeSpec>){
        .type = parseInputTypeSpec(value["type"], registry),
        .nullable = value["nullable"].GetBool()
    };
};

FieldDefinition<InputFieldSpec> parseInputFieldDefinition(
    const JSONObjectEntry &entry, const TypeRegistry &registry) {
    return { .name = entry.name.GetString(),
             .spec = parseInputFieldSpec(entry.value["spec"], registry),
             .nullable = entry.value["nullable"].GetBool() };
};

ObjectTypeSpec parseObjectTypeSpec(const JSONValue &value,
                                   const TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    const std::string &name = value["name"].GetString();
    if (_type == "Scalar") {
        return registry.scalars.at(name);
    } else if (_type == "Enum") {
        return registry.enums.at(name);
    } else if (_type == "Union") {
        return registry.unions.at(name);
    } else if (_type == "InterfaceType") {
        return registry.interfaces.at(name);
    };
    return registry.objects.at(name);
};

NonCallableFieldSpec<ObjectTypeSpec> parseNonCallableFieldSpec(
    const JSONValue &value, const TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        return (LiteralFieldSpec<ObjectTypeSpec>){
            .type = parseObjectTypeSpec(value["type"], registry)
        };
    }
    return (ArrayFieldSpec<ObjectTypeSpec>){
        .type = parseObjectTypeSpec(value["type"], registry),
        .nullable = value["nullable"].GetBool()
    };
};

ObjectFieldSpec parseObjectFieldSpec(const JSONValue &value,
                                     const TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        return (LiteralFieldSpec<ObjectTypeSpec>){
            .type = parseObjectTypeSpec(value["type"], registry)
        };
    } else if (_type == "array") {
        return (ArrayFieldSpec<ObjectTypeSpec>){
            .type = parseObjectTypeSpec(value["type"], registry),
            .nullable = value["nullable"].GetBool()
        };
    };
    return (CallableFieldSpec){
        .returnType = parseNonCallableFieldSpec(value["returnType"], registry),
        .arguments = value["arguments"].GetObject() |
                     std::views::transform([&registry](const auto &entry) {
                         const auto &field =
                             std::make_shared<FieldDefinition<InputFieldSpec>>(
                                 parseInputFieldDefinition(entry, registry));
                         return std::make_pair(field->name, field);
                     }) |
                     std::ranges::to<std::map>()
    };
};

std::shared_ptr<FieldDefinition<ObjectFieldSpec>> parseObjectFieldDefinition(
    const JSONObjectEntry &entry, const TypeRegistry &registry) {
    return std::make_shared<FieldDefinition<ObjectFieldSpec>>(
        entry.name.GetString(),
        parseObjectFieldSpec(entry.value["spec"], registry),
        entry.value["nullable"].GetBool());
};
};  // namespace json::parsers::schema
