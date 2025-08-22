#include "./config_action.hpp"

#include <CLI/Error.hpp>
#include <filesystem>
#include <functional>
#include <iostream>
#include <optional>
#include <string>

#include "app/config/config.hpp"
#include "app/utils.hpp"
#include "libgql/json/serializers/schema/schema.hpp"
#include "libgql/parsers/schema/operations_map.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "yaml-cpp/yaml.h"

namespace cli {
void run_config_action(
    const std::string &configPath,
    const std::function<void(const std::string &, const std::filesystem::path &,
                             const std::string &, const std::string &)>
        &jsonStringCallback) {
    const auto &buffer = utils::readFile(configPath);
    const auto &yaml = YAML::LoadFile(configPath);
    const auto &configDirPath = std::filesystem::path(configPath).parent_path();
    const auto &parseResult = config::Config::fromYaml(yaml);
    if (!parseResult.has_value()) {
        std::cerr << parseResult.error() << std::endl;
        throw CLI::RuntimeError(1);
    };
    const auto &config = parseResult.value();
    gql::parsers::schema::TypeRegistry registry;
    const auto &serverResult = utils::loadServerSchemaFromInputs(
        registry, config.server.inputs, configDirPath);
    if (!serverResult.has_value()) {
        for (const auto &error : serverResult.error()) {
            std::cerr << error << std::endl;
        };
        throw CLI::RuntimeError(1);
    };
    const auto &serverSchema = serverResult.value();
    auto serverOnlyRegistry = registry;

    std::optional<gql::parsers::schema::ClientSchema> clientSchema;

    if (config.client.has_value()) {
        const auto &result = utils::loadClientSchemaFromInputs(
            registry, config.client->inputs, configDirPath);
        if (!result.has_value()) {
            for (const auto &error : result.error()) {
                std::cerr << error << std::endl;
            };
            throw CLI::RuntimeError(1);
        };
        clientSchema = result.value();
    };

    std::optional<gql::parsers::schema::operations_map::OperationsMapContainer>
        operationsMapContainer;

    if (config.operationsMap.has_value()) {
        const auto &result = utils::loadOperationsMapContainerFromInputs(
            serverOnlyRegistry, config.operationsMap->inputs, configDirPath);
        if (!result.has_value()) {
            for (const auto &error : result.error()) {
                std::cerr << error << std::endl;
            };
            throw CLI::RuntimeError(1);
        };
        operationsMapContainer = result.value();
    };

    if (config.server.outputs.has_value()) {
        const auto &onlyUsedInOperations =
            config.server.outputs->onlyUsedInOperations;
        const auto &jsonString =
            utils::serializeToJSONString([&serverSchema, &clientSchema,
                                          &onlyUsedInOperations](auto &writer) {
                gql::json::serializers::schema::writeServerSchema(
                    writer, serverSchema,
                    onlyUsedInOperations && clientSchema.has_value()
                        ? (std::optional<gql::parsers::schema::ClientSchema>)
                              clientSchema.value()
                        : std::nullopt);
            });
        jsonStringCallback(jsonString, configDirPath,
                           config.server.outputs->filepath, "Server");
    };

    if (config.client.has_value() && config.client->outputs.has_value()) {
        const auto &jsonString =
            utils::serializeToJSONString([&clientSchema](auto &writer) {
                gql::json::serializers::schema::writeClientSchema(
                    writer, clientSchema.value());
            });
        jsonStringCallback(jsonString, configDirPath,
                           config.client->outputs->filepath, "Client");
    };

    if (config.operationsMap.has_value()) {
        const auto &jsonString = utils::serializeToJSONString(
            [&operationsMapContainer](auto &writer) {
                gql::json::serializers::schema::writeOperationsMapContainer(
                    writer, operationsMapContainer.value());
            });
        jsonStringCallback(jsonString, configDirPath,
                           config.operationsMap->outputPath, "Operations map");
    };
};
};  // namespace cli
