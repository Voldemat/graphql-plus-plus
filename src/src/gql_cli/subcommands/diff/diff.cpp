#include "./diff.hpp"

#include <rapidjson/document.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <algorithm>
#include <ranges>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <vector>

#include "HTTPRequest.hpp"
#include "gql_cli/json/introspection/parser.hpp"
#include "gql_cli/json/parser.hpp"
#include "gql_cli/utils.hpp"
#include "libgql/parsers/schema/schema.hpp"

const char *INTROSPECTION_QUERY =
#include "./query.data"
    rapidjson::Document getIntrospectionDocument(const std::string &urlToApi) {
    http::HeaderFields headers{ { "Accept", "application/json" },
                                { "Content-Type", "application/json" } };
    http::Request request{ urlToApi };
    const auto &response = request.send("POST", INTROSPECTION_QUERY, headers);
    if (response.status.code != http::Status::Ok) {
        std::cerr << std::format("Expected 200 status code, while received {}",
                                 response.status.code)
                  << std::endl;
        throw CLI::RuntimeError(1);
    };
    std::string buffer{ response.body.begin(), response.body.end() };
    rapidjson::Document d;
    d.Parse(buffer.c_str());
    return d;
};

rapidjson::Document getDocumentFromSchemaJson(const std::string &pathToSchema) {
    std::string buffer;
    if (pathToSchema == "-") {
        buffer = getAllStdin();
    } else {
        if (!std::filesystem::exists(pathToSchema)) {
            std::cerr << std::format("Path \"{}\" does not exists",
                                     pathToSchema)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (!std::filesystem::is_regular_file(
                std::filesystem::status(pathToSchema))) {
            std::cerr << std::format("Path {} is not regular file",
                                     pathToSchema)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::ifstream file(pathToSchema);
        std::stringstream fileStream;
        fileStream << file.rdbuf();
        buffer = fileStream.str();
    };
    rapidjson::Document d;
    d.Parse(buffer.c_str());
    return d;
};

void createDifSubcommand(CLI::App *app) {
    CLI::App *diffCmd = app->add_subcommand("diff", "Diff between two schemas");
    CLI::App *diffParseCmd = diffCmd->add_subcommand(
        "parse", "Parse input stream into tokens in json format");

    std::shared_ptr<std::string> pathToSchema = std::make_shared<std::string>();
    std::shared_ptr<std::string> urlToApi = std::make_shared<std::string>();
    diffParseCmd
        ->add_option("--path-to-schema", *pathToSchema,
                     "Path to schema json file")
        ->required();
    diffParseCmd->add_option("--url-to-api", *urlToApi, "Url to api")
        ->required();
    diffParseCmd->callback([pathToSchema, urlToApi]() {
        const auto &schemaDocument = getDocumentFromSchemaJson(*pathToSchema);
        const auto &schema = json::parser::parseSchema(schemaDocument);
        const auto &introspectionDocument = getIntrospectionDocument(*urlToApi);
        const auto &secondSchema =
            json::parser::introspection::parseIntrospectionSchema(
                introspectionDocument);
        const auto& schemaObjectsNames = schema.server.objects | std::views::keys | std::ranges::to<std::vector>();
        const auto& serverObjectsNames = secondSchema.objects | std::views::keys | std::ranges::to<std::vector>();
        std::vector<std::string>
            serverOnlyObjects;
        std::set_difference(
            serverObjectsNames.begin(), serverObjectsNames.end(),
            schemaObjectsNames.begin(), schemaObjectsNames.end(),
            std::inserter(serverOnlyObjects, serverOnlyObjects.begin()));
        for (const auto& name : serverOnlyObjects) {
            std::cout << name << std::endl;
        };
        //std::cout << std::format("Number of objects: {} - {}",
        //                         schema.server.objects.size(),
        //                         secondSchema.objects.size())
        //          << std::endl;
        if (schema.server != secondSchema) {
            std::cerr << "Schemas are different" << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::cout << "Schemas are identical" << std::endl;
    });
};
