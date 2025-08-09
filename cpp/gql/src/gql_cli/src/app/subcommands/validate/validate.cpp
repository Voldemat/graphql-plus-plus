#include "./validate.hpp"

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <filesystem>
#include <memory>
#include <string>

#include "app/config_action.hpp"
#include "app/utils.hpp"

namespace cli::commands::validate {

void createSubcommand(CLI::App *app) {
    auto command = app->add_subcommand("validate", "Validate");
    auto configPath = std::make_shared<std::string>("./gql.yaml");
    command->add_option("--config", *configPath, "Path to config");
    command->callback([configPath]() {
        run_config_action(
            *configPath,
            [](const std::string &jsonString,
               const std::filesystem::path &configDirPath,
               const std::string &outputPath, const std::string &schemaName) {
                if (outputPath == "-") {
                    return;
                };
                const auto &path = configDirPath / outputPath;
                const auto &hasChanges = utils::doesFileHaveChanges(
                    path.string(), jsonString, schemaName);
                if (!hasChanges) return;
                throw CLI::RuntimeError(1);
            });
    });
};
};  // namespace cli::commands::validate
