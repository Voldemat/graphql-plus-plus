#ifndef GQL_CLI
#define GQL_CLI

#include <CLI/App.hpp>
#include <memory>

std::unique_ptr<CLI::App> createCLIApp() noexcept;

#endif
