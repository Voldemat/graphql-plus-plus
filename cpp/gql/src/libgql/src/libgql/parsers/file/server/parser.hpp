#pragma once

#include <vector>

#include "./ast.hpp"
#include "libgql/parsers/file/base/directive.hpp"
#include "libgql/parsers/file/shared/ast.hpp"

namespace gql::parsers::file::server {

class Parser : public BaseDirectiveParser<ast::DirectiveLocation> {
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
    ast::DirectiveLocation parseDirectiveLocation() override;

public:
    using BaseDirectiveParser<ast::DirectiveLocation>::BaseDirectiveParser;
    std::vector<ast::ASTNode> parse();
};
};  // namespace gql::parsers::file::server
