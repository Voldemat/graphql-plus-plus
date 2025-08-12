#pragma once

#include <memory>
#include <set>
#include <string>

#include "../schema.hpp"
#include "../shared_ast.hpp"

namespace gql::parsers::schema::uses {
bool usesScalar(const ClientSchema &schema, const std::string &name);
bool usesInput(const ClientSchema &schema,
               const std::shared_ptr<ast::InputType> &input);
bool usesEnum(const ClientSchema &schema,
              const std::shared_ptr<ast::Enum> &enumType);
struct ServerUsesMap {
    std::set<std::string> objects;
    std::set<std::string> inputs;
    std::set<std::string> scalars;
    std::set<std::string> enums;
    std::set<std::string> unions;
    std::set<std::string> interfaces;
    std::set<std::string> directives;
    std::set<std::string> queries;
    std::set<std::string> mutations;
    std::set<std::string> subscriptions;
};

ServerUsesMap buildServerUsesMap(const ClientSchema &schema);
};  // namespace gql::parsers::schema::uses
