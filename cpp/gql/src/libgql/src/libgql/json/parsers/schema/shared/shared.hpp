#pragma once
#include <rapidjson/document.h>

#include <functional>
#include <map>
#include <memory>
#include <ranges>
#include <string>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace gql::json::parsers::schema::shared {
template <typename T>
std::map<std::string, T> parseNodesFromArray(
    const JSONConstArray &array,
    const std::function<T(const JSONValue &)> &parserFunc,
    ::gql::parsers::schema::TypeRegistry &registry) {
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
    ::gql::parsers::schema::TypeRegistry &registry) {
    return array |
           std::views::transform([&registry, &parserFunc](const auto &el) {
               const auto &node = parserFunc(el);
               registry.addNode(node);
               return std::make_pair(node->name, node);
           }) |
           std::ranges::to<std::map>();
};

template <typename T>
void addNodesToRegistry(const JSONObject &object,
                        ::gql::parsers::schema::TypeRegistry &registry) {
    for (const auto &obj : object) {
        registry.addNode(std::make_shared<T>(obj.name.GetString()));
    };
};

std::shared_ptr<::gql::parsers::schema::ast::Scalar> parseScalar(
    const JSONValue &value);

std::shared_ptr<::gql::parsers::schema::ast::Enum> parseEnum(
    const JSONObjectEntry &entry);

::gql::parsers::schema::ast::InputTypeSpec parseInputTypeSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry);
::gql::parsers::schema::ast::InputFieldSpec parseInputFieldSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry);

::gql::parsers::schema::ast::FieldDefinition<
    ::gql::parsers::schema::ast::InputFieldSpec>
parseInputFieldDefinition(const JSONObjectEntry &entry,
                          const ::gql::parsers::schema::TypeRegistry &registry);
::gql::parsers::schema::ast::ObjectTypeSpec parseObjectTypeSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry);
::gql::parsers::schema::ast::NonCallableFieldSpec<
    ::gql::parsers::schema::ast::ObjectTypeSpec>
parseNonCallableFieldSpec(const JSONValue &value,
                          const ::gql::parsers::schema::TypeRegistry &registry);
::gql::parsers::schema::ast::ObjectFieldSpec parseObjectFieldSpec(
    const JSONValue &value,
    const ::gql::parsers::schema::TypeRegistry &registry);
std::shared_ptr<::gql::parsers::schema::ast::FieldDefinition<
    ::gql::parsers::schema::ast::ObjectFieldSpec>>
parseObjectFieldDefinition(
    const JSONObjectEntry &entry,
    const ::gql::parsers::schema::TypeRegistry &registry);
};  // namespace gql::json::parsers::schema::shared
