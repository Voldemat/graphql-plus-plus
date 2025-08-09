#include "./gql_cli.hpp"

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <CLI/Option.hpp>
#include <CLI/Validators.hpp>
#include <memory>
#include "app/subcommands/generate/generate.hpp"
#include "app/subcommands/internal/internal.hpp"
#include "app/subcommands/validate/validate.hpp"

namespace cli {
std::unique_ptr<CLI::App> createCLIApp() noexcept {
    std::unique_ptr<CLI::App> app = std::make_unique<CLI::App>("Graphql++ cli");
    app->require_subcommand(1);
    commands::internal::createSubcommand(app.get());
    commands::generate::createSubcommand(app.get());
    commands::validate::createSubcommand(app.get());
    return app;
};
};
