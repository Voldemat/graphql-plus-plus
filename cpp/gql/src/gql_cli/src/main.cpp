#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <CLI/Option.hpp>
#include <memory>

#include <cstdlib>
#include <memory>

#include "app/gql_cli.hpp"

int main(int argc, char **argv) {
    std::unique_ptr<CLI::App> app = cli::createCLIApp();
    argv = app->ensure_utf8(argv);
    CLI11_PARSE(*app, argc, argv);
    return EXIT_SUCCESS;
}
