#include "./gql_cli.hpp"

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <CLI/Option.hpp>
#include <CLI/Validators.hpp>
#include <memory>

#include "./subcommands/parser/parser.hpp"
#include "./subcommands/lexer/lexer.hpp"
#include "./subcommands/diff/diff.hpp"

std::unique_ptr<CLI::App> createCLIApp() noexcept {
    std::unique_ptr<CLI::App> app = std::make_unique<CLI::App>("Graphql++ cli");
    app->require_subcommand(1);
    CLI::App* internal = app->add_subcommand("internal", "Internal commands for debugging");
    createLexerSubcommand(internal);
    createParserSubcommand(internal);
    createDifSubcommand(internal);
    return app;
};
