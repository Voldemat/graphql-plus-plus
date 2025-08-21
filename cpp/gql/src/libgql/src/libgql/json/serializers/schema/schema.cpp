#include "./schema.hpp"

#include <rapidjson/encodings.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/rapidjson.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <format>
#include <memory>
#include <optional>
#include <ranges>
#include <set>
#include <string>
#include <variant>
#include <vector>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/uses/uses.hpp"
#include "magic_enum.hpp"
#include "utils.hpp"

using namespace rapidjson;
using namespace gql::parsers::schema;
using namespace gql::parsers::schema::ast;

namespace gql::json::serializers::schema {
void writeTypeSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                   const ObjectTypeSpec &field) {
    writer.StartObject();
    std::visit(
        utils::overloaded{
            [&writer](const std::shared_ptr<ObjectType> &node) {
                writer.String("_type");
                writer.String("ObjectType");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("$ref");
                writer.String(
                    std::format("#/server/objects/{}", node->name).c_str());
            },
            [&writer](const std::shared_ptr<Interface> &node) {
                writer.String("_type");
                writer.String("InterfaceType");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("$ref");
                writer.String(
                    std::format("#/server/interfaces/{}", node->name).c_str());
            },
            [&writer](const std::shared_ptr<Scalar> &node) {
                writer.String("_type");
                writer.String("Scalar");
                writer.String("name");
                writer.String(node->name.c_str());
            },
            [&writer](const std::shared_ptr<Enum> &node) {
                writer.String("_type");
                writer.String("Enum");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("$ref");
                writer.String(
                    std::format("#/server/enums/{}", node->name).c_str());
            },
            [&writer](const std::shared_ptr<Union> &node) {
                writer.String("_type");
                writer.String("Union");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("$ref");
                writer.String(
                    std::format("#/server/unions/{}", node->name).c_str());
            },
        },
        field);
    writer.EndObject();
};

void writeTypeSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                   const InputTypeSpec &field) {
    writer.StartObject();
    std::visit(
        utils::overloaded{
            [&writer](const std::shared_ptr<InputType> &node) {
                writer.String("_type");
                writer.String("InputType");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("$ref");
                writer.String(
                    std::format("#/server/inputs/{}", node->name).c_str());
            },
            [&writer](const std::shared_ptr<Scalar> &node) {
                writer.String("_type");
                writer.String("Scalar");
                writer.String("name");
                writer.String(node->name.c_str());
            },
            [&writer](const std::shared_ptr<Enum> &node) {
                writer.String("_type");
                writer.String("Enum");
                writer.String("name");
                writer.String(node->name.c_str());
                writer.String("$ref");
                writer.String(
                    std::format("#/server/enums/{}", node->name).c_str());
            },
        },
        field);
    writer.EndObject();
};

void writeDefaultValue(JSONWriter &writer,
                       const std::optional<const ast::Literal> &literal) {
    writer.String("defaultValue");
    if (!literal.has_value()) {
        writer.Null();
        return;
    };
    std::visit(utils::overloaded{
                   [&writer](const int &value) { writer.Int(value); },
                   [&writer](const float &value) {
                       const auto &rawValue = std::to_string(value);
                       writer.RawNumber(rawValue.data(), rawValue.size());
                   },
                   [&writer](const bool &value) { writer.Bool(value); },
                   [&writer](const std::string &value) {
                       writer.String(value.c_str());
                   },
               },
               literal.value());
};

void writeDefaultValue(JSONWriter &writer,
                       const std::optional<const ast::ArrayLiteral> &literal) {
    writer.String("defaultValue");
    if (!literal.has_value()) {
        writer.Null();
        return;
    };
    writer.StartArray();
    std::visit(utils::overloaded{
                   [&writer](const std::vector<int> &array) {
                       for (const auto &value : array) {
                           writer.Int(value);
                       };
                   },
                   [&writer](const std::vector<float> &array) {
                       for (const auto &value : array) {
                           const auto &rawValue = std::to_string(value);
                           writer.RawNumber(rawValue.data(), rawValue.size());
                       };
                   },
                   [&writer](const std::vector<bool> &array) {
                       for (const auto &value : array) {
                           writer.Bool(value);
                       };
                   },
                   [&writer](const std::vector<std::string> &array) {
                       for (const auto &value : array) {
                           writer.String(value.c_str());
                       };
                   },
               },
               literal.value());
    writer.EndArray();
};

