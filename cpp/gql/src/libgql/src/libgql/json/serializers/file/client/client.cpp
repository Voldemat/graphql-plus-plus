#include "./client.hpp"

#include <map>
#include <string>
#include <variant>
#include <vector>

#include "libgql/json/serializers/file/shared/shared.hpp"
#include "libgql/json/utils.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "magic_enum.hpp"
#include "utils.hpp"

using namespace gql::parsers::file::client;
namespace shared_ast = gql::parsers::file::shared::ast;

namespace gql::json::serializers::file::client {
void writeParameters(
    JSONWriter &writer,
    const std::map<std::string, shared_ast::InputFieldDefinitionNode>
        &parameters) {
    writer.StartObject();
    for (const auto &[name, param] : parameters) {
        writer.String(name.c_str());
        shared::writeInputFieldDefinitionNode(writer, param);
    };
    writer.EndObject();
};

void writeSpreadSelectionNode(JSONWriter &writer,
                              const ast::SpreadSelectionNode &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("SpreadSelectionNode");
    writer.String("fragmentName");
    shared::writeNameNode(writer, node.fragmentName);
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeFragmentSpec(JSONWriter &writer, const ast::FragmentSpec &spec);

void writeConditionalSpreadSelectionNode(
    JSONWriter &writer, const ast::ConditionalSpreadSelectionNode &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("ConditionalSpreadSelectionNode");
    writer.String("typeName");
    shared::writeNameNode(writer, node.typeName);
    writer.String("fragment");
    writeFragmentSpec(writer, *node.fragment.get());
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeObjectLiteralFieldSpec(JSONWriter &writer,
                                 const ast::ObjectLiteralFieldSpec &spec) {
    writer.StartObject();
    writer.String("_type");
    writer.String("ObjectLiteralFieldSpec");
    writer.String("name");
    shared::writeNameNode(writer, spec.name);
    writer.String("selectionName");
    shared::writeNameNode(writer, spec.selectionName);
    writer.String("location");
    shared::writeNodeLocation(writer, spec.location);
    writer.EndObject();
};

void writeObjectCallableFieldSpec(JSONWriter &writer,
                                  const ast::ObjectCallableFieldSpec &spec) {
    writer.StartObject();
    writer.String("_type");
    writer.String("ObjectCallableFieldSpec");
    writer.String("name");
    shared::writeNameNode(writer, spec.name);
    writer.String("selectionName");
    shared::writeNameNode(writer, spec.selectionName);
    writer.String("arguments");
    shared::writeArguments(writer, spec.arguments);
    writer.String("location");
    shared::writeNodeLocation(writer, spec.location);
    writer.EndObject();
};

void writeObjectFieldSpec(JSONWriter &writer,
                          const ast::ObjectFieldSpec &fieldSpec) {
    std::visit(utils::overloaded{
                   [&writer](const ast::ObjectLiteralFieldSpec &spec) {
                       writeObjectLiteralFieldSpec(writer, spec);
                   },
                   [&writer](const ast::ObjectCallableFieldSpec &spec) {
                       writeObjectCallableFieldSpec(writer, spec);
                   },
               },
               fieldSpec);
};

void writeFieldSelectionNode(JSONWriter &writer,
                             const ast::FieldSelectionNode &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("FieldSelectionNode");
    writer.String("field");
    writeObjectFieldSpec(writer, node.field);
    writer.String("spec");
    if (!node.spec.has_value()) {
        writer.Null();
    } else {
        const auto &spec = node.spec.value();
        writeFragmentSpec(writer, *spec.get());
    };
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeFragmentSelection(JSONWriter &writer,
                            const ast::SelectionNode &selectionNode) {
    std::visit(utils::overloaded{
                   [&writer](const ast::SpreadSelectionNode &node) {
                       writeSpreadSelectionNode(writer, node);
                   },
                   [&writer](const ast::ConditionalSpreadSelectionNode &node) {
                       writeConditionalSpreadSelectionNode(writer, node);
                   },
                   [&writer](const ast::FieldSelectionNode &node) {
                       writeFieldSelectionNode(writer, node);
                   },
               },
               selectionNode);
};

void writeFragmentSpec(JSONWriter &writer, const ast::FragmentSpec &spec) {
    writer.StartObject();
    writer.String("location");
    shared::writeNodeLocation(writer, spec.location);
    writer.String("selections");
    writer.StartArray();
    for (const auto &selection : spec.selections) {
        writeFragmentSelection(writer, selection);
    };
    writer.EndArray();
    writer.EndObject();
};

void writeOperationDefinition(JSONWriter &writer,
                              const ast::OperationDefinition &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("OperationDefinition");
    writer.String("type");
    writer.String(magic_enum::enum_name(node.type).data());
    writer.String("name");
    shared::writeNameNode(writer, node.name);
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.String("fragment");
    writeFragmentSpec(writer, node.fragment);
    writer.String("parameters");
    writeParameters(writer, node.parameters);
    writer.EndObject();
};

void writeFragmentDefinition(JSONWriter &writer,
                             const ast::FragmentDefinition &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("FragmentDefinition");
    writer.String("name");
    shared::writeNameNode(writer, node.name);
    writer.String("typeName");
    shared::writeNameNode(writer, node.typeName);
    writer.String("spec");
    writeFragmentSpec(writer, node.spec);
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeDirectiveLocation(JSONWriter &writer,
                            const ast::DirectiveLocationNode &node) {
    writer.StartObject();
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.String("directiveLocation");
    writer.String(magic_enum::enum_name(node.directiveLocation).data());
    writer.EndObject();
};

void writeDirectiveLocations(
    JSONWriter &writer, const std::vector<ast::DirectiveLocationNode> &nodes) {
    writer.StartArray();
    for (const auto &node : nodes) {
        writeDirectiveLocation(writer, node);
    };
    writer.EndArray();
};

void writeDirectiveDefinition(JSONWriter &writer,
                              const ast::DirectiveDefinition &node) {
    writer.StartObject();
    writer.String("_type");
    writer.String("DirectiveDefinition");
    writer.String("name");
    shared::writeNameNode(writer, node.name);
    writer.String("arguments");
    shared::writeInputFieldDefinitionNodes(writer, node.arguments);
    writer.String("targets");
    writeDirectiveLocations(writer, node.targets);
    writer.String("location");
    shared::writeNodeLocation(writer, node.location);
    writer.EndObject();
};

void writeNodes(JSONWriter &writer, const std::vector<ast::ASTNode> &nodes) {
    writer.StartArray();
    for (const auto &astNode : nodes) {
        std::visit(gql::utils::overloaded{
                       [&writer](const ast::OperationDefinition &node) {
                           writeOperationDefinition(writer, node);
                       },
                       [&writer](const ast::FragmentDefinition &node) {
                           writeFragmentDefinition(writer, node);
                       },
                       [&writer](const ast::DirectiveDefinition &node) {
                           writeDirectiveDefinition(writer, node);
                       },
                   },
                   astNode);
    };
    writer.EndArray();
};
};  // namespace gql::json::serializers::file::client
