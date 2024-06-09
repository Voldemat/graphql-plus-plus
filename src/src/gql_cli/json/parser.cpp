#include "./parser.hpp"

#include <rapidjson/allocators.h>
#include <rapidjson/document.h>
#include <rapidjson/encodings.h>

#include <cassert>
#include <expected>
#include <format>
#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "gql_cli/json/utils.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

using namespace json::parser;
using namespace parsers::schema;

std::expected<std::vector<GQLToken>, std::string>
json::parser::parseTokensArray(rapidjson::Document const &document) noexcept {
    if (!document.IsArray())
        return std::unexpected("JSON root is not an array");
    unsigned int index = 0;
    std::vector<GQLToken> tokens;
    for (const auto &item : document.GetArray()) {
        if (!item.IsObject()) {
            return std::unexpected(
                std::format("Element {} is not an object", index));
        };
        if (!item.HasMember("type")) {
            return std::unexpected(
                std::format("Element {} doesn`t have \"type\" key", index));
        };
        auto maybeType = gqlTokenTypeFromString(item["type"].GetString());
        if (!maybeType.has_value()) {
            return std::unexpected(
                std::format("Element {}.type is invalid", index));
        };
        const GQLTokenType type = maybeType.value();
        if (!item.HasMember("lexeme")) {
            return std::unexpected(
                std::format("Element {} doesn`t have \"lexeme\" key", index));
        };
        const std::string lexeme = item["lexeme"].GetString();
        if (!item.HasMember("location")) {
            return std::unexpected(
                std::format("Element {} doesn`t have \"location\" key", index));
        };
        const auto locationOrError = parseLocation(item["location"]);
        if (!locationOrError.has_value()) {
            const auto &[key, eType] = locationOrError.error();
            if (eType == ParsingErrorType::INVALID_MEMBER) {
                return std::unexpected(std::format(
                    "Element {}.location.{} is invalid", index, key));
            };
            return std::unexpected(std::format(
                "Element {}.location doesn`t have \"{}\" key", index, key));
        };
        const Location location = locationOrError.value();
        index++;
        tokens.push_back(
            { .type = type, .lexeme = lexeme, .location = location });
    };
    return tokens;
};

#define ASSERT_HAS_MEMBER(key)                                 \
    if (!value.HasMember(key)) {                               \
        return std::unexpected(                                \
            std::make_pair(key, ParsingErrorType::NO_MEMBER)); \
    }

#define ASSERT_MEMBER_IS_VALID(key, isValid)                        \
    if (!isValid) {                                                 \
        return std::unexpected(                                     \
            std::make_pair(key, ParsingErrorType::INVALID_MEMBER)); \
    }
std::expected<Location, std::tuple<std::string, ParsingErrorType>>
json::parser::parseLocation(
    rapidjson::GenericValue<rapidjson::UTF8<>> const &value) noexcept {
    ASSERT_HAS_MEMBER("line");
    ASSERT_MEMBER_IS_VALID("line", value["line"].IsUint());
    unsigned int line = value["line"].GetUint();
    ASSERT_HAS_MEMBER("start");
    ASSERT_MEMBER_IS_VALID("start", value["start"].IsUint());
    unsigned int start = value["start"].GetUint();
    ASSERT_HAS_MEMBER("end");
    ASSERT_MEMBER_IS_VALID("end", value["end"].IsUint());
    unsigned int end = value["end"].GetUint();
    return (Location){ .line = line, .start = start, .end = end };
};

const std::shared_ptr<Scalar> parseScalar(
    const rapidjson::GenericValue<rapidjson::UTF8<char>> &value) {
    return std::make_shared<Scalar>(value.GetString());
};

const std::shared_ptr<Enum> parseEnum(const JSONObjectEntry &entry) {
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

const Schema json::parser::parseSchema(const rapidjson::Document &document) {
    const auto &serverSchema = document["server"];
    TypeRegistry registry;
    Schema schema;
    schema.server.scalars = serverSchema["scalars"].GetArray() |
                            std::views::transform([&registry](const auto &el) {
                                const auto &scalar = parseScalar(el);
                                registry.addNode(scalar);
                                return std::make_pair(scalar->name, scalar);
                            }) |
                            std::ranges::to<std::map>();
    schema.server.enums = serverSchema["enums"].GetObject() |
                          std::views::transform([&registry](const auto &el) {
                              const auto &node = parseEnum(el);
                              registry.addNode(node);
                              return std::make_pair(node->name, node);
                          }) |
                          std::ranges::to<std::map>();
    for (const auto &obj : serverSchema["inputs"].GetObject()) {
        registry.addNode(std::make_shared<InputType>(obj.name.GetString()));
    };
    for (const auto &obj : serverSchema["objects"].GetObject()) {
        registry.addNode(std::make_shared<ObjectType>(obj.name.GetString()));
    };
    for (const auto &obj : serverSchema["interfaces"].GetObject()) {
        registry.addNode(std::make_shared<Interface>(obj.name.GetString()));
    };
    for (const auto &obj : serverSchema["unions"].GetObject()) {
        registry.addNode(std::make_shared<Union>(obj.name.GetString()));
    };

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

