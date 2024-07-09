#ifndef LIBGQL_PARSERS_CLIENT_AST
#define LIBGQL_PARSERS_CLIENT_AST

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "../shared/ast.hpp"

namespace parsers::file::client::ast {

struct Argument {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::NameNode paramName;
};

struct FragmentSpec;

struct ObjectLiteralFieldSpec {
    shared::ast::NodeLocation location;
    shared::ast::NameNode selectionName;
    shared::ast::NameNode name;
};

struct ObjectCallableFieldSpec {
    shared::ast::NodeLocation location;
    shared::ast::NameNode selectionName;
    shared::ast::NameNode name;
    std::vector<Argument> arguments;
};

using ObjectFieldSpec =
    std::variant<ObjectLiteralFieldSpec, ObjectCallableFieldSpec>;

struct FieldSelectionNode {
    shared::ast::NodeLocation location;
    ObjectFieldSpec field;
    std::optional<std::shared_ptr<FragmentSpec>> spec;
};

struct SpreadSelectionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode fragmentName;
};

struct ConditionalSpreadSelectionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode typeName;
    std::shared_ptr<FragmentSpec> fragment;
};

using SelectionNode =
    std::variant<FieldSelectionNode, ConditionalSpreadSelectionNode,
                 SpreadSelectionNode>;

struct FragmentSpec {
    shared::ast::NodeLocation location;
    std::vector<SelectionNode> selections;
};

enum class OpType { MUTATION, QUERY, SUBSCRIPTION };

std::optional<OpType> opTypeFromObjectName(const std::string &value);
std::optional<OpType> opTypeFromClientOp(const std::string &value);

struct OperationDefinition {
    shared::ast::NodeLocation location;
    OpType type;
    shared::ast::NameNode name;
    std::map<std::string, shared::ast::InputValueDefinitionNode> parameters;
    FragmentSpec fragment;
};

struct FragmentDefinition {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::NameNode typeName;
    FragmentSpec spec;
};

using ClientDefinition = std::variant<OperationDefinition, FragmentDefinition>;

};  // namespace parsers::file::client::ast
#endif
