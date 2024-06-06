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

struct Scalar {
    std::string name;
};

struct Interface;
struct ObjectType;
struct InputType;
struct Union;

struct Enum {
    std::string name;
    std::vector<std::string> values;
};

using ObjectTypeSpec =
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Interface>,
                 std::shared_ptr<Scalar>, std::shared_ptr<Enum>,
                 std::shared_ptr<Union>>;

using InputTypeSpec =
    std::variant<std::shared_ptr<InputType>, std::shared_ptr<Scalar>,
                 std::shared_ptr<Enum>>;

struct Union {
    std::string name;
    std::map<std::string, std::shared_ptr<ObjectType>> items;
};

using Literal = std::variant<int, float, std::string, bool>;

struct EmptyMixin {};
struct DefaultValueMixin {
    std::optional<Literal> defaultValue;

    inline bool operator==(const DefaultValueMixin &) const = default;
};

template <typename T>
struct LiteralFieldSpec
    : public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                DefaultValueMixin, EmptyMixin> {
    T type;
    inline bool operator==(const LiteralFieldSpec<T> &) const = default;
};

struct ArrayDefaultValueMixin {
    std::optional<std::vector<Literal>> defaultValue;

    inline bool operator==(const ArrayDefaultValueMixin &) const = default;
};

template <typename T>
struct ArrayFieldSpec
    : public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                ArrayDefaultValueMixin, EmptyMixin> {
    T type;
    bool nullable = true;

    inline bool operator==(const ArrayFieldSpec<T> &) const = default;
};

template <typename T>
using NonCallableFieldSpec =
    std::variant<LiteralFieldSpec<T>, ArrayFieldSpec<T>>;

using InputFieldSpec = NonCallableFieldSpec<InputTypeSpec>;

template <typename T>
struct FieldDefinition;

struct CallableFieldSpec {
    NonCallableFieldSpec<ObjectTypeSpec> returnType;
    std::map<std::string, std::shared_ptr<FieldDefinition<InputFieldSpec>>> arguments;

    inline bool operator==(const CallableFieldSpec &) const = default;
};

using ObjectFieldSpec =
    std::variant<LiteralFieldSpec<ObjectTypeSpec>,
                 ArrayFieldSpec<ObjectTypeSpec>, CallableFieldSpec>;

template <typename T>
struct FieldDefinition {
    std::string name;
    T spec;
    bool nullable = true;

    inline bool operator==(const FieldDefinition &) const = default;
};

struct InputType {
    std::string name;
    std::map<std::string, FieldDefinition<InputFieldSpec>> fields;
};

struct Interface {
    std::string name;
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        fields;
};

struct ObjectType {
    std::string name;
    std::map<std::string, std::shared_ptr<FieldDefinition<ObjectFieldSpec>>>
        fields;
    std::map<std::string, std::shared_ptr<Interface>> implements;
};

struct ExtendObjectType {
    ObjectType type;
};

using SchemaNode =
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Interface>,
                 std::shared_ptr<Scalar>, std::shared_ptr<Union>,
                 std::shared_ptr<Enum>, std::shared_ptr<InputType>>;

struct Fragment;
struct FieldSelection;
struct ConditionalSpreadSelection;
struct SpreadSelection {
    std::shared_ptr<Fragment> fragment;
};

struct TypenameField {};
using UnionSelection =
    std::variant<TypenameField, SpreadSelection, ConditionalSpreadSelection>;
using ObjectSelection =
    std::variant<TypenameField, SpreadSelection, FieldSelection>;

struct UnionFragmentSpec {
    std::shared_ptr<Union> type;
    std::vector<UnionSelection> selections;
};

template <typename T>
struct ObjectFragmentSpec {
    std::shared_ptr<T> type;
    std::vector<ObjectSelection> selections;
};

struct ConditionalSpreadSelection {
    std::shared_ptr<ObjectType> type;
    std::shared_ptr<ObjectFragmentSpec<ObjectType>> selection;
};

using FragmentSpec =
    std::variant<UnionFragmentSpec, ObjectFragmentSpec<ObjectType>,
                 ObjectFragmentSpec<Interface>>;

struct FieldSelectionArgument {
    std::string name;
    std::string parameterName;
    std::shared_ptr<FieldDefinition<InputFieldSpec>> type;
};

struct FieldSelection {
    std::string name;
    std::string alias;
    std::optional<std::map<std::string, FieldSelectionArgument>> arguments;
    std::optional<std::shared_ptr<FragmentSpec>> selection;
};

struct Fragment {
    std::string name;
    FragmentSpec spec;
};

struct Operation {
    client::ast::OpType type;
    std::string name;
    std::map<std::string, FieldDefinition<InputFieldSpec>> parameters;
    FragmentSpec fragmentSpec;
};

using ClientSchemaNode =
    std::variant<std::shared_ptr<Fragment>, std::shared_ptr<Operation>>;

struct Schema {
    std::vector<SchemaNode> serverNodes;
    std::vector<ClientSchemaNode> clientNodes;
};

const Schema parseSchema(
    std::vector<parsers::server::ast::FileNodes> astArray,
    std::vector<parsers::client::ast::ClientDefinition> definitions);

};  // namespace schema
};  // namespace parsers

#endif
