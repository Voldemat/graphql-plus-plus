#pragma once

#include <vector>

#include "./ast.hpp"
#include "libgql/parsers/file/base/parser.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

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
    std::vector<shared::ast::NameNode> parseImplementsClause();
    std::vector<shared::ast::InputValueDefinitionNode> parseArguments();

public:
    using BaseParser::BaseParser;
    ast::FileNodes parse();
};
};  // namespace parsers::file::server
