#include "./server_ast.hpp"

#include <variant>

#include "./shared_ast.hpp"
#include "utils.hpp"

namespace gql::parsers::schema::ast {

bool InputFieldSpec_hasDefaultValue(const InputFieldSpec &spec) {
    return std::visit<bool>(utils::overloaded{ [](const auto &node) {
                                return node.hasDefaultValue();
                            } },
                            spec);
};

InputTypeSpec extractInputTypeSpec(const InputFieldSpec &spec) {
    return std::visit<InputTypeSpec>(
        utils::overloaded{ [](const LiteralFieldSpec<InputTypeSpec> &node) {
                              return node.type;
                          },
                           [](const ArrayFieldSpec<InputTypeSpec> &node) {
                               return node.type;
                           } },
        spec);
};
};  // namespace gql::parsers::schema::ast
