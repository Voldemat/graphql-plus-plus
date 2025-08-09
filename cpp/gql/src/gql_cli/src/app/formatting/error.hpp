#pragma once

#include <string>

#include "libgql/parsers/file/shared/parser_error.hpp"

namespace cli::formatting {
std::string formatError(const gql::parsers::file::shared::ParserError &exc);
};
