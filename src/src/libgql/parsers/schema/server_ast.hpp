#pragma once

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>
namespace parsers::schema::ast {
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
using ArrayLiteral = std::variant<std::vector<int>, std::vector<float>,
                                  std::vector<std::string>, std::vector<bool>>;

struct EmptyMixin {
    bool hasDefaultValue() const { return false; };

    inline bool operator==(const EmptyMixin &) const = default;
};
struct DefaultValueMixin {
    std::optional<Literal> defaultValue;

    bool hasDefaultValue() const { return defaultValue.has_value(); };

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
    std::optional<ArrayLiteral> defaultValue;

    bool hasDefaultValue() const { return defaultValue.has_value(); };

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
    std::map<std::string, std::shared_ptr<FieldDefinition<InputFieldSpec>>>
        arguments;

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

using ServerSchemaNode =
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Interface>,
                 std::shared_ptr<Scalar>, std::shared_ptr<Union>,
                 std::shared_ptr<Enum>, std::shared_ptr<InputType>>;
bool InputFieldSpec_hasDefaultValue(const InputFieldSpec &spec);
InputTypeSpec extractInputTypeSpec(const InputFieldSpec &spec);
};  // namespace parsers::schema::ast
