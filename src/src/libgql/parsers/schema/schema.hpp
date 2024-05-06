#ifndef GRAPHQL_SCHEMA
#define GRAPHQL_SCHEMA

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "libgql/parsers/server/ast.hpp"
namespace parsers {
namespace schema {

using Literal = std::variant<int, float, std::string, bool>;
using ListLiteral = std::variant<std::vector<int>, std::vector<float>,
                                 std::vector<std::string>, std::vector<bool>>;

struct Scalar {
    std::string name;
};

struct EnumType {
    std::string name;
    std::vector<std::string> values;
};

struct ObjectType;
struct InputType;

struct UnionType {
    std::string name;
    std::vector<std::shared_ptr<ObjectType>> items;
};

using ObjectTypeKind =
    std::variant<std::shared_ptr<Scalar>, std::shared_ptr<EnumType>,
                 std::shared_ptr<UnionType>, std::shared_ptr<ObjectType>>;

using InputTypeKind =
    std::variant<std::shared_ptr<Scalar>, std::shared_ptr<EnumType>,
                 std::shared_ptr<InputType>>;

template <typename T>
struct TypeSpec {
    T type;
    bool nullable = true;
};

template <typename T>
struct BasicFieldSpec {
    TypeSpec<T> spec;
    std::optional<Literal> defaultValue;
};

template <typename T>
struct ListFieldSpec {
    TypeSpec<T> spec;
    bool nullable = true;
    std::optional<ListLiteral> defaultValue;
};

struct CallableFieldSpec {
    TypeSpec<ObjectTypeKind> returnType;
    std::map<std::string, BasicFieldSpec<InputTypeKind>> arguments;
};

template <typename T>
using LiteralFieldSpec = std::variant<BasicFieldSpec<T>, ListFieldSpec<T>>;
template <typename T>
using FieldSpec =
    std::variant<BasicFieldSpec<T>, ListFieldSpec<T>, CallableFieldSpec>;

struct InterfaceType {
    std::string name;
    std::map<std::string, FieldSpec<ObjectTypeKind>> fields;
};

struct ObjectType {
    std::string name;
    std::map<std::string, FieldSpec<ObjectTypeKind>> fields;
    std::vector<std::shared_ptr<InterfaceType>> implements;
};

struct InputType {
    std::string name;
    std::map<std::string, FieldSpec<InputTypeKind>> fields;
};

struct ASTSchema {
    std::map<std::string, server::ast::ScalarDefinitionNode> scalars;
    std::map<std::string, server::ast::EnumDefinitionNode> enums;
    std::map<std::string, server::ast::UnionDefinitionNode> unions;
    std::map<std::string, server::ast::ObjectDefinitionNode> objects;
    std::map<std::string, server::ast::InterfaceDefinitionNode> interfaces;
    std::map<std::string, server::ast::InputObjectDefinitionNode> inputs;
    std::map<std::string, std::vector<server::ast::ExtendTypeNode>>
        objectsExtensions;

    ASTSchema(std::vector<server::ast::FileNodes> astList);
};

struct Schema {
    std::map<std::string, std::shared_ptr<Scalar>> scalars;
    std::map<std::string, std::shared_ptr<EnumType>> enums;
    std::map<std::string, std::shared_ptr<UnionType>> unions;
    std::map<std::string, std::shared_ptr<ObjectType>> objects;
    std::map<std::string, std::shared_ptr<InterfaceType>> interfaces;
    std::map<std::string, std::shared_ptr<InputType>> inputs;

    Schema(ASTSchema astSchema);
    FieldSpec<ObjectTypeKind> parseObjectFieldSpec(
        const server::ast::FieldDefinitionNode &field);
    Literal parseLiteral(const server::ast::LiteralNode& node);
};

};  // namespace schema
};  // namespace parsers

#endif
