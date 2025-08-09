#pragma once

#include <rapidjson/document.h>

#include <memory>

#include "libgql/json/utils.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "libgql/parsers/schema/type_registry.hpp"

namespace gql::json::parsers::schema {
::gql::parsers::schema::Schema parseSchema(const rapidjson::Document &document);
std::shared_ptr<::gql::parsers::schema::ast::Scalar> parseScalar(
    const JSONValue &value);

std::shared_ptr<::gql::parsers::schema::ast::Enum> parseEnum(
    const JSONObjectEntry &entry);

::gql::parsers::schema::ast::FieldDefinition<
    ::gql::parsers::schema::ast::InputFieldSpec>
parseInputFieldDefinition(const JSONObjectEntry &entry,
                          const ::gql::parsers::schema::TypeRegistry &registry);

std::shared_ptr<::gql::parsers::schema::ast::FieldDefinition<
    ::gql::parsers::schema::ast::ObjectFieldSpec>>
parseObjectFieldDefinition(
    const JSONObjectEntry &entry,
    const ::gql::parsers::schema::TypeRegistry &registry);
};  // namespace gql::json::parsers::schema
