#include "./shared.hpp"

#include <rapidjson/document.h>

#include <map>
#include <memory>
#include <ranges>
#include <string>
#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

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

::gql::parsers::schema::ast::InputFieldSpec parseInputFieldSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry) {
    const std::string &_type = value["_type"].GetString();
    if (_type == "literal") {
        return (::gql::parsers::schema::ast::LiteralFieldSpec<
                ::gql::parsers::schema::ast::InputTypeSpec>){
            .type = parseInputTypeSpec(value["type"], registry)
        };
    }
    return (::gql::parsers::schema::ast::ArrayFieldSpec<
            ::gql::parsers::schema::ast::InputTypeSpec>){
        .type = parseInputTypeSpec(value["type"], registry),
        .nullable = value["nullable"].GetBool()
    };
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
