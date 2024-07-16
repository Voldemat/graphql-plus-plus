#ifndef GRAPHQL_PARSERS_SERVER_AST
#define GRAPHQL_PARSERS_SERVER_AST

#include <memory>
#include <variant>
#include <vector>

#include "../shared/ast.hpp"

namespace parsers::file::server::ast {

struct FieldDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    shared::ast::TypeNode type;
    std::vector<shared::ast::InputValueDefinitionNode> arguments;
};

struct InterfaceDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<FieldDefinitionNode> fields;
};

struct ObjectDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<shared::ast::NameNode> interfaces;
    std::vector<FieldDefinitionNode> fields;
};

struct InputObjectDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<FieldDefinitionNode> fields;
};

struct EnumValueDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode value;
};

struct EnumDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<EnumValueDefinitionNode> values;
};

struct UnionDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
    std::vector<shared::ast::NameNode> values;
};

struct ScalarDefinitionNode {
    shared::ast::NodeLocation location;
    shared::ast::NameNode name;
};

using TypeDefinitionNode =
    std::variant<ScalarDefinitionNode, UnionDefinitionNode, EnumDefinitionNode,
                 InputObjectDefinitionNode, ObjectDefinitionNode,
                 InterfaceDefinitionNode>;

struct ExtendTypeNode {
    shared::ast::NodeLocation location;
    ObjectDefinitionNode typeNode;
};

using ASTNode = std::variant<TypeDefinitionNode, ExtendTypeNode>;

struct FileNodes {
    std::shared_ptr<shared::ast::SourceFile> source;
    std::vector<TypeDefinitionNode> definitions;
    std::vector<ExtendTypeNode> extensions;
};

};  // namespace parsers::file::server::ast
#endif
