#include "./server.hpp"

#include <map>
#include <memory>
#include <ranges>

#include "../shared/shared.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "rapidjson/document.h"

using namespace gql::parsers::schema;

namespace gql::json::parsers::schema::server {
ServerSchema parseSchema(TypeRegistry &registry,
                         const rapidjson::Document &document) {
    ServerSchema schema;
    schema.scalars = shared::parseNodesFromArray<std::shared_ptr<ast::Scalar>>(
        document["scalars"].GetArray(), shared::parseScalar, registry);
    schema.enums = shared::parseNodesFromObject<std::shared_ptr<ast::Enum>>(
        document["enums"].GetObject(), shared::parseEnum, registry);
    shared::addNodesToRegistry<ast::InputType>(document["inputs"].GetObject(),
                                               registry);
    shared::addNodesToRegistry<ast::ObjectType>(document["objects"].GetObject(),
                                                registry);
    shared::addNodesToRegistry<ast::Interface>(document["interfaces"].GetObject(),
                                               registry);
    shared::addNodesToRegistry<ast::Union>(document["unions"].GetObject(),
                                           registry);
    schema.inputs =
        document["inputs"].GetObject() |
        std::views::transform([&registry](const auto &obj) {
            const auto &input = registry.inputs.at(obj.name.GetString());
            input->fields =
                obj.value["fields"].GetObject() |
                std::views::transform([&registry](const auto &entry) {
                    const auto &field =
                        shared::parseInputFieldDefinition(entry, registry);
                    return std::make_pair(field.name, field);
                }) |
                std::ranges::to<std::map>();
            return std::make_pair(input->name, input);
        }) |
        std::ranges::to<std::map>();
    schema.objects =
        document["objects"].GetObject() |
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
            obj->fields =
                entry.value["fields"].GetObject() |
                std::views::transform([&registry](const auto &entry) {
                    const auto &field =
                        shared::parseObjectFieldDefinition(entry, registry);
                    return std::make_pair(field->name, field);
                }) |
                std::ranges::to<std::map>();
            return std::make_pair(obj->name, obj);
        }) |
        std::ranges::to<std::map>();
    schema.interfaces =
        document["interfaces"].GetObject() |
        std::views::transform([&registry](const auto &entry) {
            const auto &interface =
                registry.interfaces.at(entry.name.GetString());
            interface->fields =
                entry.value["fields"].GetObject() |
                std::views::transform([&registry](const auto &entry) {
                    const auto &field =
                        shared::parseObjectFieldDefinition(entry, registry);
                    return std::make_pair(field->name, field);
                }) |
                std::ranges::to<std::map>();
            return std::make_pair(interface->name, interface);
        }) |
        std::ranges::to<std::map>();
    schema.unions =
        document["unions"].GetObject() |
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
};  // namespace gql::json::parsers::schema::server
