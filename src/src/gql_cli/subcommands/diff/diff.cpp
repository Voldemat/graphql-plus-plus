#include "./diff.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <algorithm>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <ranges>
#include <sstream>
#include <string>
#include <variant>
#include <vector>

#include "HTTPRequest.hpp"
#include "gql_cli/utils.hpp"
#include "libgql/json/introspection/parser.hpp"
#include "libgql/json/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "utils.hpp"

using namespace parsers::schema;
using namespace parsers::schema::ast;

const char *INTROSPECTION_QUERY =
#include "./query.data"

    rapidjson::Document getIntrospectionDocument(const std::string &urlToApi) {
    http::HeaderFields headers{ { "Accept", "application/json" },
                                { "Content-Type", "application/json" } };
    http::Request request{ urlToApi };
    const auto &response = request.send("POST", INTROSPECTION_QUERY, headers);
    if (response.status.code != http::Status::Ok) {
        std::cerr << std::format("Expected 200 status code, while received {}",
                                 response.status.code)
                  << std::endl;
        throw CLI::RuntimeError(1);
    };
    std::string buffer{ response.body.begin(), response.body.end() };
    rapidjson::Document d;
    d.Parse(buffer.c_str());
    return d;
};

rapidjson::Document getDocumentFromSchemaJson(const std::string &pathToSchema) {
    std::string buffer;
    if (pathToSchema == "-") {
        buffer = getAllStdin();
    } else {
        if (!std::filesystem::exists(pathToSchema)) {
            std::cerr << std::format("Path \"{}\" does not exists",
                                     pathToSchema)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (!std::filesystem::is_regular_file(
                std::filesystem::status(pathToSchema))) {
            std::cerr << std::format("Path {} is not regular file",
                                     pathToSchema)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::ifstream file(pathToSchema);
        std::stringstream fileStream;
        fileStream << file.rdbuf();
        buffer = fileStream.str();
    };
    rapidjson::Document d;
    d.Parse(buffer.c_str());
    return d;
};

bool compareTwoTypeSpecs(const ObjectTypeSpec &spec,
                         const ObjectTypeSpec &spec2, const std::string &path) {
    if (spec.index() != spec2.index()) return false;
    return std::visit(
        overloaded{
            [&spec2](const std::shared_ptr<ObjectType> &node) {
                const auto &node2 =
                    std::get<std::shared_ptr<ObjectType>>(spec2);
                return node->name == node2->name;
            },
            [&spec2](const std::shared_ptr<Interface> &node) {
                const auto &node2 = std::get<std::shared_ptr<Interface>>(spec2);
                return node->name == node2->name;
            },
            [&spec2](const std::shared_ptr<Scalar> &node) {
                const auto &node2 = std::get<std::shared_ptr<Scalar>>(spec2);
                return node->name == node2->name;
            },
            [&spec2](const std::shared_ptr<Union> &node) {
                const auto &node2 = std::get<std::shared_ptr<Union>>(spec2);
                return node->name == node2->name;
            },
            [&spec2](const std::shared_ptr<Enum> &node) {
                const auto &node2 = std::get<std::shared_ptr<Enum>>(spec2);
                return node->name == node2->name;
            },
        },
        spec);
};

bool compareTwoTypeSpecs(const InputTypeSpec &spec,
                         const InputTypeSpec &spec2) {
    if (spec.index() != spec2.index()) return false;
    return std::visit(
        overloaded{
            [&spec2](const std::shared_ptr<Scalar> &node) {
                const auto &node2 = std::get<std::shared_ptr<Scalar>>(spec2);
                return node->name == node2->name;
            },
            [&spec2](const std::shared_ptr<Enum> &node) {
                const auto &node2 = std::get<std::shared_ptr<Enum>>(spec2);
                return node->name == node2->name;
            },
            [&spec2](const std::shared_ptr<InputType> &node) {
                const auto &node2 = std::get<std::shared_ptr<InputType>>(spec2);
                return node->name == node2->name;
            },
        },
        spec);
};

bool compareTwoFieldTypeSpecs(const NonCallableFieldSpec<ObjectTypeSpec> &spec,
                              const NonCallableFieldSpec<ObjectTypeSpec> &spec2,
                              const std::string &path) {
    if (spec.index() != spec2.index()) {
        std::cerr << std::format("[{}] Change type from {} to {}", path,
                                 spec.index() == 0 ? "Literal" : "Array",
                                 spec2.index() == 0 ? "Literal" : "Array")
                  << std::endl;
        return false;
    };
    return std::visit(
        overloaded{
            [&spec2, &path](const LiteralFieldSpec<ObjectTypeSpec> &node) {
                const auto &node2 =
                    std::get<LiteralFieldSpec<ObjectTypeSpec>>(spec2);
                return compareTwoTypeSpecs(node.type, node2.type, path);
            },
            [&spec2, &path](const ArrayFieldSpec<ObjectTypeSpec> &node) {
                const auto &node2 =
                    std::get<ArrayFieldSpec<ObjectTypeSpec>>(spec2);
                if (!node.nullable && node2.nullable) {
                    std::cerr << std::format("[{}] Became nullable", path)
                              << std::endl;
                    return false;
                };
                return compareTwoTypeSpecs(node.type, node2.type, path);
            },
        },
        spec);
};

bool compareTwoFieldDefinitions(const FieldDefinition<InputFieldSpec> &field,
                                const FieldDefinition<InputFieldSpec> &field2,
                                const std::string &typeName);
bool compareArguments(
    const std::map<std::string,
                   std::shared_ptr<FieldDefinition<InputFieldSpec>>> &arguments,
    const std::map<std::string,
                   std::shared_ptr<FieldDefinition<InputFieldSpec>>>
        &arguments2,
    const std::string &path) {
    bool isValid = true;
    for (const auto &[name, field] : arguments) {
        if (!arguments2.contains(name)) {
            std::cerr << std::format("[{}] Removed argument {}", path, name)
                      << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoFieldDefinitions(*field, *arguments2.at(name),
                                        path + ":args")) {
            isValid = false;
        };
    };
    return isValid;
};

std::string getFieldSpecName(const ObjectFieldSpec &spec) {
    return std::visit(
        overloaded{
            [](const LiteralFieldSpec<ObjectTypeSpec> &) { return "Literal"; },
            [](const ArrayFieldSpec<ObjectTypeSpec> &) { return "Array"; },
            [](const CallableFieldSpec &) { return "Callable"; },
        },
        spec);
};

bool compareTwoFieldTypeSpecs(const ObjectFieldSpec &spec,
                              const ObjectFieldSpec &spec2,
                              const std::string &path) {
    if (spec.index() != spec2.index()) {
        const auto &specName = getFieldSpecName(spec);
        const auto &spec2Name = getFieldSpecName(spec2);
        std::cerr << std::format("[{}] Change type from {} to {}", path,
                                 specName, spec2Name)
                  << std::endl;
        return false;
    };
    return std::visit(
        overloaded{
            [&spec2, &path](const LiteralFieldSpec<ObjectTypeSpec> &node) {
                const auto &node2 =
                    std::get<LiteralFieldSpec<ObjectTypeSpec>>(spec2);
                return compareTwoTypeSpecs(node.type, node2.type, path);
            },
            [&spec2, &path](const ArrayFieldSpec<ObjectTypeSpec> &node) {
                const auto &node2 =
                    std::get<ArrayFieldSpec<ObjectTypeSpec>>(spec2);
                if (!node.nullable && node2.nullable) return false;
                return compareTwoTypeSpecs(node.type, node2.type, path);
            },
            [&spec2, &path](const CallableFieldSpec &node) {
                const auto &node2 = std::get<CallableFieldSpec>(spec2);
                return compareTwoFieldTypeSpecs(node.returnType,
                                                node2.returnType,
                                                path + ".returnType") &&
                       compareArguments(node.arguments, node2.arguments, path);
            } },
        spec);
};

bool compareTwoFieldTypeSpecs(const InputFieldSpec &spec,
                              const InputFieldSpec &spec2,
                              const std::string &path) {
    if (spec.index() != spec2.index()) return false;
    return std::visit(
        overloaded{
            [&spec2](const LiteralFieldSpec<InputTypeSpec> &node) {
                const auto &node2 =
                    std::get<LiteralFieldSpec<InputTypeSpec>>(spec2);
                return compareTwoTypeSpecs(node.type, node2.type);
            },
            [&spec2](const ArrayFieldSpec<InputTypeSpec> &node) {
                const auto &node2 =
                    std::get<ArrayFieldSpec<InputTypeSpec>>(spec2);
                if (!node.nullable && node2.nullable) return false;
                return compareTwoTypeSpecs(node.type, node2.type);
            },
        },
        spec);
};

bool compareTwoFieldDefinitions(const FieldDefinition<InputFieldSpec> &field,
                                const FieldDefinition<InputFieldSpec> &field2,
                                const std::string &typeName) {
    bool isValid = true;
    if (field.nullable && !field2.nullable) {
        std::cerr << std::format("[{}.{}] Removed nullability", typeName,
                                 field.name)
                  << std::endl;
        isValid = false;
    };
    if (!compareTwoFieldTypeSpecs(field.spec, field2.spec,
                                  std::format("{}.{}", typeName, field.name))) {
        std::cerr << std::format("[{}.{}] Changed type spec", typeName,
                                 field.name)
                  << std::endl;
        isValid = false;
    };
    return isValid;
};

template <typename T>
bool compareTwoFieldDefinitions(const FieldDefinition<T> &field,
                                const FieldDefinition<T> &field2,
                                const std::string &typeName) {
    bool isValid = true;
    if (!field.nullable && field2.nullable) {
        std::cerr << std::format("[{}.{}] Became nullable", typeName,
                                 field.name)
                  << std::endl;
        isValid = false;
    };
    if (!compareTwoFieldTypeSpecs(field.spec, field2.spec,
                                  std::format("{}.{}", typeName, field.name))) {
        std::cerr << std::format("[{}.{}] Changed type spec", typeName,
                                 field.name)
                  << std::endl;
        isValid = false;
    };
    return isValid;
};

bool compareTwoObjects(const std::shared_ptr<ObjectType> &object,
                       const std::shared_ptr<ObjectType> &object2) {
    bool isValid = true;
    for (const auto &name : object->implements | std::views::keys) {
        if (!object2->implements.contains(name)) {
            std::cerr << std::format(
                             "[{}] Removed interface {} from extends list",
                             object->name, name)
                      << std::endl;
            isValid = false;
        };
    };
    for (const auto &[name, field] : object->fields) {
        if (!object2->fields.contains(name)) {
            std::cerr << std::format("[{}] Deleted field {}", object->name,
                                     name)
                      << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoFieldDefinitions(*field, *object2->fields.at(name),
                                        object->name)) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareObjects(
    const std::map<std::string, std::shared_ptr<ObjectType>> &objects,
    const std::map<std::string, std::shared_ptr<ObjectType>> &objects2) {
    bool isValid = true;
    for (const auto &[name, object] : objects) {
        if (!objects2.contains(name)) {
            std::cerr << std::format("Deleted object {}", name) << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoObjects(object, objects2.at(name))) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareTwoUnions(const std::shared_ptr<Union> &node,
                      const std::shared_ptr<Union> &node2) {
    bool isValid = true;
    for (const auto &name : node->items | std::views::keys) {
        if (!node2->items.contains(name)) {
            std::cerr << std::format("[{}] Removed type {}", node->name, name)
                      << std::endl;
            isValid = false;
        };
    };
    return isValid;
};

bool compareUnions(
    const std::map<std::string, std::shared_ptr<Union>> &unions,
    const std::map<std::string, std::shared_ptr<Union>> &unions2) {
    bool isValid = true;
    for (const auto &[name, unionNode] : unions) {
        if (!unions2.contains(name)) {
            std::cerr << std::format("Deleted union {}", name) << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoUnions(unionNode, unions2.at(name))) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareTwoInputs(const std::shared_ptr<InputType> &input,
                      const std::shared_ptr<InputType> &input2) {
    bool isValid = true;
    for (const auto &[name, field] : input->fields) {
        if (!input2->fields.contains(name)) {
            std::cerr << std::format("[{}] Deleted field {}", input->name, name)
                      << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoFieldDefinitions(field, input2->fields.at(name),
                                        input->name)) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareInputs(
    const std::map<std::string, std::shared_ptr<InputType>> &inputs,
    const std::map<std::string, std::shared_ptr<InputType>> &inputs2) {
    bool isValid = true;
    for (const auto &[name, input] : inputs) {
        if (!inputs2.contains(name)) {
            std::cerr << std::format("Deleted input {}", name) << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoInputs(input, inputs2.at(name))) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareTwoInterfaces(const std::shared_ptr<Interface> &interface,
                          const std::shared_ptr<Interface> &interface2) {
    bool isValid = true;
    for (const auto &[name, field] : interface->fields) {
        if (!interface2->fields.contains(name)) {
            std::cerr << std::format("Deleted field {} in interface {}", name,
                                     interface->name)
                      << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoFieldDefinitions(*field, *interface2->fields.at(name),
                                        interface->name)) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareInterfaces(
    const std::map<std::string, std::shared_ptr<Interface>> &interfaces,
    const std::map<std::string, std::shared_ptr<Interface>> &interfaces2) {
    bool isValid = true;
    for (const auto &[name, interface] : interfaces) {
        if (!interfaces2.contains(name)) {
            std::cerr << std::format("Deleted interface {}", name) << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoInterfaces(interface, interfaces2.at(name))) {
            isValid = false;
        };
    };
    return isValid;
};

bool compareScalars(
    const std::map<std::string, std::shared_ptr<Scalar>> &scalars,
    const std::map<std::string, std::shared_ptr<Scalar>> &scalars2) {
    bool isValid = true;
    for (const auto &name : scalars | std::views::keys) {
        if (!scalars2.contains(name)) {
            std::cerr << std::format("Deleted scalar {}", name) << std::endl;
            isValid = false;
        };
    };
    return isValid;
};

bool compareTwoEnums(const std::shared_ptr<Enum> &node,
                     const std::shared_ptr<Enum> &node2) {
    bool isValid = true;
    for (const auto &value : node->values) {
        if (std::find(node2->values.begin(), node2->values.end(), value) ==
            node2->values.end()) {
            std::cerr << std::format("Removed {} value from enum {}", value,
                                     node->name)
                      << std::endl;
        };
    };
    return isValid;
};

bool compareEnums(const std::map<std::string, std::shared_ptr<Enum>> &enums,
                  const std::map<std::string, std::shared_ptr<Enum>> &enums2) {
    bool isValid = true;
    for (const auto &[name, enumNode] : enums) {
        if (!enums2.contains(name)) {
            std::cerr << std::format("Deleted enum {}", name) << std::endl;
            isValid = false;
            continue;
        };
        if (!compareTwoEnums(enumNode, enums2.at(name))) {
            isValid = false;
        };
    };
    return isValid;
};

void findDifferenceBetweenSchemas(
    const parsers::schema::ServerSchema &schema,
    const parsers::schema::ServerSchema &schema2) {
    bool isObjectsValid = compareObjects(schema.objects, schema2.objects);
    bool isUnionsValid = compareUnions(schema.unions, schema2.unions);
    bool isInputsValid = compareInputs(schema.inputs, schema2.inputs);
    bool isInterfacesValid =
        compareInterfaces(schema.interfaces, schema2.interfaces);
    bool isScalarsValid = compareScalars(schema.scalars, schema2.scalars);
    bool isEnumsValid = compareEnums(schema.enums, schema2.enums);
    bool isValid = (isObjectsValid && isUnionsValid && isInputsValid &&
                    isInterfacesValid && isScalarsValid && isEnumsValid);
    if (!isValid) {
        std::cerr << "Schema is incompatible" << std::endl;
        throw CLI::RuntimeError(1);
    };
    std::cout << "Schema is compatible" << std::endl;
};

void createDifSubcommand(CLI::App *app) {
    CLI::App *diffCmd = app->add_subcommand("diff", "Diff between two schemas");
    CLI::App *diffParseCmd = diffCmd->add_subcommand(
        "parse", "Parse input stream into tokens in json format");

    std::shared_ptr<std::string> pathToSchema = std::make_shared<std::string>();
    std::shared_ptr<std::string> urlToApi = std::make_shared<std::string>();
    diffParseCmd
        ->add_option("--path-to-schema", *pathToSchema,
                     "Path to schema json file")
        ->required();
    diffParseCmd->add_option("--url-to-api", *urlToApi, "Url to api")
        ->required();
    diffParseCmd->callback([pathToSchema, urlToApi]() {
        const auto &schemaDocument = getDocumentFromSchemaJson(*pathToSchema);
        const auto &schema = json::parsers::schema::parseSchema(schemaDocument);
        const auto &introspectionDocument = getIntrospectionDocument(*urlToApi);
        const auto &secondSchema =
            json::parser::introspection::parseIntrospectionSchema(
                introspectionDocument);
        findDifferenceBetweenSchemas(schema.server, secondSchema);
    });
};
