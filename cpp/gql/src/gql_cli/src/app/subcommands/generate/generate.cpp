#include "./generate.hpp"

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <string>

#include "app/config_action.hpp"

namespace cli::commands::generate {

void createSubcommand(CLI::App *app) {
    auto command =
        app->add_subcommand("generate", "Generate outputs based on config");
    auto configPath = std::make_shared<std::string>("./gql.yaml");
    command->add_option("--config", *configPath, "Path to config");
    command->callback([configPath]() {
        run_config_action(
            *configPath,
            [](const std::string &jsonString,
               const std::filesystem::path &configDirPath,
               const std::string &outputPath, const std::string &) {
                if (outputPath == "-") {
                    std::cout << jsonString << std::endl;
                } else {
                    const auto &filepath = configDirPath / outputPath;
                    std::ofstream file(outputPath, std::ios::trunc);
                    file << jsonString;
                };
            });
    });
};
};  // namespace cli::commands::generate
