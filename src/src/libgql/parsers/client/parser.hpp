#ifndef LIBGQL_PARSERS_CLIENT
#define LIBGQL_PARSERS_CLIENT

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"
#include "libgql/parsers/client/ast.hpp"
#include "libgql/parsers/shared/shared.hpp"

namespace parsers {
namespace client {

class Parser {
    unsigned int index = 0;
    std::vector<lexer::GQLToken> tokens;
    std::shared_ptr<shared::ast::SourceFile> source;
    lexer::GQLToken currentToken;
    const lexer::GQLToken lookahead();
    void advance();
    void consume(const lexer::GQLTokenType expectedType);
    void consumeIdentifier();
    void consumeIdentifierByLexeme(const std::string &lexeme);
    bool consumeIdentifierByLexemeIfIsAhead(const std::string &lexeme);
    bool consumeIfIsAhead(lexer::GQLTokenType expectedType);
    bool isAhead(lexer::GQLTokenType expectedType);
    bool isAheadByLexeme(const std::string &lexeme);
    shared::ast::NameNode parseNameNode(bool raiseOnKeyword = false);
    ast::OperationDefinition parseOperationDefinition();
    shared::ast::InputValueDefinitionNode parseInputValueDefinitionNode();
    shared::ast::TypeNode parseTypeNode();
    shared::ast::NamedTypeNode parseNamedTypeNode();
    shared::ast::ListTypeNode parseListTypeNode();
    shared::ast::LiteralNode parseLiteralNode();
    ast::Argument parseArgument();
    ast::FragmentSpec parseFragmentSpec();
    ast::ClientDefinition parseClientDefinition();
    ast::FragmentDefinition parseFragmentDefinition();
    ast::SelectionNode parseSelectionNode();
    ast::FieldSelectionNode parseFieldSelectionNode();
    ast::ConditionalSpreadSelectionNode parseConditionalSpreadSelectionNode();
    ast::ObjectFieldSpec parseObjectFieldSpec();
    std::pair<shared::ast::NameNode, shared::ast::NameNode>
    parseNameAndSelectionName();

public:
    Parser(std::vector<lexer::GQLToken> tokens,
           std::shared_ptr<shared::ast::SourceFile> source) noexcept;
    std::vector<ast::ClientDefinition> parse();
};
};  // namespace client
};  // namespace parsers
#endif
