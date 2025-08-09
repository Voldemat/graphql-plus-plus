#pragma once

#include <vector>
#include "../shared/ast.hpp"
#include "./parser.hpp"
#include "libgql/lexer/token_type.hpp"

namespace gql::parsers::file {
template <class T>
class BaseDirectiveParser : public BaseParser {
public:
    using BaseParser::BaseParser;
protected:
    shared::ast::DirectiveNode<T> parseDirectiveNode() {
        consume(lexer::SimpleTokenType::AT_SIGN);
        const auto& nameNode = parseNameNode();
        const auto& arguments = parseInputValueDefinitionNodes();
        consumeIdentifierByLexeme("on");
        const auto& locations = parseDirectiveLocations();
        return (shared::ast::DirectiveNode<T>){
            .location = {
                .startToken = nameNode.location.startToken,
                .endToken = locations.back().location.endToken,
                .source = source,
            },
            .name = nameNode,
            .targets = locations,
            .arguments = arguments,
        };
    };
    shared::ast::DirectiveInvocationNode parseDirectiveInvocationNode() {
        const auto& startToken = currentToken;
        const auto& name = parseNameNode(true);
        std::vector<shared::ast::Argument> arguments;
        if (consumeIfIsAhead(lexer::SimpleTokenType::LEFT_PAREN)) {
            arguments = parseArguments();
            consume(lexer::SimpleTokenType::RIGHT_PAREN);
        };
        return {
            .location = {
                .startToken = startToken,
                .endToken = currentToken,
                .source = source
            },
            .name = name,
            .arguments = arguments,
        };
    };
private:

    std::vector<shared::ast::DirectiveLocationNode<T>> parseDirectiveLocations() {
        std::vector<shared::ast::DirectiveLocationNode<T>> locations = {
            parseDirectiveLocationNode()
        };
        while (consumeIfIsAhead(lexer::SimpleTokenType::COMMA)) {
            locations.push_back(parseDirectiveLocationNode());
        };
        return locations;
    };

    shared::ast::DirectiveLocationNode<T> parseDirectiveLocationNode() {
        const auto& location = parseDirectiveLocation();
        return {
            .location = {
                .startToken = currentToken,
                .endToken = currentToken,
                .source = source
            },
            .directiveLocation = location
        };
    };

    virtual T parseDirectiveLocation() = 0;
};
};  // namespace parsers::file
