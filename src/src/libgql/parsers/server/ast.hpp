#ifndef GRAPHQL_PARSERS_SERVER_AST
#define GRAPHQL_PARSERS_SERVER_AST

#include <filesystem>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>
#include "libgql/lexer/token.hpp"
namespace parsers {
namespace server {
namespace ast {

struct SourceFile {
    std::filesystem::path filepath;
    std::string buffer;
};

struct NodeLocation {
    GQLToken startToken;
    GQLToken endToken;
    std::shared_ptr<SourceFile> source;
};

struct NameNode {
    NodeLocation location;
    std::string name;
};

struct LiteralIntNode {
    NodeLocation location;
    int value;
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

using LiteralNode = std::variant<
    LiteralIntNode,
    LiteralFloatNode,
    LiteralStringNode,
    LiteralBooleanNode
>;

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

struct InputValueDefinitionNode {
    NodeLocation location;
    NameNode name;
    TypeNode type;
    std::optional<LiteralNode> defaultValue;
};

struct FieldDefinitionNode {
    NodeLocation location;
    NameNode name;
    TypeNode type;
    std::vector<InputValueDefinitionNode> arguments;
};

struct InterfaceDefinitionNode {
    NodeLocation location;
    NameNode name;
    std::vector<FieldDefinitionNode> fields;
};

struct ObjectDefinitionNode {
    NodeLocation location;
    NameNode name;
    std::vector<NameNode> interfaces;
    std::vector<FieldDefinitionNode> fields;
};

struct InputObjectDefinitionNode {
    NodeLocation location;
    NameNode name;
    std::vector<FieldDefinitionNode> fields;
};

struct EnumValueDefinitionNode {
    NodeLocation location;
    NameNode value;
};

struct EnumDefinitionNode {
    NodeLocation location;
    NameNode name;
    std::vector<EnumValueDefinitionNode> values;
};

struct UnionDefinitionNode {
    NodeLocation location;
    NameNode name;
    std::vector<NameNode> values;
};

struct ScalarDefinitionNode {
    NodeLocation location;
    NameNode name;
};

using TypeDefinitionNode = std::variant<
    ScalarDefinitionNode,
    UnionDefinitionNode,
    EnumDefinitionNode,
    InputObjectDefinitionNode,
    ObjectDefinitionNode,
    InterfaceDefinitionNode
>;

struct ExtendTypeNode {
    NodeLocation location;
    ObjectDefinitionNode typeNode;
};

using ASTNode = std::variant<TypeDefinitionNode, ExtendTypeNode>;

};  // namespace ast
};  // namespace server
};  // namespace parsers
#endif
