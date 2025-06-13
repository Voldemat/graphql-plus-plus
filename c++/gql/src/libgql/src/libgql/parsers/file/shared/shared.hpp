#pragma once

#include <memory>
#include <string>

#include "./ast.hpp"
#include "libgql/lexer/token.hpp"

namespace parsers::file::shared {

void assertIsNotKeyword(const lexer::GQLToken token,
                        const std::shared_ptr<ast::SourceFile> &source);
const bool isKeyword(const std::string lexeme);
};  // namespace parsers::file::shared
