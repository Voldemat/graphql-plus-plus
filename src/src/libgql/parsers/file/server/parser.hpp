#ifndef GRAPHQL_SERVER_PARSER
#define GRAPHQL_SERVER_PARSER

#include <string>
#include <utility>

#include "./ast.hpp"
#include "libgql/parsers/file/base/parser.hpp"

namespace parsers::file::server {

class Parser : public BaseParser {
    ast::ASTNode parseASTNode();
    ast::ScalarDefinitionNode parseScalarTypeDefinitionNode();
    ast::UnionDefinitionNode parseUnionTypeDefinitionNode();
    ast::ExtendTypeNode parseExtendTypeNode();
    ast::EnumDefinitionNode parseEnumTypeDefinitionNode();
    ast::EnumValueDefinitionNode parseEnumValueDefinitionNode();
    ast::InterfaceDefinitionNode parseInterfaceTypeDefinitionNode();
    ast::FieldDefinitionNode parseFieldDefinitionNode();
    ast::ObjectDefinitionNode parseObjectTypeDefinitionNode();

public:
    using BaseParser::BaseParser;
    ast::FileNodes parse();
};
};  // namespace parsers::file::server
#endif
