#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <cstdlib>
#include <memory>

#include "gql_cli/gql_cli.hpp"

int main(int argc, char **argv) {
    std::unique_ptr<CLI::App> app = createCLIApp();
    argv = app->ensure_utf8(argv);
    CLI11_PARSE(*app, argc, argv);
    return EXIT_SUCCESS;
}
