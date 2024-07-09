#include "./parser.hpp"

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "../client/ast.hpp"
#include "../shared/ast.hpp"
#include "../shared/parser_error.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"

using namespace parsers::file;
using namespace parsers::file::client;
using namespace lexer;

std::vector<ast::ClientDefinition> Parser::parse() {
    std::vector<ast::ClientDefinition> operations;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        operations.emplace_back(parseClientDefinition());
    };
    return operations;
};

ast::ClientDefinition Parser::parseClientDefinition() {
    if (currentToken.lexeme == "fragment") {
        return parseFragmentDefinition();
    };
    return parseOperationDefinition();
};

ast::FragmentDefinition Parser::parseFragmentDefinition() {
    shared::ast::NodeLocation location = {
        .startToken = currentToken,
        .source = source,
    };
    const auto &name = parseNameNode();
    consumeIdentifierByLexeme("on");
    const auto &typeName = parseNameNode();
    const auto &spec = parseFragmentSpec();
    location.endToken = spec.location.endToken;
    return {
        .location = location, .name = name, .typeName = typeName, .spec = spec
    };
};

ast::OperationDefinition Parser::parseOperationDefinition() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &type = client::ast::opTypeFromClientOp(currentToken.lexeme);
    if (!type.has_value()) {
        throw shared::ParserError(currentToken,
                                  "Unexpected client operation type", source);
    };
    const auto &name = parseNameNode();
    std::map<std::string, shared::ast::InputValueDefinitionNode> parameters;
    if (consumeIfIsAhead(SimpleTokenType::LEFT_PAREN)) {
        while (isAhead(ComplexTokenType::IDENTIFIER)) {
            const auto &node = parseInputValueDefinitionNode();
            if (parameters.contains(node.name.name)) {
                throw shared::ParserError(
                    node.name.location.startToken,
                    "Parameter with this name already exists",
                    node.name.location.source);
            };
            parameters[node.name.name] = node;
            consumeIfIsAhead(SimpleTokenType::COMMA);
        };
        consume(SimpleTokenType::RIGHT_PAREN);
    };
    ast::FragmentSpec fragment = parseFragmentSpec();
    location.endToken = fragment.location.endToken;
    return { .location = location,
             .type = type.value(),
             .name = name,
             .parameters = parameters,
             .fragment = fragment };
};

ast::Argument Parser::parseArgument() {
    const auto &name = parseNameNode();
    consume(SimpleTokenType::COLON);
    const auto &paramName = parseNameNode();
    return { .name = name, .paramName = paramName };
};

ast::FragmentSpec Parser::parseFragmentSpec() {
    consume(SimpleTokenType::LEFT_BRACE);
    shared::ast::NodeLocation location = {
        .startToken = currentToken,
        .source = source,
    };
    ast::FragmentSpec spec;
    while (isAhead(ComplexTokenType::IDENTIFIER) ||
           isAhead(ComplexTokenType::SPREAD)) {
        spec.selections.emplace_back(parseSelectionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_BRACE);
    location.endToken = currentToken;
    spec.location = location;
    return spec;
};

ast::ConditionalSpreadSelectionNode
Parser::parseConditionalSpreadSelectionNode() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    consumeIdentifierByLexeme("on");
    const auto &typeName = parseNameNode();
    const auto &fragmentSpec = parseFragmentSpec();
    location.endToken = fragmentSpec.location.endToken;
    return { .location = location,
             .typeName = typeName,
             .fragment = std::make_shared<ast::FragmentSpec>(fragmentSpec) };
}

ast::SelectionNode Parser::parseSelectionNode() {
    if (!consumeIfIsAhead(ComplexTokenType::SPREAD)) {
        return parseFieldSelectionNode();
    };
    if (isAheadByLexeme("on")) {
        return parseConditionalSpreadSelectionNode();
    }
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &fragmentName = parseNameNode();
    location.endToken = fragmentName.location.endToken;
    return (ast::SpreadSelectionNode){ .fragmentName = fragmentName };
};

std::pair<shared::ast::NameNode, shared::ast::NameNode>
Parser::parseNameAndSelectionName() {
    shared::ast::NameNode selectionName = parseNameNode();
    shared::ast::NameNode fieldName = selectionName;
    std::optional<std::vector<shared::ast::InputValueDefinitionNode>> arguments;
    if (consumeIfIsAhead(SimpleTokenType::COLON)) {
        fieldName = parseNameNode();
    };
    return { fieldName, selectionName };
};

ast::ObjectFieldSpec Parser::parseObjectFieldSpec() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };

    const auto &[fieldName, selectionName] = parseNameAndSelectionName();
    location.endToken = fieldName.location.endToken;
    if (!consumeIfIsAhead(SimpleTokenType::LEFT_PAREN))
        return (ast::ObjectLiteralFieldSpec){ .location = location,
                                              .selectionName = selectionName,
                                              .name = fieldName };
    std::vector<ast::Argument> args;
    while (isAhead(ComplexTokenType::IDENTIFIER)) {
        args.push_back(parseArgument());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    consume(SimpleTokenType::RIGHT_PAREN);
    location.endToken = currentToken;
    return (ast::ObjectCallableFieldSpec){ .selectionName = selectionName,
                                           .name = fieldName,
                                           .arguments = args };
};

ast::FieldSelectionNode Parser::parseFieldSelectionNode() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };
    const auto &fieldSpec = parseObjectFieldSpec();
    std::optional<std::shared_ptr<ast::FragmentSpec>> spec;
    if (isAhead(SimpleTokenType::LEFT_BRACE)) {
        spec = std::make_shared<ast::FragmentSpec>(parseFragmentSpec());
    };
    location.endToken = currentToken;
    return { .location = location, .field = fieldSpec, .spec = spec };
};
