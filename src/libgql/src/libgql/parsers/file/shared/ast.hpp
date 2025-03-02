#pragma once

#include <filesystem>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "libgql/lexer/token.hpp"

namespace parsers::file::shared::ast {
struct SourceFile {
    std::filesystem::path filepath;
    std::string buffer;
};

struct NodeLocation {
    lexer::GQLToken startToken;
    lexer::GQLToken endToken;
    std::shared_ptr<SourceFile> source;
};

struct NameNode {
    NodeLocation location;
    std::string name;
};

struct LiteralIntNode {
    NodeLocation location;
    int value = 0;
};

struct LiteralFloatNode {
    NodeLocation location;
    float value = 0.0;
};

struct LiteralStringNode {
    NodeLocation location;
    std::string value;
};

struct LiteralBooleanNode {
    NodeLocation location;
    bool value = false;
};

struct LiteralEnumValueNode {
    NodeLocation location;
    std::string value;
};

using LiteralNode =
    std::variant<LiteralIntNode, LiteralFloatNode, LiteralStringNode,
                 LiteralBooleanNode, LiteralEnumValueNode>;

struct NamedTypeNode {
    NodeLocation location;
    NameNode name;
    bool nullable = true;
};

struct ListTypeNode {
    NodeLocation location;
    NamedTypeNode type;
    bool nullable = true;
};

using TypeNode = std::variant<NamedTypeNode, ListTypeNode>;

using ArgumentValue = std::variant<
    shared::ast::NameNode,
    shared::ast::LiteralNode
>;

struct Argument {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    ArgumentValue value;
};

struct DirectiveInvocationNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<shared::ast::Argument> arguments;
};

struct InputValueDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::TypeNode type;
    std::optional<shared::ast::LiteralNode> defaultValue;
    std::vector<DirectiveInvocationNode> directives;
};

template <class T>
struct DirectiveLocationNode {
    shared::ast::NodeLocation location;
    T directiveLocation;
};

template <class T>
struct DirectiveNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<DirectiveLocationNode<T>> targets;
    std::vector<shared::ast::InputValueDefinitionNode> arguments;
};

};  // namespace parsers::file::shared::ast
