#pragma once

#include <expected>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "yaml-cpp/yaml.h"

namespace cli::config {

struct InputsConfig {
    std::vector<std::string> graphql;
    std::vector<std::string> jsonSchema;
};

struct OutputsConfig {
    std::string filepath;
    bool onlyUsedInOperations;
};

struct ServerConfig {
    InputsConfig inputs;
    std::optional<OutputsConfig> outputs;
};

struct ClientConfig {
    InputsConfig inputs;
    std::optional<OutputsConfig> outputs;
};

using OperationsMapInputsConfig =
    std::map<std::string, std::vector<std::string>>;

struct OperationsMapConfig {
    OperationsMapInputsConfig inputs;
    std::string outputPath;
};

struct Config {
    ServerConfig server;
    std::optional<ClientConfig> client;
    std::optional<OperationsMapConfig> operationsMap;
    std::expected<Config, std::string> static fromYaml(const YAML::Node &yaml);
};
};  // namespace cli::config
