#ifndef GRAPHQL_SCHEMA
#define GRAPHQL_SCHEMA

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>

#include "libgql/parsers/client/ast.hpp"
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
    : public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                DefaultValueMixin, EmptyMixin> {
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
    std::variant<LiteralFieldSpec<ObjectTypeSpec>,
                 ArrayFieldSpec<ObjectTypeSpec>, CallableFieldSpec>;

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
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Interface>,
                 std::shared_ptr<Scalar>, std::shared_ptr<Union>,
                 std::shared_ptr<Enum>, std::shared_ptr<InputType>>;

struct Fragment;
struct FieldSelection;
struct ConditionalSpreadSelection;
struct SpreadSelection {
    NodeOrLazy<std::shared_ptr<Fragment>> fragment;
};
using Selection =
    std::variant<FieldSelection, SpreadSelection, ConditionalSpreadSelection>;

struct FragmentSpec {
    std::vector<Selection> selections;
};

struct ConditionalSpreadSelection {
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Union>> type;
    std::shared_ptr<FragmentSpec> selection;
};

struct FieldSelection {
    std::string name;
    std::string alias;
    std::optional<std::shared_ptr<FragmentSpec>> selection;
};

struct Fragment {
    std::string name;
    std::string typeName;
    std::shared_ptr<FragmentSpec> spec;
};

struct Operation {
    client::ast::OpType type;
    std::string name;
    std::vector<FieldDefinition<InputFieldSpec>> arguments;
    std::string opName;
    std::string returnFieldName;
    std::map<std::string, std::string> argumentsMapping;
    std::shared_ptr<FragmentSpec> fragmentSpec;
};

using ClientSchemaNode =
    std::variant<std::shared_ptr<Fragment>, std::shared_ptr<Operation>>;

struct Schema {
    std::vector<SchemaNode> serverNodes;
    std::vector<ClientSchemaNode> clientNodes;
};

Schema parseSchema(
    std::vector<parsers::server::ast::FileNodes> astArray,
    std::vector<parsers::client::ast::ClientDefinition> definitions);

};  // namespace schema
};  // namespace parsers

#endif
