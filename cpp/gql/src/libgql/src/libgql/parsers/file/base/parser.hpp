#pragma once

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

namespace gql::parsers::file {

class BaseParser {
protected:
    unsigned int index = 0;
    std::vector<lexer::GQLToken> tokens;
    std::shared_ptr<shared::ast::SourceFile> source;
    lexer::GQLToken currentToken;
    std::optional<lexer::GQLToken> lookahead();
    void advance();
    void consume(const lexer::GQLTokenType expectedType);
    void consumeIdentifier();
    bool consumeIfIsAhead(lexer::GQLTokenType expectedType);
    bool consumeIdentifierByLexemeIfIsAhead(const std::string &lexeme);
    void consumeIdentifierByLexeme(const std::string &lexeme);
    bool isAhead(lexer::GQLTokenType expectedType);
    bool isAheadByLexeme(const std::string &lexeme);
    shared::ast::NameNode parseNameNode(bool raiseOnKeyword = false);
    shared::ast::TypeNode parseTypeNode();
    shared::ast::NamedTypeNode parseNamedTypeNode();
    shared::ast::ListTypeNode parseListTypeNode();
    shared::ast::InputValueDefinitionNode parseInputValueDefinitionNode();
    std::vector<shared::ast::InputValueDefinitionNode>
    parseInputValueDefinitionNodes();
    shared::ast::LiteralNode parseLiteralNode();
    std::optional<shared::ast::LiteralIntNode> parseLiteralIntNode();
    std::optional<shared::ast::LiteralFloatNode> parseLiteralFloatNode();
    std::vector<shared::ast::Argument> parseArguments();
    shared::ast::Argument parseArgument();
    shared::ast::ArgumentValue parseArgumentValue();

public:
    BaseParser(const std::vector<lexer::GQLToken> &tokens,
               const std::shared_ptr<shared::ast::SourceFile> &source);
};
};  // namespace gql::parsers::file
