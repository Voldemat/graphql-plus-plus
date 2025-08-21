#include "./shared.hpp"

#include <rapidjson/document.h>

#include <algorithm>
#include <format>
#include <map>
#include <memory>
#include <optional>
#include <ranges>
#include <stdexcept>
#include <string>
#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "magic_enum.hpp"
#include "rapidjson/rapidjson.h"

namespace gql::json::parsers::schema::shared {
std::shared_ptr<::gql::parsers::schema::ast::Scalar> parseScalar(
    const JSONValue &value) {
    return std::make_shared<::gql::parsers::schema::ast::Scalar>(
        value.GetString());
};

std::shared_ptr<::gql::parsers::schema::ast::Enum> parseEnum(
    const JSONObjectEntry &entry) {
    return std::make_shared<::gql::parsers::schema::ast::Enum>(
        entry.name.GetString(),
        entry.value["values"].GetArray() |
            std::views::transform([](const auto &item) -> std::string {
                return item.GetString();
            }) |
            std::ranges::to<std::vector>());
};

::gql::parsers::schema::ast::InputTypeSpec parseInputTypeSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    const std::string &name = value["name"].GetString();
    if (_type == "Scalar") {
        return registry.scalars.at(name);
    } else if (_type == "Enum") {
        return registry.enums.at(name);
    }
    return registry.inputs.at(name);
};

std::optional<::gql::parsers::schema::ast::Literal> parseLiteral(
    const JSONValue &value) {
    if (value.IsNull()) return std::nullopt;
    switch (value.GetType()) {
        case rapidjson::kNullType: {
            return std::nullopt;
        }
        case rapidjson::kNumberType: {
            if (value.IsFloat()) return value.GetFloat();
            return value.GetInt();
        }
        case rapidjson::kStringType: {
            return value.GetString();
        }
        case rapidjson::kTrueType:
        case rapidjson::kFalseType: {
            return value.GetBool();
        }
        default: {
            throw std::runtime_error(
                std::format("Unknown type for literal: {}",
                            magic_enum::enum_name(value.GetType())));
        }
    };
};

template <typename T, typename JSONT=T>
std::vector<T> parseJSONArray(const JSONConstArray &array,
                              const std::vector<rapidjson::Type> &jsonTypes) {
    std::vector<T> elements(array.Size());
    for (const auto &value : array) {
        if (std::ranges::find(jsonTypes.begin(), jsonTypes.end(),
                              value.GetType()) == jsonTypes.end())
            throw std::runtime_error(
                "Array literal elements must be of the same type");
        elements.emplace_back(value.Get<JSONT>());
    };
    return elements;
};

std::optional<::gql::parsers::schema::ast::ArrayLiteral> parseArrayLiteral(
    const JSONValue &value) {
    if (value.IsNull()) return std::nullopt;
    if (!value.IsArray()) {
        throw std::runtime_error("Array literal must be an array");
    };
    const auto &jsonArray = value.GetArray();
    if (jsonArray.Size() == 0) {
        throw std::runtime_error(
            "Array literal must have at least one element");
    };
    const auto &type = jsonArray[0].GetType();
    switch (type) {
        case rapidjson::kStringType:
            return parseJSONArray<std::string, const char*>(jsonArray, { type });
        case rapidjson::kNumberType:
            if (jsonArray[0].IsFloat())
                return parseJSONArray<float>(jsonArray, { type });
            return parseJSONArray<int>(jsonArray, { type });
        case rapidjson::kFalseType:
        case rapidjson::kTrueType:
            return parseJSONArray<bool>(jsonArray, { type });
        default: {
            throw std::runtime_error(
                std::format("Unsupported array literal type: {}",
                            magic_enum::enum_name(type)));
        }
    };
};

::gql::parsers::schema::ast::InputFieldSpec parseInputFieldSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        ::gql::parsers::schema::ast::LiteralFieldSpec<
            ::gql::parsers::schema::ast::InputTypeSpec>
            result = { .type = parseInputTypeSpec(value["type"], registry) };
        result.defaultValue = parseLiteral(value["defaultValue"]);
        return result;
    }
    ::gql::parsers::schema::ast::ArrayFieldSpec<
        ::gql::parsers::schema::ast::InputTypeSpec>
        result = { .type = parseInputTypeSpec(value["type"], registry),
                   .nullable = value["nullable"].GetBool() };
    result.defaultValue = parseArrayLiteral(value["defaultValue"]);
    return result;
};

::gql::parsers::schema::ast::FieldDefinition<
    ::gql::parsers::schema::ast::InputFieldSpec>
parseInputFieldDefinition(
    const JSONObjectEntry &entry,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    return { .name = entry.name.GetString(),
             .spec = parseInputFieldSpec(entry.value["spec"], registry),
             .nullable = entry.value["nullable"].GetBool() };
};

::gql::parsers::schema::ast::ObjectTypeSpec parseObjectTypeSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry) {
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

::gql::parsers::schema::ast::NonCallableFieldSpec<
    ::gql::parsers::schema::ast::ObjectTypeSpec>
parseNonCallableFieldSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        return (::gql::parsers::schema::ast::LiteralFieldSpec<
                ::gql::parsers::schema::ast::ObjectTypeSpec>){
            .type = parseObjectTypeSpec(value["type"], registry)
        };
    }
    return (::gql::parsers::schema::ast::ArrayFieldSpec<
            ::gql::parsers::schema::ast::ObjectTypeSpec>){
        .type = parseObjectTypeSpec(value["type"], registry),
        .nullable = value["nullable"].GetBool(),
    };
};

::gql::parsers::schema::ast::ObjectFieldSpec parseObjectFieldSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        return (::gql::parsers::schema::ast::LiteralFieldSpec<
                ::gql::parsers::schema::ast::ObjectTypeSpec>){
            .type = parseObjectTypeSpec(value["type"], registry)
        };
    } else if (_type == "array") {
        return (::gql::parsers::schema::ast::ArrayFieldSpec<
                ::gql::parsers::schema::ast::ObjectTypeSpec>){
            .type = parseObjectTypeSpec(value["type"], registry),
            .nullable = value["nullable"].GetBool()
        };
    };
    return (::gql::parsers::schema::ast::CallableFieldSpec){
        .returnType = parseNonCallableFieldSpec(value["returnType"], registry),
        .arguments = value["arguments"].GetObject() |
                     std::views::transform([&registry](const auto &entry) {
                         const auto &field = std::make_shared<
                             ::gql::parsers::schema::ast::FieldDefinition<
                                 ::gql::parsers::schema::ast::InputFieldSpec>>(
                             parseInputFieldDefinition(entry, registry));
                         return std::make_pair(field->name, field);
                     }) |
                     std::ranges::to<std::map>()
    };
};

std::shared_ptr<::gql::parsers::schema::ast::FieldDefinition<
    ::gql::parsers::schema::ast::ObjectFieldSpec>>
parseObjectFieldDefinition(
    const JSONObjectEntry &entry,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    return std::make_shared<::gql::parsers::schema::ast::FieldDefinition<
        ::gql::parsers::schema::ast::ObjectFieldSpec>>(
        entry.name.GetString(),
        parseObjectFieldSpec(entry.value["spec"], registry),
        entry.value["nullable"].GetBool());
};
};  // namespace gql::json::parsers::schema::shared
