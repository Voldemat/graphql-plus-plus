#pragma once

#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "../shared/ast.hpp"

namespace gql::parsers::file::server::ast {

enum class DirectiveLocation {
    SCHEMA,
    SCALAR,
    OBJECT,
    FIELD_DEFINITION,
    ARGUMENT_DEFINITION,
    INTERFACE,
    UNION,
    ENUM,
    ENUM_VALUE,
    INPUT_OBJECT,
    INPUT_FIELD_DEFINITION,
};

std::optional<DirectiveLocation> stringToDirectiveLocation(
    const std::string &str);

using DirectiveLocationNode =
    shared::ast::DirectiveLocationNode<DirectiveLocation>;
using DirectiveDefinitionNode = shared::ast::DirectiveNode<DirectiveLocation>;

struct FieldDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::TypeNode type;
    std::vector<shared::ast::InputValueDefinitionNode> arguments;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct InterfaceDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<FieldDefinitionNode> fields;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct ObjectDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<shared::ast::NameNode> interfaces;
    std::vector<FieldDefinitionNode> fields;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct InputObjectDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<FieldDefinitionNode> fields;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct EnumValueDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode value;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct EnumDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<EnumValueDefinitionNode> values;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct UnionDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<shared::ast::NameNode> values;
    std::vector<shared::ast::DirectiveInvocationNode> directives;
};

struct ScalarDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
};

using TypeDefinitionNode =
    std::variant<ScalarDefinitionNode, UnionDefinitionNode, EnumDefinitionNode,
                 InputObjectDefinitionNode, ObjectDefinitionNode,
                 InterfaceDefinitionNode, DirectiveDefinitionNode>;

struct ExtendTypeNode {
    shared::ast::NodeLocation location;
    ObjectDefinitionNode typeNode;
};

using ASTNode = std::variant<TypeDefinitionNode, ExtendTypeNode>;
};  // namespace parsers::file::server::ast
