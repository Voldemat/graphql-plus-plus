#pragma once

#include <map>
#include <memory>
#include <string>
#include <variant>

#include "./shared_ast.hpp"

namespace gql::parsers::schema::ast {

struct Interface;
struct ObjectType;
struct InputType;
struct Union;
using ObjectTypeSpec =
    std::variant<std::shared_ptr<ObjectType>, std::shared_ptr<Interface>,
                 std::shared_ptr<Scalar>, std::shared_ptr<Enum>,
                 std::shared_ptr<Union>>;

struct Union {
    std::string name;
    std::map<std::string, std::shared_ptr<ObjectType>> items;
};

struct CallableFieldSpec : public ServerDirectiveMixin {
    NonCallableFieldSpec<ObjectTypeSpec> returnType;
    std::map<std::string, std::shared_ptr<FieldDefinition<InputFieldSpec>>>
        arguments;
    inline bool operator==(const CallableFieldSpec &) const = default;
};

using ObjectFieldSpec =
    std::variant<LiteralFieldSpec<ObjectTypeSpec>,
                 ArrayFieldSpec<ObjectTypeSpec>, CallableFieldSpec>;

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
                 std::shared_ptr<Enum>, std::shared_ptr<InputType>,
                 std::shared_ptr<ServerDirective>>;
bool InputFieldSpec_hasDefaultValue(const InputFieldSpec &spec);
InputTypeSpec extractInputTypeSpec(const InputFieldSpec &spec);
};  // namespace parsers::schema::ast
