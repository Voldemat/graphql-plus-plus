#include "./selection_argument.hpp"

#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"

namespace parsers::schema::nodes {
ast::FieldSelectionArgument parseSelectionArgument(
    const file::client::ast::Argument &node,
    const ast::CallableFieldSpec &spec) {
    if (!spec.arguments.contains(node.name.name)) {
        throw file::shared::ParserError(
            node.name.location.startToken,
            "Argument with this name does not exists",
            node.name.location.source);
    };
    return { .name = node.name.name,
             .parameterName = node.paramName.name,
             .type = spec.arguments.at(node.name.name) };
};
};  // namespace parsers::schema::nodes
