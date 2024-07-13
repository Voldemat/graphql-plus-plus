#pragma once

#include <map>
#include <string>
#include <utility>
#include <vector>

#include "../client/ast.hpp"
#include "../shared/ast.hpp"
#include "libgql/parsers/file/base/parser.hpp"

namespace parsers::file::client {

class Parser : public BaseParser {
    ast::ClientDefinition parseClientDefinition();
    ast::FragmentDefinition parseFragmentDefinition();
    ast::OperationDefinition parseOperationDefinition();
    std::map<std::string, shared::ast::InputValueDefinitionNode>
    parseOperationParameters();
    ast::Argument parseArgument();
    std::vector<ast::Argument> parseArguments();
    ast::FragmentSpec parseFragmentSpec();
    std::vector<client::ast::SelectionNode> parseSelectionNodes();
    ast::SelectionNode parseSelectionNode();
    ast::FieldSelectionNode parseFieldSelectionNode();
    ast::ConditionalSpreadSelectionNode parseConditionalSpreadSelectionNode();
    ast::ObjectFieldSpec parseObjectFieldSpec();
    std::pair<shared::ast::NameNode, shared::ast::NameNode>
    parseNameAndSelectionName();

public:
    using BaseParser::BaseParser;
    std::vector<ast::ClientDefinition> parse();
};
};  // namespace parsers::file::client
