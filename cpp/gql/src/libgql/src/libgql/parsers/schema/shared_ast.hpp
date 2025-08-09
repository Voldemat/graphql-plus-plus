#pragma once

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>

#include "libgql/parsers/file/server/ast.hpp"

namespace gql::parsers::schema::ast {

struct Scalar {
    std::string name;
};

struct Enum {
    std::string name;
    std::vector<std::string> values;
};

struct InputType;

using InputTypeSpec =
    std::variant<std::shared_ptr<InputType>, std::shared_ptr<Scalar>,
                 std::shared_ptr<Enum>>;

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

struct ServerDirectiveInvocation;
struct ClientDirectiveMixin {
    inline bool operator==(const ClientDirectiveMixin &) const = default;
};
struct ServerDirectiveMixin {
    std::vector<ServerDirectiveInvocation> invocations;
    inline bool operator==(const ServerDirectiveMixin &) const = default;
};

template <typename T>
struct LiteralFieldSpec
    : public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                DefaultValueMixin, EmptyMixin>,
      public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                ClientDirectiveMixin, ServerDirectiveMixin> {
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
                                ArrayDefaultValueMixin, EmptyMixin>,
      public std::conditional_t<std::is_same_v<InputTypeSpec, T>,
                                ClientDirectiveMixin, ServerDirectiveMixin> {
    T type;
    bool nullable = true;

    inline bool operator==(const ArrayFieldSpec<T> &) const = default;
};

template <typename T>
using NonCallableFieldSpec =
    std::variant<LiteralFieldSpec<T>, ArrayFieldSpec<T>>;
using InputFieldSpec = NonCallableFieldSpec<InputTypeSpec>;

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
struct ArgumentRefValue {
    std::string name;
};
struct ArgumentEnumValue {
    std::string value;
};
using ArgumentLiteralValue =
    std::variant<std::string, int, float, bool, ArgumentEnumValue>;
using ArgumentValue = std::variant<ArgumentRefValue, ArgumentLiteralValue>;

struct FieldSelectionArgument {
    std::string name;
    ArgumentValue value;
    std::shared_ptr<FieldDefinition<InputFieldSpec>> type;
};

struct ServerDirective {
    std::string name;
    std::map<std::string, std::shared_ptr<FieldDefinition<InputFieldSpec>>>
        arguments;
    std::vector<file::server::ast::DirectiveLocation> locations;
};

struct ServerDirectiveInvocation {
    std::shared_ptr<ServerDirective> directive;
    std::map<std::string, FieldSelectionArgument> arguments;
};
};  // namespace parsers::schema::ast
