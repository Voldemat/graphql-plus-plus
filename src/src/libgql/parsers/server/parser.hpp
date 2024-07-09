#ifndef GRAPHQL_SERVER_PARSER
#define GRAPHQL_SERVER_PARSER

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/token_type.hpp"
#include "libgql/parsers/shared/shared.hpp"

namespace parsers {
namespace server {

class Parser {
    unsigned int index = 0;
    std::vector<lexer::GQLToken> tokens;
    std::shared_ptr<shared::ast::SourceFile> source;
    lexer::GQLToken currentToken;
    std::optional<lexer::GQLToken> lookahead();
    void advance();
    void consume(const lexer::GQLTokenType expectedType);
    void consumeIdentifier();
    bool consumeIfIsAhead(lexer::GQLTokenType expectedType);
    bool isAhead(lexer::GQLTokenType expectedType);
    shared::ast::NameNode parseNameNode(bool raiseOnKeyword = false);
    ast::ScalarDefinitionNode parseScalarTypeDefinitionNode();
    ast::UnionDefinitionNode parseUnionTypeDefinitionNode();
    std::pair<std::string, ast::ASTNode> parseASTNode();
    ast::ExtendTypeNode parseExtendTypeNode();
    ast::EnumDefinitionNode parseEnumTypeDefinitionNode();
    ast::EnumValueDefinitionNode parseEnumValueDefinitionNode();
    ast::InterfaceDefinitionNode parseInterfaceTypeDefinitionNode();
    ast::FieldDefinitionNode parseFieldDefinitionNode();
    shared::ast::TypeNode parseTypeNode();
    shared::ast::NamedTypeNode parseNamedTypeNode();
    shared::ast::ListTypeNode parseListTypeNode();
    shared::ast::InputValueDefinitionNode parseInputValueDefinitionNode();
    shared::ast::LiteralNode parseLiteralNode();
    ast::ObjectDefinitionNode parseObjectTypeDefinitionNode();

public:
    Parser(const std::vector<lexer::GQLToken> &tokens,
           const std::shared_ptr<shared::ast::SourceFile> &source);
    ast::FileNodes parse();
};
};  // namespace server
};  // namespace parsers
#endif
