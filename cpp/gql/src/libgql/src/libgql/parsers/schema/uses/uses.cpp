#include "./uses.hpp"

#include <functional>
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
#include "utils.hpp"

namespace gql::parsers::schema::uses {

template <typename T, typename Arg>
using TypeSpecChecker = std::function<bool(const T &spec, const Arg &arg)>;
template <typename T>
using InputTypeSpecChecker = TypeSpecChecker<ast::InputTypeSpec, T>;
template <typename T>
using ObjectTypeSpecChecker = TypeSpecChecker<ast::ObjectTypeSpec, T>;

template <typename Arg>
bool fragmentSpecUses(const ast::FragmentSpec &fragment, const Arg &arg,
                      const InputTypeSpecChecker<Arg> &inputChecker,
                      const ObjectTypeSpecChecker<Arg> &objectChecker);

template <typename T, typename Arg>
bool nonCallableFieldSpecUses(const ast::NonCallableFieldSpec<T> &fieldSpec,
                              const Arg &arg,
                              const TypeSpecChecker<T, Arg> &checker) {
    return std::visit<bool>(
        utils::overloaded{
            [&arg, &checker](const ast::LiteralFieldSpec<T> &spec) {
                return checker(spec.type, arg);
            },
            [&arg, &checker](const ast::ArrayFieldSpec<T> &spec) {
                return checker(spec.type, arg);
            },
        },
        fieldSpec);
};

template <typename T>
bool objectFieldSpecUses(const ast::ObjectFieldSpec &fieldSpec, const T &arg,
                         const InputTypeSpecChecker<T> &inputFunc,
                         const ObjectTypeSpecChecker<T> &objectFunc) {
    return std::visit<bool>(
        utils::overloaded{
            [&arg, &objectFunc](
                const ast::LiteralFieldSpec<ast::ObjectTypeSpec> &spec) {
                return objectFunc(spec.type, arg);
            },
            [&arg, &objectFunc](
                const ast::ArrayFieldSpec<ast::ObjectTypeSpec> &spec) {
                return objectFunc(spec.type, arg);
            },
            [&arg, &objectFunc,
             &inputFunc](const ast::CallableFieldSpec &spec) {
                for (const auto &argument :
                     spec.arguments | std::views::values) {
                    if (nonCallableFieldSpecUses<ast::InputTypeSpec>(
                            argument->spec, arg, inputFunc))
                        return true;
                };
                return nonCallableFieldSpecUses<ast::ObjectTypeSpec>(
                    spec.returnType, arg, objectFunc);
            },
        },
        fieldSpec);
};

template <typename T, typename Arg>
bool fieldSelectionUses(const ast::FieldSelection &selection,
                        const std::shared_ptr<T> &type, const Arg &arg,
                        const InputTypeSpecChecker<Arg> &inputFunc,
                        const ObjectTypeSpecChecker<Arg> &objectFunc) {
    for (const auto &argument : selection.arguments | std::views::values) {
        if (nonCallableFieldSpecUses<ast::InputTypeSpec, Arg>(
                argument.type->spec, arg, inputFunc))
            return true;
    };

    if (objectFieldSpecUses(type->fields[selection.name]->spec, arg, inputFunc,
                            objectFunc)) {
        return true;
    };

    if (selection.selection.has_value()) {
        if (fragmentSpecUses<Arg>(*selection.selection.value().get(), arg,
                                  inputFunc, objectFunc))
            return true;
    };
    return false;
};

template <typename T, typename Arg>
bool objectSelectionUses(const ast::ObjectSelection &objectSelection,
                         const std::shared_ptr<T> &type, const Arg &arg,
                         const InputTypeSpecChecker<Arg> &inputChecker,
                         const ObjectTypeSpecChecker<Arg> &objectChecker) {
    return std::visit<bool>(
        utils::overloaded{
            [](const ast::TypenameField &selection) { return false; },
            [&arg, &inputChecker,
             &objectChecker](const ast::SpreadSelection &selection) {
                return fragmentSpecUses(selection.fragment->spec, arg,
                                        inputChecker, objectChecker);
            },
            [&type, &arg, &inputChecker,
             &objectChecker](const ast::FieldSelection &selection) {
                return fieldSelectionUses(selection, type, arg, inputChecker,
                                          objectChecker);
            },
        },
        objectSelection);
};

template <typename T, typename Arg>
bool objectFragmentSpecUses(const ast::ObjectFragmentSpec<T> &spec,
                            const Arg &arg,
                            const InputTypeSpecChecker<Arg> &inputChecker,
                            const ObjectTypeSpecChecker<Arg> &objectChecker) {
    for (const auto &selection : spec.selections) {
        if (objectSelectionUses<T, Arg>(selection, spec.type, arg, inputChecker,
                                        objectChecker))
            return true;
    };
    return false;
};

template <typename Arg>
bool unionSelectionUses(const ast::UnionSelection &unionSelection,
                        const Arg &arg,
                        const InputTypeSpecChecker<Arg> &inputChecker,
                        const ObjectTypeSpecChecker<Arg> &objectChecker) {
    return std::visit<bool>(
        utils::overloaded{
            [](const ast::TypenameField &field) { return false; },
            [&arg, &inputChecker, &objectChecker](
                const ast::ObjectConditionalSpreadSelection &field) {
                return objectFragmentSpecUses(*field.selection.get(), arg,
                                              inputChecker, objectChecker);
            },
            [](const ast::UnionConditionalSpreadSelection &selection) {
                return false;
            },
            [&arg, &inputChecker,
             &objectChecker](const ast::SpreadSelection &selection) {
                return fragmentSpecUses(selection.fragment->spec, arg,
                                        inputChecker, objectChecker);
            } },
        unionSelection);
};

template <typename Arg>
bool unionFragmentSpecUses(const ast::UnionFragmentSpec &spec, const Arg &arg,
                           const InputTypeSpecChecker<Arg> &inputChecker,
                           const ObjectTypeSpecChecker<Arg> &objectChecker) {
    for (const auto &selection : spec.selections) {
        if (unionSelectionUses<Arg>(selection, arg, inputChecker,
                                    objectChecker))
            return true;
    };
    return false;
};

template <typename Arg>
bool fragmentSpecUses(const ast::FragmentSpec &fragmentSpec, const Arg &arg,
                      const InputTypeSpecChecker<Arg> &inputChecker,
                      const ObjectTypeSpecChecker<Arg> &objectChecker) {
    return std::visit<bool>(
        utils::overloaded{
            [&arg, &inputChecker, &objectChecker](
                const ast::ObjectFragmentSpec<ast::ObjectType> &spec) {
                return objectFragmentSpecUses(spec, arg, inputChecker,
                                              objectChecker);
            },
            [&arg, &inputChecker, &objectChecker](
                const ast::ObjectFragmentSpec<ast::Interface> &spec) {
                return objectFragmentSpecUses(spec, arg, inputChecker,
                                              objectChecker);
            },
            [&arg, &inputChecker,
             &objectChecker](const ast::UnionFragmentSpec &spec) {
                return unionFragmentSpecUses(spec, arg, inputChecker,
                                             objectChecker);
            },
        },
        fragmentSpec);
};

template <typename Arg>
bool operationUses(const std::shared_ptr<ast::Operation> &operation,
                   const Arg &arg,
                   const InputTypeSpecChecker<Arg> &inputChecker,
                   const ObjectTypeSpecChecker<Arg> &objectChecker) {
    for (const auto &argument : operation->parameters | std::views::values) {
        if (nonCallableFieldSpecUses(argument.spec, arg, inputChecker))
            return true;
    };
    return fragmentSpecUses<Arg>(operation->fragmentSpec, arg, inputChecker,
                                 objectChecker);
};

template <typename Arg>
bool directiveUses(const std::shared_ptr<ast::ClientDirective> &directive,
                   const Arg &arg,
                   const InputTypeSpecChecker<Arg> &inputChecker) {
    for (const auto &argument : directive->arguments | std::views::values) {
        if (nonCallableFieldSpecUses(argument->spec, arg, inputChecker))
            return true;
    };
    return false;
};

bool inputTypeSpecUsesScalar(const ast::InputTypeSpec &spec,
                             const std::string &name) {
    if (std::holds_alternative<std::shared_ptr<ast::InputType>>(spec)) {
        const auto &input = std::get<std::shared_ptr<ast::InputType>>(spec);
        for (const auto &field : input->fields | std::views::values) {
            if (nonCallableFieldSpecUses<ast::InputTypeSpec, std::string>(
                    field.spec, name, inputTypeSpecUsesScalar))
                return true;
        };
    };
    return std::holds_alternative<std::shared_ptr<ast::Scalar>>(spec) &&
           std::get<std::shared_ptr<ast::Scalar>>(spec)->name == name;
};

template <typename T>
bool objectTypeUses(const ast::ObjectTypeSpec &spec, const T &name,
                    const InputTypeSpecChecker<T> &inputChecker,
                    const ObjectTypeSpecChecker<T> &objectChecker) {
    if (std::holds_alternative<std::shared_ptr<ast::ObjectType>>(spec)) {
        const auto &object = std::get<std::shared_ptr<ast::ObjectType>>(spec);
        for (const auto &field : object->fields | std::views::values) {
            if (objectFieldSpecUses<T>(field->spec, name, inputChecker,
                                       objectChecker))
                return true;
        };
    };
    if (std::holds_alternative<std::shared_ptr<ast::Interface>>(spec)) {
        const auto &interface = std::get<std::shared_ptr<ast::Interface>>(spec);
        for (const auto &field : interface->fields | std::views::values) {
            if (objectFieldSpecUses<T>(field->spec, name, inputChecker,
                                       objectChecker))
                return true;
        };
    };
    if (std::holds_alternative<std::shared_ptr<ast::Union>>(spec)) {
        const auto &unionType = std::get<std::shared_ptr<ast::Union>>(spec);
        for (const auto &object : unionType->items | std::views::values) {
            for (const auto &field : object->fields | std::views::values) {
                if (objectFieldSpecUses<T>(field->spec, name, inputChecker,
                                           objectChecker))
                    return true;
            };
        };
    };
    return false;
};

bool objectTypeSpecUsesScalar(const ast::ObjectTypeSpec &spec,
                              const std::string &name) {
    if (objectTypeUses<std::string>(spec, name, inputTypeSpecUsesScalar,
                                    objectTypeSpecUsesScalar)) {
        return true;
    };
    return std::holds_alternative<std::shared_ptr<ast::Scalar>>(spec) &&
           std::get<std::shared_ptr<ast::Scalar>>(spec)->name == name;
};

bool usesScalar(const ClientSchema &schema, const std::string &name) {
    bool hasScalar = false;
    visitor::ASTVisitorHooks hooks = {
        .visitInputTypeSpec =
            [&hasScalar, &name](const ast::InputTypeSpec &spec) {
                if (hasScalar) return;
                if (std::holds_alternative<std::shared_ptr<ast::Scalar>>(
                        spec) &&
                    std::get<std::shared_ptr<ast::Scalar>>(spec)->name ==
                        name) {
                    hasScalar = true;
                };
            },
        .visitObjectTypeSpec =
            [&hasScalar, &name](const ast::ObjectTypeSpec &spec) {
                if (hasScalar) return;
                if (std::holds_alternative<std::shared_ptr<ast::Scalar>>(
                        spec) &&
                    std::get<std::shared_ptr<ast::Scalar>>(spec)->name ==
                        name) {
                    hasScalar = true;
                };
            },
    };
    visitor::visitClientSchema(hooks, schema);
    return hasScalar;
};

bool inputTypeSpecUsesInput(const ast::InputTypeSpec &spec,
                            const std::shared_ptr<ast::InputType> &input) {
    return std::holds_alternative<std::shared_ptr<ast::InputType>>(spec) &&
           std::get<std::shared_ptr<ast::InputType>>(spec)->name == input->name;
};

bool objectTypeSpecUsesInput(const ast::ObjectTypeSpec &spec,
                             const std::shared_ptr<ast::InputType> &input) {
    return false;
};

bool usesInput(const ClientSchema &schema,
               const std::shared_ptr<ast::InputType> &input) {
    bool hasInput = false;
    visitor::ASTVisitorHooks hooks = {
        .visitInputTypeSpec =
            [&hasInput, &input](const ast::InputTypeSpec &spec) {
                if (hasInput) return;
                if (std::holds_alternative<std::shared_ptr<ast::InputType>>(
                        spec) &&
                    std::get<std::shared_ptr<ast::InputType>>(spec)->name ==
                        input->name) {
                    hasInput = true;
                };
            },
    };
    visitor::visitClientSchema(hooks, schema);
    return hasInput;
};

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

bool usesEnum(const ClientSchema &schema,
              const std::shared_ptr<ast::Enum> &enumType) {
    bool hasEnum = false;
    visitor::ASTVisitorHooks hooks = {
        .visitInputTypeSpec =
            [&hasEnum, &enumType](const ast::InputTypeSpec &spec) {
                if (hasEnum) return;
                if (std::holds_alternative<std::shared_ptr<ast::Enum>>(spec) &&
                    std::get<std::shared_ptr<ast::Enum>>(spec)->name ==
                        enumType->name) {
                    hasEnum = true;
                };
            },
        .visitObjectTypeSpec =
            [&hasEnum, &enumType](const ast::ObjectTypeSpec &spec) {
                if (hasEnum) return;
                if (std::holds_alternative<std::shared_ptr<ast::Enum>>(spec) &&
                    std::get<std::shared_ptr<ast::Enum>>(spec)->name ==
                        enumType->name) {
                    hasEnum = true;
                };
            },
    };
    visitor::visitClientSchema(hooks, schema);
    return hasEnum;
};
};  // namespace gql::parsers::schema::uses
