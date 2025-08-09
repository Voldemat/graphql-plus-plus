#pragma once

#include <expected>
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
};

struct ServerConfig {
    InputsConfig inputs;
    std::optional<OutputsConfig> outputs;
};

struct ClientConfig {
    InputsConfig inputs;
    std::optional<OutputsConfig> outputs;
};

struct Config {
    ServerConfig server;
    std::optional<ClientConfig> client;
    std::expected<Config, std::string> static fromYaml(const YAML::Node &yaml);
};
};  // namespace cli::config
