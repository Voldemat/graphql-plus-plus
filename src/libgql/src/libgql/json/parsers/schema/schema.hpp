#pragma once

#include <rapidjson/document.h>

#include <memory>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace json::parsers::schema {
::parsers::schema::Schema parseSchema(const rapidjson::Document &document);
std::shared_ptr<::parsers::schema::ast::Scalar> parseScalar(
    const JSONValue &value);

std::shared_ptr<::parsers::schema::ast::Enum> parseEnum(
    const JSONObjectEntry &entry);

::parsers::schema::ast::FieldDefinition<::parsers::schema::ast::InputFieldSpec>
parseInputFieldDefinition(const JSONObjectEntry &entry,
                          const ::parsers::schema::TypeRegistry &registry);

std::shared_ptr<::parsers::schema::ast::FieldDefinition<
    ::parsers::schema::ast::ObjectFieldSpec>>
parseObjectFieldDefinition(const JSONObjectEntry &entry,
                           const ::parsers::schema::TypeRegistry &registry);
};  // namespace json::parsers::schema
