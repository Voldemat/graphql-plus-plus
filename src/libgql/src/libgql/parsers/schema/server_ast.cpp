#include <variant>

#include "./server_ast.hpp"
#include "utils.hpp"

namespace parsers::schema::ast {

bool InputFieldSpec_hasDefaultValue(const InputFieldSpec &spec) {
    return std::visit<bool>(
        overloaded{ [](const auto &node) { return node.hasDefaultValue(); } },
        spec);
};

InputTypeSpec extractInputTypeSpec(const InputFieldSpec &spec) {
    return std::visit<InputTypeSpec>(
        overloaded{ [](const LiteralFieldSpec<InputTypeSpec> &node) {
                       return node.type;
                   },
                    [](const ArrayFieldSpec<InputTypeSpec> &node) {
                        return node.type;
                    } },
        spec);
};
};  // namespace parsers::schema::ast
