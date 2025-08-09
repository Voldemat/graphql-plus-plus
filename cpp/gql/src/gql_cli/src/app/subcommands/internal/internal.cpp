#include "./internal.hpp"

#include <CLI/App.hpp>

#include "./diff/diff.hpp"
#include "./lexer/lexer.hpp"
#include "./parser/parser.hpp"

namespace cli::commands::internal {
void createSubcommand(CLI::App *app) {
    CLI::App *internal =
        app->add_subcommand("internal", "Internal commands for debugging");
    lexer::createSubcommand(internal);
    parser::createSubcommand(internal);
    diff::createSubcommand(internal);
};
};  // namespace cli::commands::internal
