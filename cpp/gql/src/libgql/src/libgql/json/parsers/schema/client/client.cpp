#include "./client.hpp"

#include <cstdlib>
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

#include "../shared/shared.hpp"
#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "rapidjson/document.h"
#include "utils.hpp"

using namespace gql::parsers::schema;
using OpType = ::gql::parsers::file::client::ast::OpType;

namespace gql::json::parsers::schema::client {
std::map<std::string,
         std::shared_ptr<ast::FieldDefinition<ast::InputFieldSpec>>>
parseArguments(TypeRegistry &registry, const JSONObject &object) {
    return object |
           std::views::transform([&registry](const JSONObjectEntry &aEntry) {
               return std::make_pair(
                   std::string(aEntry.name.GetString()),
                   std::make_shared<ast::FieldDefinition<ast::InputFieldSpec>>(
                       shared::parseInputFieldDefinition(aEntry, registry)));
           }) |
           std::ranges::to<std::map>();
};

std::shared_ptr<ast::ClientDirective> parseClientDirective(
    TypeRegistry &registry, const JSONObject &object) {
    return std::make_shared<ast::ClientDirective>((ast::ClientDirective){
        .name = object["name"].GetString(),
        .arguments =
            parseArguments(registry, object["arguments"].GetObject()) });
};

OpType parseOpType(const std::string &value) {
    if (value == "QUERY") return OpType::QUERY;
    if (value == "MUTATION") return OpType::MUTATION;
    if (value == "SUBSCRIPTION") return OpType::SUBSCRIPTION;
    throw new std::runtime_error(std::format("Unknown OpType: {}", value));
};

ast::TypenameField parseTypenameField(const JSONObject &object) {
    std::optional<std::string> alias;
    if (object["alias"].IsString()) {
        alias = object["alias"].GetString();
    };
    return { .alias = alias };
};

ast::SpreadSelection parseSpreadSelection(const TypeRegistry &registry,
                                          const JSONObject &object) {
    return {
        .fragment = registry.fragments.at(object["fragment"].GetString()),
    };
};

ast::FragmentSpec parseFragmentSpec(const TypeRegistry &registry,
                                    const JSONObject &object);

ast::ArgumentRefValue parseArgumentRefValue(const JSONObject &object) {
    return { .name = object["name"].GetString() };
};

ast::ArgumentLiteralValue parseArgumentLiteralValue(
    const JSONObject &object,
    const std::shared_ptr<ast::FieldDefinition<ast::InputFieldSpec>> &spec) {
    const auto &typeSpec = std::visit(
        utils::overloaded{
            [](const ast::LiteralFieldSpec<ast::InputTypeSpec> &literal) {
                return literal.type;
            },
            [](const ast::ArrayFieldSpec<ast::InputTypeSpec> &array) {
                return array.type;
            },
        },
        spec->spec);
    const auto &jsonValue = object["value"];
    return std::visit<ast::ArgumentLiteralValue>(
        utils::overloaded{
            [&jsonValue](const std::shared_ptr<ast::Scalar> &scalar)
                -> ast::ArgumentLiteralValue {
                if (scalar->name == "String")
                    return std::string(jsonValue.GetString());
                if (scalar->name == "Int") return jsonValue.GetInt();
                if (scalar->name == "Float") return jsonValue.GetFloat();
                if (scalar->name == "Boolean") return jsonValue.GetBool();
                throw std::runtime_error(
                    "Custom scalars cant have default values");
            },
            [](const std::shared_ptr<ast::InputType> &input)
                -> ast::ArgumentLiteralValue {
                throw std::runtime_error(
                    "Argument with input type cant have default value");
            },
            [&jsonValue](const std::shared_ptr<ast::Enum> &enumType) {
                return (ast::ArgumentEnumValue){ .value =
                                                     jsonValue.GetString() };
            },
        },
        typeSpec);
};

ast::ArgumentValue parseArgumentValue(
    const JSONObject &object,
    const std::shared_ptr<ast::FieldDefinition<ast::InputFieldSpec>> &spec) {
    const std::string &type = object["_type"].GetString();
    if (type == "ref") {
        return parseArgumentRefValue(object);
    };
    return parseArgumentLiteralValue(object, spec);
};

ast::FieldSelectionArgument parseFieldSelectionArgument(
    const JSONObject &object,
    const std::shared_ptr<ast::FieldDefinition<ast::InputFieldSpec>> &spec) {
    return {
        .name = object["name"].GetString(),
        .value = parseArgumentValue(object["value"].GetObject(), spec),
        .type = spec,
    };
};

template <typename T>
ast::FieldSelection parseFieldSelection(const TypeRegistry &registry,
                                        const JSONObject &object,
                                        const std::shared_ptr<T> &objectType) {
    std::optional<std::shared_ptr<ast::FragmentSpec>> selection;
    if (object["selection"].IsObject()) {
        selection = std::make_shared<ast::FragmentSpec>(
            parseFragmentSpec(registry, object["selection"].GetObject()));
    };
    const std::string &name = object["name"].GetString();
    const auto &fieldSpec = objectType->fields[name]->spec;
    ast::FieldSelection fieldSelection = { .name = name,
                                           .alias = object["alias"].GetString(),
                                           .selection = selection };
    if (std::holds_alternative<ast::CallableFieldSpec>(fieldSpec)) {
        const ast::CallableFieldSpec &callableFieldSpec =
            std::get<ast::CallableFieldSpec>(fieldSpec);
        const auto &argumentsSpec = callableFieldSpec.arguments;
        fieldSelection.arguments =
            object["arguments"].GetObject() |
            std::views::transform(
                [&argumentsSpec](const JSONObjectEntry &aEntry) {
                    const std::string &name = aEntry.name.GetString();
                    return std::make_pair(name, parseFieldSelectionArgument(
                                                    aEntry.value.GetObject(),
                                                    argumentsSpec.at(name)));
                }) |
            std::ranges::to<std::map>();
    };
    return fieldSelection;
};

template <typename T>
ast::ObjectSelection parseObjectSelection(
    const TypeRegistry &registry, const JSONObject &object,
    const std::shared_ptr<T> &objectType) {
    const std::string &type = object["_type"].GetString();
    if (type == "TypenameField") {
        return parseTypenameField(object);
    } else if (type == "SpreadSelection") {
        return parseSpreadSelection(registry, object);
    };
    return parseFieldSelection(registry, object, objectType);
};

template <typename T>
ast::ObjectFragmentSpec<T> parseObjectFragmentSpec(
    const TypeRegistry &registry, const JSONObject &object,
    const std::shared_ptr<T> &type) {
    return { .type = type,
             .selections = object["selections"].GetArray() |
                           std::views::transform(
                               [&registry, &type](const auto &selection) {
                                   return parseObjectSelection(
                                       registry, selection.GetObject(), type);
                               }) |
                           std::ranges::to<std::vector>() };
};

ast::ObjectConditionalSpreadSelection parseObjectConditionalSpreadSelection(
    const TypeRegistry &registry, const JSONObject &object) {
    const auto &type = registry.objects.at(object["object"].GetString());
    return { .type = type,
             .selection =
                 std::make_shared<ast::ObjectFragmentSpec<ast::ObjectType>>(
                     parseObjectFragmentSpec<ast::ObjectType>(
                         registry, object["spec"].GetObject(), type)) };
};

ast::UnionSelection parseUnionSelectionNode(const TypeRegistry &registry,
                                            const JSONObject &object) {
    const std::string &type = object["_type"].GetString();
    if (type == "TypenameField") {
        return parseTypenameField(object);
    } else if (type == "SpreadSelection") {
        return parseSpreadSelection(registry, object);
    } else if (type == "ObjectConditionalSpreadSelection") {
        return parseObjectConditionalSpreadSelection(registry, object);
    };
    throw std::runtime_error(
        std::format("Unknown UnionSelectionNode \"_type\": {}", type));
};

ast::UnionFragmentSpec parseUnionFragmentSpec(const TypeRegistry &registry,
                                              const JSONObject &object) {
    return { .type = registry.unions.at(object["name"].GetString()),
             .selections =
                 object["selections"].GetArray() |
                 std::views::transform([&registry](const auto &selection) {
                     return parseUnionSelectionNode(registry,
                                                    selection.GetObject());
                 }) |
                 std::ranges::to<std::vector>() };
};

ast::FragmentSpec parseFragmentSpec(const TypeRegistry &registry,
                                    const JSONObject &object) {
    const std::string &type = object["_type"].GetString();
    if (type == "UnionFragmentSpec") {
        return parseUnionFragmentSpec(registry, object);
    }
    return parseObjectFragmentSpec(
        registry, object, registry.objects.at(object["name"].GetString()));
};

std::shared_ptr<ast::Operation> parseOperation(TypeRegistry &registry,
                                               const JSONObject &object) {
    return std::make_shared<ast::Operation>((ast::Operation){
        .type = parseOpType(object["type"].GetString()),
        .name = object["name"].GetString(),
        .parameters = object["parameters"].GetObject() |
                      std::views::transform([&registry](const auto &param) {
                          return std::make_pair(
                              std::string(param.name.GetString()),
                              shared::parseInputFieldDefinition(param,
                                                                registry));
                      }) |
                      std::ranges::to<std::map>(),
        .fragmentSpec =
            parseFragmentSpec(registry, object["fragmentSpec"].GetObject()),
        .sourceText = object["sourceText"].GetString(),
        .parametersHash =
            std::strtoul(object["parametersHash"].GetString(), nullptr, 10),
        .fragmentSpecHash = std::strtoul(object["fragmentSpecHash"].GetString(),
                                         nullptr, 10) });
};

void addFragmentsToTypeRegistry(TypeRegistry &registry,
                                const JSONObject &object) {
    for (const auto &[key, value] : object) {
        registry.fragments[key.GetString()] =
            std::make_shared<ast::Fragment>(key.GetString());
    };
};

void parseFragment(const TypeRegistry &registry,
                   const std::shared_ptr<ast::Fragment> &fragment,
                   const JSONObject &object) {
    fragment->sourceText = object["sourceText"].GetString();
    fragment->spec = parseFragmentSpec(registry, object["spec"].GetObject());
};

ClientSchema parseSchema(TypeRegistry &registry,
                         const rapidjson::Document &document) {
    ClientSchema schema;
    schema.directives =
        document["directives"].GetObject() |
        std::views::transform([&registry](const auto &directive) {
            return std::make_pair(
                std::string(directive.name.GetString()),
                parseClientDirective(registry, directive.value.GetObject()));
        }) |
        std::ranges::to<std::map>();
    const auto &fragmentsObject = document["fragments"].GetObject();
    addFragmentsToTypeRegistry(registry, fragmentsObject);
    schema.fragments =
        fragmentsObject | std::views::transform([&registry](const auto &obj) {
            const auto &fragment = registry.fragments.at(obj.name.GetString());
            parseFragment(registry, fragment, obj.value.GetObject());
            return std::make_pair(fragment->name, fragment);
        }) |
        std::ranges::to<std::map>();
    schema.operations =
        document["operations"].GetObject() |
        std::views::transform([&registry](const auto &obj) {
            return std::make_pair(
                std::string(obj.name.GetString()),
                parseOperation(registry, obj.value.GetObject()));
        }) |
        std::ranges::to<std::map>();
    return schema;
};
};  // namespace gql::json::parsers::schema::client
