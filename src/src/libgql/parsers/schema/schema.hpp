#ifndef GRAPHQL_SCHEMA
#define GRAPHQL_SCHEMA

#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>

#include "libgql/parsers/server/ast.hpp"

namespace parsers {
namespace schema {

struct LazySchemaNode {
    std::string name;
};

struct Scalar {
    std::string name;
};

template <typename T>
using NodeOrLazy = std::variant<T, LazySchemaNode>;

struct ObjectType;
struct InputType;
struct Union;

struct Enum {
    std::string name;
    std::vector<std::string> values;
};

using ObjectTypeSpec =
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Scalar>,
                 std::shared_ptr<Enum>, std::shared_ptr<Union>>;

using InputTypeSpec =
    std::variant<std::shared_ptr<InputType>, std::shared_ptr<Scalar>,
                 std::shared_ptr<Enum>>;

struct Union {
    std::string name;
    std::vector<NodeOrLazy<std::shared_ptr<ObjectType>>> items;
};

using Literal = std::variant<int, float, std::string, bool>;

struct EmptyMixin {};
struct DefaultValueMixin {
    std::optional<Literal> defaultValue;
};

template <typename T>
struct LiteralFieldSpec
    : public std::conditional_t<std::is_same_v<InputTypeSpec, T>, DefaultValueMixin,
                                EmptyMixin> {
    NodeOrLazy<T> type;
};

struct ArrayDefaultValueMixin {
    std::optional<std::vector<Literal>> defaultValue;
};

template <typename T>
struct ArrayFieldSpec
    : public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                ArrayDefaultValueMixin, EmptyMixin> {
    NodeOrLazy<T> type;
    bool nullable = true;
};

template <typename T>
using NonCallableFieldSpec =
    std::variant<LiteralFieldSpec<T>, ArrayFieldSpec<T>>;

using InputFieldSpec = NonCallableFieldSpec<InputTypeSpec>;

template <typename T>
struct FieldDefinition;

struct CallableFieldSpec {
    NodeOrLazy<NonCallableFieldSpec<ObjectTypeSpec>> returnType;
    std::vector<FieldDefinition<InputFieldSpec>> arguments;
};

using ObjectFieldSpec =
    std::variant<LiteralFieldSpec<ObjectTypeSpec>, ArrayFieldSpec<ObjectTypeSpec>,
                 CallableFieldSpec>;

template <typename T>
struct FieldDefinition {
    std::string name;
    T spec;
    bool nullable = true;
};

struct InputType {
    std::string name;
    std::vector<FieldDefinition<InputFieldSpec>> fields;
};

struct Interface {
    std::string name;
    std::vector<FieldDefinition<ObjectFieldSpec>> fields;
};

struct ObjectType {
    std::string name;
    std::vector<FieldDefinition<ObjectFieldSpec>> fields;
    std::vector<NodeOrLazy<std::shared_ptr<Interface>>> implements;
};

using SchemaNode =
    std::variant<
        std::shared_ptr<ObjectType>,
        std::shared_ptr<Interface>,
        std::shared_ptr<Scalar>,
        std::shared_ptr<Union>,
        std::shared_ptr<Enum>,
        std::shared_ptr<InputType>
    >;

std::vector<SchemaNode> parseSchema(
    std::vector<parsers::server::ast::FileNodes> astArray);

};  // namespace schema
};  // namespace parsers

#endif
