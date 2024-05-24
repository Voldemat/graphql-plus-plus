#ifndef LIBGQL_PARSERS_CLIENT_AST
#define LIBGQL_PARSERS_CLIENT_AST

#include <memory>
#include <optional>
#include <variant>
#include <vector>

#include "libgql/parsers/shared/shared.hpp"

namespace parsers {
namespace client {
namespace ast {

struct Argument {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::NameNode argAliasName;
};

struct FragmentSpec;

struct ObjectFieldSpec {
    shared::ast::NodeLocation location;
    shared::ast::NameNode selectionName;
    shared::ast::NameNode name;
    std::optional<std::vector<shared::ast::InputValueDefinitionNode>> arguments;
};

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

struct OperationArg {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::NameNode paramName;
};

struct OperationSpec {
    shared::ast::NodeLocation location;
    shared::ast::NameNode selectionName;
    shared::ast::NameNode name;
    std::vector<OperationArg> args;
};

struct OperationDefinition {
    shared::ast::NodeLocation location;
    OpType type;
    shared::ast::NameNode name;
    std::vector<shared::ast::InputValueDefinitionNode> parameters;
    OperationSpec spec;
    FragmentSpec fragment;
};

struct FragmentDefinition {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::NameNode typeName;
    FragmentSpec spec;
};

using ClientDefinition = std::variant<OperationDefinition, FragmentDefinition>;

};  // namespace ast
};  // namespace client
};  // namespace parsers
#endif
