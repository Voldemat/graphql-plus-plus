#pragma once

#include <utility>
#include <vector>

#include "../client/ast.hpp"
#include "../shared/ast.hpp"
#include "libgql/parsers/file/base/parser.hpp"

namespace parsers::file::client {

class Parser : public BaseParser {
    ast::OperationDefinition parseOperationDefinition();
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
    using BaseParser::BaseParser;
    std::vector<ast::ClientDefinition> parse();
};
};  // namespace parsers::file::client
