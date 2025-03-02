#include "./ast.hpp"

#include <optional>
#include <string>

namespace parsers::file::server::ast {
std::optional<DirectiveLocation> stringToDirectiveLocation(
    const std::string &str) {
    if (str == "SCHEMA")
        return DirectiveLocation::SCHEMA;
    else if (str == "SCALAR")
        return DirectiveLocation::SCALAR;
    else if (str == "OBJECT")
        return DirectiveLocation::OBJECT;
    else if (str == "FIELD_DEFINITION")
        return DirectiveLocation::FIELD_DEFINITION;
    else if (str == "ARGUMENT_DEFINITION")
        return DirectiveLocation::ARGUMENT_DEFINITION;
    else if (str == "INTERFACE")
        return DirectiveLocation::INTERFACE;
    else if (str == "UNION")
        return DirectiveLocation::UNION;
    else if (str == "ENUM")
        return DirectiveLocation::ENUM;
    else if (str == "ENUM_VALUE")
        return DirectiveLocation::ENUM_VALUE;
    else if (str == "INPUT_OBJECT")
        return DirectiveLocation::INPUT_OBJECT;
    else if (str == "INPUT_FIELD_DEFINITION")
        return DirectiveLocation::INPUT_FIELD_DEFINITION;
    return std::nullopt;
};
};  // namespace parsers::file::server::ast
