#include "./uses.hpp"

#include <memory>
#include <ranges>
#include <set>
#include <string>
#include <variant>

#include "../client_ast.hpp"
#include "../schema.hpp"
#include "../server_ast.hpp"
#include "../shared_ast.hpp"
#include "libgql/parsers/schema/visitor.hpp"

namespace gql::parsers::schema::uses {
ServerUsesMap buildServerUsesMap(const ClientSchema &schema) {
    ServerUsesMap map;
    visitor::ASTVisitorHooks hooks = {
        .visitObjectType =
            [&map](const std::shared_ptr<ast::ObjectType> &objectType) {
                map.objects.insert(objectType->name);
            },
        .visitObjectFragmentSpec_ObjectType = 
            [&map](const ast::ObjectFragmentSpec<ast::ObjectType>& fragmentSpec) {
                if (fragmentSpec.type->name == "Query") {
                    for (const auto& field : fragmentSpec.selections | 
                        std::views::filter([](const ast::ObjectSelection& selection){
                            return std::holds_alternative<ast::FieldSelection>(selection);
                        }) |
                        std::views::transform([](const ast::ObjectSelection& selection){
                            return std::get<ast::FieldSelection>(selection);
                        })) {
                        map.queries.insert(field.name);
                    };
                } else if (fragmentSpec.type->name == "Mutation") {
                    for (const auto& field : fragmentSpec.selections | 
                        std::views::filter([](const ast::ObjectSelection& selection){
                            return std::holds_alternative<ast::FieldSelection>(selection);
                        }) |
                        std::views::transform([](const ast::ObjectSelection& selection){
                            return std::get<ast::FieldSelection>(selection);
                        })) {
                        map.mutations.insert(field.name);
                    };
                } else if (fragmentSpec.type->name == "Subscription") {
                    for (const auto& field : fragmentSpec.selections | 
                        std::views::filter([](const ast::ObjectSelection& selection){
                            return std::holds_alternative<ast::FieldSelection>(selection);
                        }) |
                        std::views::transform([](const ast::ObjectSelection& selection){
                            return std::get<ast::FieldSelection>(selection);
                        })) {
                        map.subscriptions.insert(field.name);
                    };
                };
            },
        .visitInterface =
            [&map](const std::shared_ptr<ast::Interface> &interface) {
                map.interfaces.insert(interface->name);
            },
        .visitInputType = [&map](const std::shared_ptr<ast::InputType>& input) {
            map.inputs.insert(input->name);
        },
        .visitScalar =
            [&map](const std::shared_ptr<ast::Scalar> &scalar) {
                map.scalars.insert(scalar->name);
            },
        .visitEnum =
            [&map](const std::shared_ptr<ast::Enum> &enumType) {
                map.enums.insert(enumType->name);
            },
        .visitUnion =
            [&map](const std::shared_ptr<ast::Union> &unionType) {
                map.unions.insert(unionType->name);
            },
    };
    visitor::visitClientSchema(hooks, schema);
    return map;
};
};  // namespace gql::parsers::schema::uses
