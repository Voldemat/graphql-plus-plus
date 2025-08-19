#include "./operation_hash.hpp"

#include <algorithm>
#include <cstddef>
#include <format>
#include <functional>
#include <map>
#include <memory>
#include <numeric>
#include <ranges>
#include <set>
#include <string>
#include <tuple>
#include <variant>
#include <vector>

#include "./client_ast.hpp"
#include "./server_ast.hpp"
#include "./shared_ast.hpp"
#include "./type_registry.hpp"
#include "magic_enum.hpp"
#include "utils.hpp"

namespace gql::parsers::schema {
std::string getInputTypeSpecHashInput(const ast::InputTypeSpec &typeSpec) {
    return std::visit<std::string>(
        [](const auto &spec) -> std::string { return spec->name; }, typeSpec);
};

std::string getArrayLiteralHashInput(const ast::ArrayLiteral &literal) {
    return std::visit<std::string>(
        utils::overloaded{
            [](const std::vector<std::string> &strings) -> std::string {
                return std::accumulate(strings.begin(), strings.end(),
                                       std::string{});
            },
            [](const std::vector<int> &ints) -> std::string {
                return std::accumulate(
                    ints.begin(), ints.end(), std::string{},
                    [](const std::string &buffer, const int &value) {
                        return buffer + std::to_string(value);
                    });
            },
            [](const std::vector<float> &floats) -> std::string {
                return std::accumulate(
                    floats.begin(), floats.end(), std::string{},
                    [](const std::string &buffer, const float &value) {
                        return buffer + std::to_string(value);
                    });
            },
            [](const std::vector<bool> &booleans) -> std::string {
                return std::accumulate(
                    booleans.begin(), booleans.end(), std::string{},
                    [](const std::string &buffer, const bool &value) {
                        return buffer + std::to_string(value);
                    });
            },
        },
        literal);
};

std::string getLiteralHashInput(const ast::Literal &literal) {
    return std::visit<std::string>(
        utils::overloaded{
            [](const std::string &str) -> std::string { return str; },
            [](const int &value) -> std::string {
                return std::to_string(value);
            },
            [](const float &value) -> std::string {
                return std::to_string(value);
            },
            [](const bool &value) -> std::string {
                return std::to_string(value);
            },
        },
        literal);
};

std::string getInputFieldSpecHashInput(const ast::InputFieldSpec &fieldSpec) {
    return std::visit<std::string>(
        utils::overloaded{
            [](const ast::LiteralFieldSpec<ast::InputTypeSpec> &spec)
                -> std::string {
                return "l" + getInputTypeSpecHashInput(spec.type) +
                       spec.defaultValue
                           .transform([](const auto &literal) {
                               return getLiteralHashInput(literal);
                           })
                           .value_or("");
            },
            [](const ast::ArrayFieldSpec<ast::InputTypeSpec> &spec)
                -> std::string {
                return std::format("a{}", spec.nullable) +
                       getInputTypeSpecHashInput(spec.type) +
                       spec.defaultValue
                           .transform([](const auto &literal) {
                               return getArrayLiteralHashInput(literal);
                           })
                           .value_or("");
            } },
        fieldSpec);
};

std::string getInputFieldDefinitionHashInput(
    const ast::FieldDefinition<ast::InputFieldSpec> &field) {
    std::string hashInput = field.name;
    hashInput += field.nullable;
    hashInput += getInputFieldSpecHashInput(field.spec);
    return hashInput;
};

std::string getArgumentValueHashInput(const ast::ArgumentValue &argValue) {
    return std::visit<std::string>(
        utils::overloaded{
            [](const ast::ArgumentRefValue &value) -> std::string {
                return "r" + value.name;
            },
            [](const ast::ArgumentLiteralValue &value) -> std::string {
                return "l" +
                       std::visit<std::string>(
                           utils::overloaded{
                               [](const int &value) {
                                   return std::to_string(value);
                               },
                               [](const float &value) {
                                   return std::to_string(value);
                               },
                               [](const std::string &value) { return value; },
                               [](const bool &value) {
                                   return std::to_string(value);
                               },
                               [](const ast::ArgumentEnumValue &value) {
                                   return "e" + value.value;
                               },
                           },
                           value);
            } },
        argValue);
};

std::string getFieldSelectionArgumentHashInput(
    const ast::FieldSelectionArgument &argument) {
    return argument.name + getArgumentValueHashInput(argument.value);
};

std::string getArgumentsHashInput(
    const std::map<std::string, ast::FieldSelectionArgument> &arguments) {
    auto keys = arguments | std::views::keys | std::ranges::to<std::vector>();
    std::ranges::sort(keys, std::ranges::greater{});
    return std::accumulate(
        keys.begin(), keys.end(), std::string{},
        [&arguments](const std::string &buffer, const std::string &key) {
            return buffer +
                   getFieldSelectionArgumentHashInput(arguments.at(key));
        });
};

using HashInputAndFragmentNames =
    std::tuple<std::string, std::set<std::string>>;

HashInputAndFragmentNames getFragmentSpecHashInput(
    const ast::FragmentSpec &fragmentSpec);

const auto getHashInputAndFragmentNamesFromTypenameField =
    [](const ast::TypenameField &field) -> HashInputAndFragmentNames {
    return { "t" + field.alias.value_or(""), {} };
};

const auto getHashInputAndFragmentNamesFromSpreadSelection =
    [](const ast::SpreadSelection &spread) -> HashInputAndFragmentNames {
    return { "s" + spread.fragment->name, { spread.fragment->name } };
};

auto getObjectSelectionHashInput(const ast::ObjectSelection &selection) {
    return std::visit<HashInputAndFragmentNames>(
        utils::overloaded{
            getHashInputAndFragmentNamesFromTypenameField,
            [](const ast::FieldSelection &field) -> HashInputAndFragmentNames {
                std::set<std::string> fragmentNames;
                const auto &selectionHashInput =
                    field.selection
                        .transform([&fragmentNames](const auto &fragment) {
                            const auto &[hashInput, fNames] =
                                getFragmentSpecHashInput(*fragment.get());
                            fragmentNames = fNames;
                            return hashInput;
                        })
                        .value_or("");
                return { "f" + field.name +
                             (field.alias == field.name ? "" : field.alias) +
                             getArgumentsHashInput(field.arguments) +
                             selectionHashInput,
                         fragmentNames };
            },
            getHashInputAndFragmentNamesFromSpreadSelection },
        selection);
};

template <typename T>
const auto getObjectFragmentSpecHashInput =
    [](const ast::ObjectFragmentSpec<T> &spec) -> HashInputAndFragmentNames {
    std::set<std::string> fragmentNames;
    auto hashInputs =
        spec.selections |
        std::views::transform([&fragmentNames](const auto &selection) {
            const auto &[h, fNames] = getObjectSelectionHashInput(selection);
            fragmentNames.insert_range(fNames);
            return h;
        }) |
        std::ranges::to<std::vector>();
    std::ranges::sort(hashInputs, std::ranges::greater{});
    return { (std::is_same_v<ast::ObjectType, T> ? "ob" : "oi") +
                 spec.type->name +
                 std::accumulate(hashInputs.begin(), hashInputs.end(),
                                 std::string{}),
             fragmentNames };
};

HashInputAndFragmentNames getUnionSelectionHashInput(
    const ast::UnionSelection &unionSelection) {
    return std::visit<HashInputAndFragmentNames>(
        utils::overloaded{
            getHashInputAndFragmentNamesFromTypenameField,
            [](const ast::UnionConditionalSpreadSelection &s)
                -> HashInputAndFragmentNames { return { "", {} }; },
            [](const ast::ObjectConditionalSpreadSelection &selection)
                -> HashInputAndFragmentNames {
                const auto &[h, fNames] =
                    getObjectFragmentSpecHashInput<ast::ObjectType>(
                        *selection.selection.get());
                return { "oc" + selection.type->name + h, fNames };
            },
            getHashInputAndFragmentNamesFromSpreadSelection },
        unionSelection);
};

HashInputAndFragmentNames getFragmentSpecHashInput(
    const ast::FragmentSpec &fragmentSpec) {
    return std::visit<HashInputAndFragmentNames>(
        utils::overloaded{
            getObjectFragmentSpecHashInput<ast::ObjectType>,
            getObjectFragmentSpecHashInput<ast::Interface>,
            [](const ast::UnionFragmentSpec &spec)
                -> HashInputAndFragmentNames {
                std::set<std::string> fragmentNames;
                auto hashInputs =
                    spec.selections |
                    std::views::transform(
                        [&fragmentNames](const auto &selection) {
                            const auto &[h, fNames] =
                                getUnionSelectionHashInput(selection);
                            fragmentNames.insert_range(fNames);
                            return h;
                        }) |
                    std::ranges::to<std::vector>();
                std::ranges::sort(hashInputs, std::ranges::greater{});
                return { "u" + spec.type->name +
                             std::accumulate(hashInputs.begin(),
                                             hashInputs.end(), std::string{}),
                         fragmentNames };
            },
        },
        fragmentSpec);
};

std::size_t getClientOperationHash(
    const TypeRegistry &registry,
    const std::shared_ptr<const ast::Operation> &operation) {
    std::string hashInput =
        std::string(magic_enum::enum_name(operation->type)) + operation->name;
    auto paramsKeysVector = operation->parameters | std::views::keys |
                            std::ranges::to<std::vector>();
    std::ranges::sort(paramsKeysVector, std::ranges::greater{});
    for (const auto &paramKey : paramsKeysVector) {
        hashInput += getInputFieldDefinitionHashInput(
            operation->parameters.at(paramKey));
    };
    auto [hInput, fragmentNames] =
        getFragmentSpecHashInput(operation->fragmentSpec);
    hashInput += hInput;
    while (fragmentNames.size() != 0) {
        const auto &name = *fragmentNames.begin();
        const auto &[h, fNames] =
            getFragmentSpecHashInput(registry.getFragment(name)->spec);
        hashInput += h;
        fragmentNames.insert_range(fNames);
        fragmentNames.erase(name);
    };
    return std::hash<std::string>()(hashInput);
};
};  // namespace gql::parsers::schema
