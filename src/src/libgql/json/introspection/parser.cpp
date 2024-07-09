#include "./parser.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <algorithm>
#include <cassert>
#include <cstring>
#include <format>
#include <map>
#include <memory>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "../utils.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "utils.hpp"

using namespace json::parser::introspection;
using namespace parsers::schema;

namespace json::parser::introspection {
ServerSchemaNode parseNodeFirstPass(const JSONValue &value) {
    const std::string &kind = value["kind"].GetString();
    const std::string &name = value["name"].GetString();
    if (kind == "SCALAR") {
        return std::make_shared<Scalar>(name);
    } else if (kind == "ENUM") {
        return std::make_shared<Enum>(
            name,
            value["enumValues"].GetArray() |
                std::views::transform([](const auto &item) -> std::string {
                    return item["name"].GetString();
                }) |
                std::ranges::to<std::vector>());
    } else if (kind == "INPUT_OBJECT") {
        return std::make_shared<InputType>(name);
    } else if (kind == "OBJECT") {
        return std::make_shared<ObjectType>(name);
    } else if (kind == "UNION") {
        return std::make_shared<Union>(name);
    } else if (kind == "INTERFACE") {
        return std::make_shared<Interface>(name);
    };
    throw std::runtime_error(
        std::format("Unknown introspection type kind: {}", kind));
};

std::optional<ArrayLiteral> parseArrayLiteral(const JSONValue &value,
                                              const InputTypeSpec &spec) {
    if (value.IsNull()) return std::nullopt;
    const std::string &str = value.GetString();
    const std::vector<std::string> &items =
        split(str.substr(1, str.size() - 2), ",");
    if (std::holds_alternative<std::shared_ptr<Scalar>>(spec)) {
        const auto &scalar = std::get<std::shared_ptr<Scalar>>(spec);
        if (scalar->name == "Int") {
            return items | std::views::transform([](const auto &item) {
                       return std::stoi(item);
                   }) |
                   std::ranges::to<std::vector>();
        } else if (scalar->name == "Float") {
            return items | std::views::transform([](const auto &item) {
                       return std::stof(item);
                   }) |
                   std::ranges::to<std::vector>();
        } else if (scalar->name == "String") {
            return items;
        } else if (scalar->name == "Boolean") {
            return items | std::views::transform([](const auto &item) {
                       return item == "true";
                   }) |
                   std::ranges::to<std::vector>();
        };
        return std::nullopt;
    } else if (std::holds_alternative<std::shared_ptr<Enum>>(spec)) {
        const auto &enumNode = std::get<std::shared_ptr<Enum>>(spec);
        return items | std::views::transform([&enumNode](const auto &item) {
                   const std::string &enumValue = item;
                   if (std::find(enumNode->values.begin(),
                                 enumNode->values.end(),
                                 enumValue) == enumNode->values.end()) {
                       throw std::runtime_error(
                           std::format("Enum {} does not have value {}",
                                       enumNode->name, enumValue));
                   };
                   return enumValue;
               }) |
               std::ranges::to<std::vector>();
    };
    return std::nullopt;
};

std::optional<Literal> parseLiteral(const JSONValue &value,
                                    const InputTypeSpec &spec) {
    if (value.IsNull()) return std::nullopt;
    const std::string &str = value.GetString();
    if (str == "null") return std::nullopt;
    if (std::holds_alternative<std::shared_ptr<Scalar>>(spec)) {
        const auto &scalar = std::get<std::shared_ptr<Scalar>>(spec);
        if (scalar->name == "Int") {
            return std::stoi(str);
        } else if (scalar->name == "Float") {
            return std::stof(str);
        } else if (scalar->name == "String") {
            return str;
        } else if (scalar->name == "Boolean") {
            return str == "true";
        };
        return std::nullopt;
    } else if (std::holds_alternative<std::shared_ptr<Enum>>(spec)) {
        const auto &enumNode = std::get<std::shared_ptr<Enum>>(spec);
        const std::string &enumValue = value.GetString();
        if (std::find(enumNode->values.begin(), enumNode->values.end(),
                      enumValue) == enumNode->values.end()) {
            throw std::runtime_error(std::format(
                "Enum {} does not have value {}", enumNode->name, enumValue));
        };
        return enumValue;
    };
    return std::nullopt;
};

InputTypeSpec parseInputTypeSpec(const JSONValue &value,
                                 const TypeRegistry &registry) {
    std::string kind = value["kind"].GetString();
    if (kind == "NON_NULL")
        return parseInputTypeSpec(value["ofType"], registry);
    std::string name = value["name"].GetString();
    if (kind == "SCALAR") {
        return registry.scalars.at(name);
    } else if (kind == "ENUM") {
        return registry.enums.at(name);
    } else if (kind == "INPUT_OBJECT") {
        return registry.inputs.at(name);
    };
    throw std::runtime_error(std::format(
        "Unknown type suitable for input with name {} was found", name));
};

InputFieldSpec parseInputFieldSpec(const JSONValue &value,
                                   const JSONValue &defaultValue,
                                   const TypeRegistry &registry) {
    std::string kind = value["kind"].GetString();
    if (kind == "NON_NULL")
        return parseInputFieldSpec(value["ofType"], defaultValue, registry);
    if (kind == "LIST") {
        const auto &type = parseInputTypeSpec(value["ofType"], registry);
        return (ArrayFieldSpec<InputTypeSpec>){
            { .defaultValue = parseArrayLiteral(defaultValue, type) },
            .type = type,
            .nullable =
                strcmp(value["ofType"]["kind"].GetString(), "NON_NULL") != 0
        };
    };
    const auto &type = parseInputTypeSpec(value, registry);
    return (LiteralFieldSpec<InputTypeSpec>){
        { .defaultValue = parseLiteral(defaultValue, type) }, .type = type
    };
};

FieldDefinition<InputFieldSpec> parseInputFieldDefinition(
    const JSONValue &value, const TypeRegistry &registry) {
    return { .name = value["name"].GetString(),
             .spec = parseInputFieldSpec(value["type"], value["defaultValue"],
                                         registry),
             .nullable =
                 strcmp(value["type"]["kind"].GetString(), "NON_NULL") != 0 };
};

ObjectTypeSpec parseObjectTypeSpec(const JSONValue &value,
                                   const TypeRegistry &registry) {
    std::string kind = value["kind"].GetString();
    if (kind == "NON_NULL")
        return parseObjectTypeSpec(value["ofType"], registry);
    std::string name = value["name"].GetString();
    if (kind == "SCALAR") {
        return registry.scalars.at(name);
    } else if (kind == "ENUM") {
        return registry.enums.at(name);
    } else if (kind == "OBJECT") {
        return registry.objects.at(name);
    } else if (kind == "UNION") {
        return registry.unions.at(name);
    } else if (kind == "INTERFACE") {
        return registry.interfaces.at(name);
    };
    throw std::runtime_error(std::format(
        "Unknown type suitable for object with name {} was found", name));
};

NonCallableFieldSpec<ObjectTypeSpec> parseNonCallableObjectFieldSpec(
    const JSONValue &value, const TypeRegistry &registry) {
    std::string kind = value["kind"].GetString();
    if (kind == "NON_NULL")
        return parseNonCallableObjectFieldSpec(value["ofType"], registry);
    if (kind == "LIST") {
        const auto &type = parseObjectTypeSpec(value["ofType"], registry);
        return (ArrayFieldSpec<ObjectTypeSpec>){
            .type = type,
            .nullable =
                strcmp(value["ofType"]["kind"].GetString(), "NON_NULL") != 0
        };
    };
    const auto &type = parseObjectTypeSpec(value, registry);
    return (LiteralFieldSpec<ObjectTypeSpec>){ .type = type };
};

ObjectFieldSpec parseObjectFieldSpec(const JSONValue &value,
                                     const JSONValue &argsValue,
                                     const TypeRegistry &registry) {
    std::string kind = value["kind"].GetString();
    const auto &args = argsValue.GetArray();
    if (kind == "NON_NULL")
        return parseObjectFieldSpec(value["ofType"], argsValue, registry);
    if (args.Size() != 0) {
        return (CallableFieldSpec){
            .returnType = parseNonCallableObjectFieldSpec(value, registry),
            .arguments =
                args | std::views::transform([&registry](const auto &argValue) {
                    const auto &field =
                        std::make_shared<FieldDefinition<InputFieldSpec>>(
                            parseInputFieldDefinition(argValue, registry));
                    return std::make_pair(field->name, field);
                }) |
                std::ranges::to<std::map>()
        };
    };
    if (kind == "LIST") {
        const auto &type = parseObjectTypeSpec(value["ofType"], registry);
        return (ArrayFieldSpec<ObjectTypeSpec>){
            .type = type,
            .nullable =
                strcmp(value["ofType"]["kind"].GetString(), "NON_NULL") != 0
        };
    };
    const auto &type = parseObjectTypeSpec(value, registry);
    return (LiteralFieldSpec<ObjectTypeSpec>){ .type = type };
};

std::shared_ptr<FieldDefinition<ObjectFieldSpec>> parseObjectFieldDefinition(
    const JSONValue &value, const TypeRegistry &registry) {
    return std::make_shared<FieldDefinition<ObjectFieldSpec>>(
        value["name"].GetString(),
        parseObjectFieldSpec(value["type"], value["args"], registry),
        strcmp(value["type"]["kind"].GetString(), "NON_NULL") != 0);
};

ServerSchemaNode parseNodeSecondPass(const JSONValue &value,
                                     const TypeRegistry &registry) {
    const std::string &kind = value["kind"].GetString();
    const std::string &name = value["name"].GetString();
    if (kind == "SCALAR") {
        return registry.scalars.at(name);
    } else if (kind == "ENUM") {
        return registry.enums.at(name);
    } else if (kind == "INPUT_OBJECT") {
        const auto &input = registry.inputs.at(name);
        input->fields = value["inputFields"].GetArray() |
                        std::views::transform([&registry](const auto &value) {
                            const auto &field =
                                parseInputFieldDefinition(value, registry);
                            return std::make_pair(field.name, field);
                        }) |
                        std::ranges::to<std::map>();
        return input;
    } else if (kind == "OBJECT") {
        const auto &object = registry.objects.at(name);
        object->implements =
            value["interfaces"].GetArray() |
            std::views::transform([&registry](const auto &value) {
                const auto &interface =
                    registry.interfaces.at(value["name"].GetString());
                return std::make_pair(interface->name, interface);
            }) |
            std::ranges::to<std::map>();
        object->fields = value["fields"].GetArray() |
                         std::views::transform([&registry](const auto &value) {
                             const auto &field =
                                 parseObjectFieldDefinition(value, registry);
                             return std::make_pair(field->name, field);
                         }) |
                         std::ranges::to<std::map>();
        return object;
    } else if (kind == "UNION") {
        const auto &unionNode = registry.unions.at(name);
        unionNode->items = value["possibleTypes"].GetArray() |
                           std::views::transform([&registry](const auto &item) {
                               const auto &object = registry.objects.at(
                                   item["name"].GetString());
                               return std::make_pair(object->name, object);
                           }) |
                           std::ranges::to<std::map>();
        return unionNode;
    } else if (kind == "INTERFACE") {
        const auto &interface = registry.interfaces.at(name);
        interface->fields =
            value["fields"].GetArray() |
            std::views::transform([&registry](const auto &value) {
                const auto &field = parseObjectFieldDefinition(value, registry);
                return std::make_pair(field->name, field);
            }) |
            std::ranges::to<std::map>();
        return interface;
    };
    throw std::runtime_error(
        std::format("Unknown introspection type kind: {}", kind));
};

const ServerSchema parseIntrospectionSchema(
    const rapidjson::Document &document) {
    const auto &types = document["data"]["__schema"]["types"].GetArray();
    TypeRegistry registry;
    for (const auto &item :
         types | std::views::filter([](const auto &el) {
             return !std::string(el["name"].GetString()).starts_with("__");
         })) {
        const auto &node = parseNodeFirstPass(item);
        registry.addNode(node);
    };

    return ServerSchema::fromNodes(
        types | std::views::filter([](const auto &el) {
            return !std::string(el["name"].GetString()).starts_with("__");
        }) |
        std::views::transform([&registry](const auto &item) {
            return parseNodeSecondPass(item, registry);
        }) |
        std::ranges::to<std::vector>());
};
};