template <typename T>
void writeFieldSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                    const NonCallableFieldSpec<T> &field) {
    writer.StartObject();
    std::visit(utils::overloaded{
                   [&writer](const LiteralFieldSpec<T> &node) {
                       writer.String("_type");
                       writer.String("literal");
                       writer.String("type");
                       writeTypeSpec(writer, node.type);
                       if constexpr (std::is_same_v<T, InputTypeSpec>) {
                           writeDefaultValue(writer, node.defaultValue);
                       };
                   },
                   [&writer](const ArrayFieldSpec<T> &node) {
                       writer.String("_type");
                       writer.String("array");
                       writer.String("nullable");
                       writer.Bool(node.nullable);
                       writer.String("type");
                       writeTypeSpec(writer, node.type);
                       if constexpr (std::is_same_v<T, InputTypeSpec>) {
                           writeDefaultValue(writer, node.defaultValue);
                       };
                   },
               },
               field);
    writer.EndObject();
};

template <typename T>
void writeFieldDefinition(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                          const FieldDefinition<T> &field);

void writeArgumentLiteralValue(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const ArgumentLiteralValue &argValue) {
    std::visit(utils::overloaded{
                   [&writer](const std::string &value) {
                       writer.String(value.c_str());
                   },
                   [&writer](const int &value) { writer.Int(value); },
                   [&writer](const float &value) { writer.Double(value); },
                   [&writer](const bool &value) { writer.Bool(value); },
                   [&writer](const ArgumentEnumValue &value) {
                       writer.String(value.value.c_str());
                   },
               },
               argValue);
};

void writeArgumentValue(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                        const ArgumentValue &argValue) {
    writer.StartObject();
    std::visit(utils::overloaded{
                   [&writer](const ArgumentLiteralValue &value) {
                       writer.String("_type");
                       writer.String("literal");
                       writer.String("value");
                       writeArgumentLiteralValue(writer, value);
                   },
                   [&writer](const ArgumentRefValue &value) {
                       writer.String("_type");
                       writer.String("ref");
                       writer.String("name");
                       writer.String(value.name.c_str());
                   },
               },
               argValue);
    writer.EndObject();
};

void writeFieldSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                    const ObjectFieldSpec &field) {
    writer.StartObject();
    std::visit(
        utils::overloaded{
            [&writer](const LiteralFieldSpec<ObjectTypeSpec> &node) {
                writer.String("_type");
                writer.String("literal");
                writer.String("type");
                writeTypeSpec(writer, node.type);
                writer.String("invocations");
                writer.StartObject();
                for (const auto &invocation : node.invocations) {
                    writer.String(invocation.directive->name.c_str());
                    writer.StartObject();
                    writer.String("arguments");
                    writer.StartObject();
                    for (const auto &[name, arg] : invocation.arguments) {
                        writer.String(name.c_str());
                        writer.StartObject();
                        writer.String("value");
                        writeArgumentValue(writer, arg.value);
                        writer.EndObject();
                    };
                    writer.EndObject();
                    writer.EndObject();
                };
                writer.EndObject();
            },
            [&writer](const ArrayFieldSpec<ObjectTypeSpec> &node) {
                writer.String("_type");
                writer.String("array");
                writer.String("nullable");
                writer.Bool(node.nullable);
                writer.String("type");
                writeTypeSpec(writer, node.type);
            },
            [&writer](const CallableFieldSpec &node) {
                writer.String("_type");
                writer.String("callable");
                writer.String("returnType");
                writeFieldSpec(writer, node.returnType);
                writer.String("arguments");
                writer.StartObject();
                for (const auto &arg : node.arguments | std::views::values) {
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
void writeFieldDefinition(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                          const FieldDefinition<T> &field) {
    writer.StartObject();
    writer.String("nullable");
    writer.Bool(field.nullable);
    writer.String("spec");
    writeFieldSpec(writer, field.spec);
    writer.EndObject();
};

void writeServerType(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                     const ObjectType &node,
                     const std::optional<std::set<std::string>> &fieldSet) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("implements");
    writer.StartObject();
    for (const auto &interface : node.implements | std::views::values) {
        writer.String(interface->name.c_str());
        writer.StartObject();
        writer.String("name");
        writer.String(interface->name.c_str());
        writer.String("$ref");
        writer.String(
            std::format("#/server/interfaces/{}", interface->name).c_str());
        writer.EndObject();
    };
    writer.EndObject();
    writer.String("fields");
    writer.StartObject();
    for (const auto &field : node.fields | std::views::values) {
        if (fieldSet.has_value() && !fieldSet->contains(field->name)) continue;
        writer.String(field->name.c_str());
        writeFieldDefinition(writer, *field.get());
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerType(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                     const Interface &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("fields");
    writer.StartObject();
    for (const auto &field : node.fields | std::views::values) {
        writer.String(field->name.c_str());
        writeFieldDefinition(writer, *field.get());
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerType(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                     const InputType &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("fields");
    writer.StartObject();
    for (const auto &field : node.fields | std::views::values) {
        writer.String(field.name.c_str());
        writer.StartObject();
        writer.String("nullable");
        writer.Bool(field.nullable);
        writer.String("spec");
        writeFieldSpec(writer, field.spec);
        writer.EndObject();
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerType(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                     const Enum &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("values");
    writer.StartArray();
    for (const auto &value : node.values) {
        writer.String(value.c_str());
    };
    writer.EndArray();
    writer.EndObject();
};

void writeServerType(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                     const Union &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("items");
    writer.StartObject();
    for (const auto &value : node.items | std::views::values) {
        writer.String(value->name.c_str());
        writer.String(std::format("#/server/objects/{}", value->name).c_str());
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerDirective(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                          const ServerDirective &node) {
    writer.StartObject();
    writer.String("name");
    writer.String(node.name.c_str());
    writer.String("locations");
    writer.StartArray();
    for (const auto &value : node.locations) {
        writer.String(magic_enum::enum_name(value).data());
    };
    writer.EndArray();
    writer.String("arguments");
    writer.StartObject();
    for (const auto &[name, arg] : node.arguments) {
        writer.String(name.c_str());
        writer.StartObject();
        writer.String("nullable");
        writer.Bool(arg->nullable);
        writer.String("spec");
        writeFieldSpec(writer, arg->spec);
        writer.EndObject();
    };
    writer.EndObject();
    writer.EndObject();
};

void writeServerSchema(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                       const ServerSchema &schema,
                       const std::optional<ClientSchema> &clientSchema) {
    const auto &serverUsesMap = clientSchema.transform(
        [](const auto &schema) { return uses::buildServerUsesMap(schema); });
    writer.StartObject();

    writer.String("objects");
    writer.StartObject();
    for (const auto &obj : schema.objects | std::views::values) {
        std::optional<std::set<std::string>> fieldsMap;
        if (serverUsesMap.has_value()) {
            if (obj->name == "Query") {
                if (serverUsesMap->queries.size() == 0) continue;
                fieldsMap = serverUsesMap->queries;
            } else if (obj->name == "Mutation") {
                if (serverUsesMap->mutations.size() == 0) continue;
                fieldsMap = serverUsesMap->mutations;
            } else if (obj->name == "Subscription") {
                if (serverUsesMap->subscriptions.size() == 0) continue;
                fieldsMap = serverUsesMap->subscriptions;
            } else if (!serverUsesMap->objects.contains(obj->name)) {
                continue;
            };
        };
        writer.String(obj->name.c_str());
        writeServerType(writer, *obj.get(), fieldsMap);
    };
    writer.EndObject();

    writer.String("interfaces");
    writer.StartObject();
    for (const auto &interface : schema.interfaces | std::views::values) {
        if (serverUsesMap.has_value() &&
            !serverUsesMap->interfaces.contains(interface->name)) {
            continue;
        };
        writer.String(interface->name.c_str());
        writeServerType(writer, *interface.get());
    };
    writer.EndObject();

    writer.String("inputs");
    writer.StartObject();
    for (const auto &input : schema.inputs | std::views::values) {
        if (serverUsesMap.has_value() &&
            !serverUsesMap->inputs.contains(input->name)) {
            continue;
        };
        writer.String(input->name.c_str());
        writeServerType(writer, *input.get());
    };
    writer.EndObject();

    writer.String("scalars");
    writer.StartArray();
    for (const auto &name : schema.scalars | std::views::keys) {
        if (serverUsesMap.has_value() &&
            !serverUsesMap->scalars.contains(name)) {
            continue;
        };
        writer.String(name.c_str());
    };
    writer.EndArray();

    writer.String("enums");
    writer.StartObject();
    for (const auto &node : schema.enums | std::views::values) {
        if (serverUsesMap.has_value() &&
            !serverUsesMap->enums.contains(node->name)) {
            continue;
        };
        writer.String(node->name.c_str());
        writeServerType(writer, *node.get());
    };
    writer.EndObject();

    writer.String("unions");
    writer.StartObject();
    for (const auto &node : schema.unions | std::views::values) {
        if (serverUsesMap.has_value() &&
            !serverUsesMap->unions.contains(node->name)) {
            continue;
        };
        writer.String(node->name.c_str());
        writeServerType(writer, *node.get());
    };
    writer.EndObject();

    writer.String("directives");
    writer.StartObject();
    for (const auto &node : schema.directives | std::views::values) {
        if (serverUsesMap.has_value() &&
            !serverUsesMap->directives.contains(node->name)) {
            continue;
        };
        writer.String(node->name.c_str());
        writeServerDirective(writer, *node.get());
    };
    writer.EndObject();

    writer.EndObject();
};

void writeFieldSelectionArgument(
    rapidjson::Writer<rapidjson::StringBuffer> &writer,
    const FieldSelectionArgument &argument) {
    writer.StartObject();
    writer.String("name");
    writer.String(argument.name.c_str());
    writer.String("value");
    writeArgumentValue(writer, argument.value);
    writer.EndObject();
}

void writeClientFragmentSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                             const FragmentSpec &spec);

void writeTypenameField(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                        const TypenameField &field) {
    writer.String("TypenameField");
    writer.String("alias");
    if (!field.alias.has_value()) {
        writer.Null();
        return;
    }
    writer.String(field.alias.value().c_str());
}

void writeObjectSelection(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                          const ObjectSelection &selection) {
    writer.StartObject();
    writer.String("_type");
    std::visit(utils::overloaded{
                   [&writer](const TypenameField &node) -> void {
                       writeTypenameField(writer, node);
                   },
                   [&writer](const SpreadSelection &node) -> void {
                       writer.String("SpreadSelection");
                       writer.String("fragment");
                       writer.String(node.fragment->name.c_str());
                   },
                   [&writer](const FieldSelection &node) -> void {
                       writer.String("FieldSelection");
                       writer.String("name");
                       writer.String(node.name.c_str());
                       writer.String("alias");
                       writer.String(node.alias.c_str());
                       writer.String("arguments");
                       writer.StartObject();
                       for (const auto &[name, argument] : node.arguments) {
                           writer.String(name.c_str());
                           writeFieldSelectionArgument(writer, argument);
                       }
                       writer.EndObject();
                       writer.String("selection");
                       if (!node.selection.has_value()) {
                           writer.Null();
                           return;
                       };
                       writeClientFragmentSpec(writer,
                                               *node.selection.value().get());
                   },
               },
               selection);
    writer.EndObject();
};

template <typename T>
void writeObjectFragmentSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                             const ObjectFragmentSpec<T> &selection) {
    writer.StartObject();
    writer.String("_type");
    writer.String("ObjectFragmentSpec");
    writer.String("name");
    writer.String(selection.type->name.c_str());
    writer.String("selections");
    writer.StartArray();
    for (const auto &selection : selection.selections) {
        writeObjectSelection(writer, selection);
    }
    writer.EndArray();
    writer.EndObject();
};

void writeUnionSelection(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                         const UnionSelection &selection) {
    writer.StartObject();
    writer.String("_type");
    std::visit(utils::overloaded{
                   [&writer](const TypenameField &node) {
                       writeTypenameField(writer, node);
                   },
                   [&writer](const SpreadSelection &node) {
                       writer.String("SpreadSelection");
                       writer.String("fragment");
                       writer.String(node.fragment->name.c_str());
                   },
                   [&writer](const UnionConditionalSpreadSelection &node) {
                       writer.String("UnionConditionalSpreadSelection");
                       writer.String("union");
                       writer.String(node.type->name.c_str());
                       writer.String("selections");
                       writer.StartArray();
                       for (const auto &s : node.selection->selections) {
                           writeUnionSelection(writer, s);
                       }
                       writer.EndArray();
                   },
                   [&writer](const ObjectConditionalSpreadSelection &node) {
                       writer.String("ObjectConditionalSpreadSelection");
                       writer.String("object");
                       writer.String(node.type->name.c_str());
                       writer.String("spec");
                       writeObjectFragmentSpec(writer, *node.selection.get());
                   },
               },
               selection);
    writer.EndObject();
};

void writeUnionFragmentSpec(JSONWriter &writer, const UnionFragmentSpec &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("UnionFragmentSpec");
    writer.String("name");
    writer.String(node.type->name.c_str());
    writer.String("selections");
    writer.StartArray();
    for (const auto &selection : node.selections) {
        writeUnionSelection(writer, selection);
    }
    writer.EndArray();
    writer.EndObject();
};

void writeClientFragmentSpec(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                             const FragmentSpec &spec) {
    std::visit(
        utils::overloaded{
            [&writer](const UnionFragmentSpec &node) -> void {
                writeUnionFragmentSpec(writer, node);
            },
            [&writer](const ObjectFragmentSpec<ObjectType> &node) -> void {
                writeObjectFragmentSpec(writer, node);
            },
            [&writer](const ObjectFragmentSpec<Interface> &node) -> void {
                writeObjectFragmentSpec(writer, node);
            },
        },
        spec);
}

void writeClientOperation(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                          const std::shared_ptr<Operation> &operation) {
    writer.StartObject();
    writer.String("name");
    writer.String(operation->name.c_str());
    writer.String("type");
    writer.String(magic_enum::enum_name(operation->type).data());
    writer.String("parameters");
    writer.StartObject();
    for (const auto &[name, parameter] : operation->parameters) {
        writer.String(name.c_str());
        writeFieldDefinition(writer, parameter);
    };
    writer.EndObject();
    writer.String("fragmentSpec");
    writeClientFragmentSpec(writer, operation->fragmentSpec);
    writer.String("sourceText");
    writer.String(operation->sourceText.c_str());
    writer.String("parametersHash");
    writer.String(std::to_string(operation->parametersHash).c_str());
    writer.String("fragmentSpecHash");
    writer.String(std::to_string(operation->fragmentSpecHash).c_str());
    writer.EndObject();
};

void writeClientDirective(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                          const std::shared_ptr<ClientDirective> &directive) {
    writer.StartObject();
    writer.String("name");
    writer.String(directive->name.c_str());
    writer.String("arguments");
    writer.StartObject();
    for (const auto &[name, argument] : directive->arguments) {
        writer.String(name.c_str());
        writeFieldDefinition(writer, *argument.get());
    };
    writer.EndObject();
    writer.String("locations");
    writer.StartArray();
    for (const auto &location : directive->locations) {
        writer.String(magic_enum::enum_name(location).data());
    }
    writer.EndArray();
    writer.EndObject();
};

void writeFragment(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                   const parsers::schema::ast::Fragment &fragment) {
    writer.StartObject();
    writer.String("sourceText");
    writer.String(fragment.sourceText.c_str());
    writer.String("spec");
    writeClientFragmentSpec(writer, fragment.spec);
    writer.EndObject();
};

void writeClientSchema(rapidjson::Writer<rapidjson::StringBuffer> &writer,
                       const parsers::schema::ClientSchema &schema) {
    writer.StartObject();
    writer.String("fragments");
    writer.StartObject();
    for (const auto &fragment : schema.fragments | std::views::values) {
        writer.String(fragment->name.c_str());
        writeFragment(writer, *fragment.get());
    }
    writer.EndObject();
    writer.String("operations");
    writer.StartObject();
    for (const auto &op : schema.operations | std::views::values) {
        writer.String(op->name.c_str());
        writeClientOperation(writer, op);
    };
    writer.EndObject();
    writer.String("directives");
    writer.StartObject();
    for (const auto &directive : schema.directives | std::views::values) {
        writer.String(directive->name.c_str());
        writeClientDirective(writer, directive);
    };
    writer.EndObject();
    writer.EndObject();
};

};  // namespace gql::json::serializers::schema
