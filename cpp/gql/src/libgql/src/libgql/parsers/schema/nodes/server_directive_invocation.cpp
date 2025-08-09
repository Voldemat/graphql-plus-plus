#include "./server_directive_invocation.hpp"

#include <ranges>
#include <vector>

#include "../../file/shared/ast.hpp"
#include "../../file/shared/parser_error.hpp"
#include "../shared_ast.hpp"
#include "../type_registry.hpp"
#include "./argument.hpp"

namespace gql::parsers::schema::nodes {
std::vector<ast::ServerDirectiveInvocation> parseServerDirectiveInvocations(
    const std::vector<file::shared::ast::DirectiveInvocationNode> &invocations,
    const TypeRegistry &registry) {
    return invocations |
           std::views::transform(
               [&registry](
                   const file::shared::ast::DirectiveInvocationNode &node)
                   -> ast::ServerDirectiveInvocation {
                   if (!registry.serverDirectives.contains(node.name.name)) {
                       throw file::shared::ParserError(
                           node.name.location.startToken,
                           "Unknown server directive",
                           node.name.location.source);
                   };
                   const auto &directive =
                       registry.getServerDirective(node.name.name);
                   const auto &arguments =
                       parseArguments(node.arguments, directive, registry);
                   return {
                       .directive = directive,
                       .arguments = arguments,
                   };
               }) |
           std::ranges::to<std::vector>();
};
};  // namespace gql::parsers::schema::nodes
