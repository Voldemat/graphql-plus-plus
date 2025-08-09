#include "./config.hpp"

#include <expected>
#include <format>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

#include "yaml-cpp/yaml.h"

namespace cli::config {
namespace yaml {
std::expected<std::vector<std::string>, std::string> parsePathsArray(
    const std::string &rootPath, const std::string &currentPath,
    const YAML::Node &node) {
    if (!node.IsDefined())
        return std::unexpected(std::format("\"{}\" should have \"{}\" key",
                                           rootPath, currentPath));
    if (!node.IsSequence())
        return std::unexpected(std::format("\"{}.{}\" value should be a map",
                                           rootPath, currentPath));
    std::vector<std::string> paths;
    for (int i = 0; i < node.size(); i++) {
        const auto &element = node[i];
        if (!element.IsScalar()) {
            return std::unexpected(
                std::format("\"{}.{}[{}]\" value should be string", rootPath,
                            currentPath, i));
        }
        paths.emplace_back(element.as<std::string>());
    }
    return paths;
};

std::expected<InputsConfig, std::string> parseInputsConfig(
    const std::string &rootPath, const YAML::Node &node) {
    if (!node.IsDefined())
        return std::unexpected(
            std::format("\"{}\" should have \"inputs\" key", rootPath));
    if (!node.IsMap())
        return std::unexpected(
            std::format("\"{}.inputs\" value should be a map", rootPath));
    const auto &graphqlResult = parsePathsArray(
        std::format("{}.inputs", rootPath), "graphql", node["graphql"]);
    if (!graphqlResult.has_value())
        return std::unexpected(graphqlResult.error());
    const auto &jsonSchemaResult = parsePathsArray(
        std::format("{}.inputs", rootPath), "jsonSchema", node["jsonSchema"]);
    if (!jsonSchemaResult.has_value())
        return std::unexpected(jsonSchemaResult.error());
    return (InputsConfig){ .graphql = graphqlResult.value(),
                           .jsonSchema = jsonSchemaResult.value() };
};

std::expected<std::optional<OutputsConfig>, std::string> parseOutputsConfig(
    const std::string &rootPath, const YAML::Node &node) {
    if (!node.IsDefined()) return std::nullopt;
    if (!node.IsMap()) {
        return std::unexpected(std::format(
            "\"{}.outputs\" value if present should be a map", rootPath));
    };
    const auto &filepath = node["filepath"];
    if (!filepath.IsDefined()) {
        return std::unexpected(std::format(
            "\"{}.outputs\" value have \"filepath\" key", rootPath));
    };
    if (!filepath.IsScalar()) {
        return std::unexpected(std::format(
            "\"{}.outputs.filepath\" value should be a string", rootPath));
    };
    return (OutputsConfig){ .filepath = filepath.as<std::string>() };
};

std::expected<ServerConfig, std::string> parseServerConfig(
    const YAML::Node &node) {
    if (!node.IsDefined()) {
        return std::unexpected("Root should have \"server\" key");
    };
    if (!node.IsMap()) {
        return std::unexpected("\"server\" value should be a map");
    };
    const auto &inputsResult = parseInputsConfig("server", node["inputs"]);
    if (!inputsResult.has_value()) return std::unexpected(inputsResult.error());
    const auto &outputsResult = parseOutputsConfig("server", node["outputs"]);
    if (!outputsResult.has_value())
        return std::unexpected(outputsResult.error());
    return (ServerConfig){ .inputs = inputsResult.value(),
                           .outputs = outputsResult.value() };
};

std::expected<std::optional<ClientConfig>, std::string> parseClientConfig(
    const YAML::Node &node) {
    if (!node.IsDefined()) {
        return std::nullopt;
    };
    if (!node.IsMap()) {
        return std::unexpected("\"client\" value should be a map");
    };
    const auto &inputsResult = parseInputsConfig("client", node["inputs"]);
    if (!inputsResult.has_value()) return std::unexpected(inputsResult.error());
    const auto &outputsResult = parseOutputsConfig("client", node["outputs"]);
    if (!outputsResult.has_value())
        return std::unexpected(outputsResult.error());
    return (ClientConfig){ .inputs = inputsResult.value(),
                           .outputs = outputsResult.value() };
};

};  // namespace yaml

std::expected<Config, std::string> Config::fromYaml(const YAML::Node &yaml) {
    if (!yaml.IsMap() || !yaml.IsDefined()) {
        return std::unexpected("Root should be an object");
    };
    const auto &sResult = yaml::parseServerConfig(yaml["server"]);
    if (!sResult.has_value()) return std::unexpected(sResult.error());
    const auto &cResult = yaml::parseClientConfig(yaml["client"]);
    if (!cResult.has_value()) return std::unexpected(cResult.error());
    return (Config){ .server = sResult.value(), .client = cResult.value() };
};
};  // namespace cli::config
