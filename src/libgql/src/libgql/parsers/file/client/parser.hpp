#pragma once

#include <map>
#include <string>
#include <utility>
#include <vector>

#include "../client/ast.hpp"
#include "../shared/ast.hpp"
#include "libgql/parsers/file/base/directive.hpp"

namespace parsers::file::client {
class Parser : public BaseDirectiveParser<ast::DirectiveLocation> {
    ast::ClientDefinition parseClientDefinition();
    ast::FragmentDefinition parseFragmentDefinition();
    ast::OperationDefinition parseOperationDefinition();
    std::map<std::string, shared::ast::InputValueDefinitionNode>
    parseOperationParameters();
    ast::FragmentSpec parseFragmentSpec();
    std::vector<client::ast::SelectionNode> parseSelectionNodes();
    ast::SelectionNode parseSelectionNode();
    ast::FieldSelectionNode parseFieldSelectionNode();
    ast::ConditionalSpreadSelectionNode parseConditionalSpreadSelectionNode();
    ast::ObjectFieldSpec parseObjectFieldSpec();
    std::pair<shared::ast::NameNode, shared::ast::NameNode>
    parseNameAndSelectionName();
    ast::DirectiveLocation parseDirectiveLocation() override;
public:
    using BaseDirectiveParser::BaseDirectiveParser;
    std::vector<ast::ClientDefinition> parse();
};
};  // namespace parsers::file::client
