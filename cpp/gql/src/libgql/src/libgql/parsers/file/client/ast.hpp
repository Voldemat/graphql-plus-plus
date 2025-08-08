#pragma once

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "../shared/ast.hpp"

namespace parsers::file::client::ast {

enum class DirectiveLocation {
    QUERY,
    MUTATION,
    SUBSCRIPTION,
    FIELD,
    FRAGMENT_DEFINITION,
    FRAGMENT_SPREAD,
    INLINE_FRAGMENT,
    VARIABLE_DEFINITION,
};

std::optional<DirectiveLocation> stringToDirectiveLocation(
    const std::string &str);

using DirectiveLocationNode =
    shared::ast::DirectiveLocationNode<DirectiveLocation>;
using DirectiveDefinition = shared::ast::DirectiveNode<DirectiveLocation>;

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
    std::vector<shared::ast::Argument> arguments;
};

using ObjectFieldSpec =
    std::variant<ObjectLiteralFieldSpec, ObjectCallableFieldSpec>;

std::optional<std::string> extractSelectionName(const ObjectFieldSpec& spec);

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

using ASTNode =
    std::variant<OperationDefinition, FragmentDefinition, DirectiveDefinition>;

};  // namespace parsers::file::client::ast
