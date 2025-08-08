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

std::vector<ast::ASTNode> Parser::parse() {
    std::vector<ast::ASTNode> operations;
    while (currentToken != tokens.back()) {
        if (index != 0) consume(ComplexTokenType::IDENTIFIER);
        operations.emplace_back(parseASTNode());
    };
    return operations;
};

ast::ASTNode Parser::parseASTNode() {
    if (currentToken.lexeme == "fragment") {
        return parseFragmentDefinition();
    } else if (currentToken.lexeme == "directive") {
        return parseDirectiveNode();
    };
    return parseOperationDefinition();
};

ast::FragmentDefinition Parser::parseFragmentDefinition() {
    const auto startToken = currentToken;
    const auto &name = parseNameNode();
    consumeIdentifierByLexeme("on");
    const auto &typeName = parseNameNode();
    const auto &spec = parseFragmentSpec();
    return { .location = { .startToken = startToken,
                           .endToken = spec.location.endToken,
                           .source = source },
             .name = name,
             .typeName = typeName,
             .spec = spec };
};

ast::OperationDefinition Parser::parseOperationDefinition() {
    const auto startToken = currentToken;
    const auto &type = client::ast::opTypeFromClientOp(currentToken.lexeme);
    if (!type.has_value()) {
        throw shared::ParserError(currentToken,
                                  "Unexpected client operation type", source);
    };
    const auto &name = parseNameNode();
    const auto &parameters = parseOperationParameters();
    ast::FragmentSpec fragment = parseFragmentSpec();
    return { .location = { .startToken = startToken,
                           .endToken = fragment.location.endToken,
                           .source = source },
             .type = type.value(),
             .name = name,
             .parameters = parameters,
             .fragment = fragment };
};

std::map<std::string, shared::ast::InputValueDefinitionNode>
Parser::parseOperationParameters() {
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
    return parameters;
};

ast::FragmentSpec Parser::parseFragmentSpec() {
    consume(SimpleTokenType::LEFT_BRACE);
    const auto startToken = currentToken;
    const auto &selections = parseSelectionNodes();
    consume(SimpleTokenType::RIGHT_BRACE);
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .selections = selections };
};

std::vector<client::ast::SelectionNode> Parser::parseSelectionNodes() {
    std::vector<client::ast::SelectionNode> selections;
    while (isAhead(ComplexTokenType::IDENTIFIER) ||
           isAhead(ComplexTokenType::SPREAD)) {
        selections.emplace_back(parseSelectionNode());
        consumeIfIsAhead(SimpleTokenType::COMMA);
    };
    return selections;
};

ast::SelectionNode Parser::parseSelectionNode() {
    if (!consumeIfIsAhead(ComplexTokenType::SPREAD)) {
        return parseFieldSelectionNode();
    };
    if (isAheadByLexeme("on")) {
        return parseConditionalSpreadSelectionNode();
    }
    const auto startToken = currentToken;
    const auto &fragmentName = parseNameNode();
    return (ast::SpreadSelectionNode){ .location = { .startToken = startToken,
                                                     .endToken = currentToken,
                                                     .source = source },
                                       .fragmentName = fragmentName };
};

ast::FieldSelectionNode Parser::parseFieldSelectionNode() {
    const auto startToken = currentToken;
    const auto &fieldSpec = parseObjectFieldSpec();
    std::optional<std::shared_ptr<ast::FragmentSpec>> spec;
    if (isAhead(SimpleTokenType::LEFT_BRACE)) {
        spec = std::make_shared<ast::FragmentSpec>(parseFragmentSpec());
    };
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .field = fieldSpec,
             .spec = spec };
};

ast::ConditionalSpreadSelectionNode
Parser::parseConditionalSpreadSelectionNode() {
    const auto startToken = currentToken;
    consumeIdentifierByLexeme("on");
    const auto &typeName = parseNameNode();
    const auto &fragmentSpec = parseFragmentSpec();
    return { .location = { .startToken = startToken,
                           .endToken = currentToken,
                           .source = source },
             .typeName = typeName,
             .fragment = std::make_shared<ast::FragmentSpec>(fragmentSpec) };
}

ast::ObjectFieldSpec Parser::parseObjectFieldSpec() {
    shared::ast::NodeLocation location = { .startToken = currentToken,
                                           .source = source };

    const auto &[fieldName, selectionName] = parseNameAndSelectionName();
    location.endToken = fieldName.location.endToken;
    if (!consumeIfIsAhead(SimpleTokenType::LEFT_PAREN))
        return (ast::ObjectLiteralFieldSpec){ .location = location,
                                              .selectionName = selectionName,
                                              .name = fieldName };
    const auto &args = parseArguments();
    consume(SimpleTokenType::RIGHT_PAREN);
    location.endToken = currentToken;
    return (ast::ObjectCallableFieldSpec){
        .location = location,
        .selectionName = selectionName,
        .name = fieldName,
        .arguments = args,
    };
};

std::pair<shared::ast::NameNode, shared::ast::NameNode>
Parser::parseNameAndSelectionName() {
    shared::ast::NameNode selectionName = parseNameNode();
    shared::ast::NameNode fieldName = selectionName;
    if (consumeIfIsAhead(SimpleTokenType::COLON)) {
        fieldName = parseNameNode();
    };
    return { fieldName, selectionName };
};

ast::DirectiveLocation Parser::parseDirectiveLocation() {
    consumeIdentifier();
    const auto &value = ast::stringToDirectiveLocation(currentToken.lexeme);
    if (!value.has_value()) {
        throw shared::ParserError(currentToken, "Unknown directive location",
                                  source);
    };
    return value.value();
};
