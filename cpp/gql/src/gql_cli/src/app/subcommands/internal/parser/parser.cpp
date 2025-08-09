#include "./parser.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

#include "../../../utils.hpp"
#include "app/formatting/error.hpp"
#include "libgql/json/serializers/parser/parser.hpp"
#include "libgql/json/serializers/schema/schema.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/schema.hpp"

using namespace gql::parsers::file;

namespace cli::commands::internal::parser {
void createSubcommand(CLI::App *app) {
    CLI::App *parserCmd = app->add_subcommand("parser", "Parser");
    CLI::App *parserParseCmd =
        parserCmd->add_subcommand("parse", "Parse ast tree");
    std::shared_ptr<std::string> sourceFilename =
        std::make_shared<std::string>();
    parserParseCmd
        ->add_option("--source-filename", *sourceFilename, "Source filename")
        ->required();
    parserParseCmd->callback([sourceFilename]() {
        const auto &buffer = utils::readFromFileOrStdin(*sourceFilename);
        const auto &tokens = utils::parseTokensFromJSON(buffer);
        const auto &source = std::make_shared<shared::ast::SourceFile>();
        const auto &astNodes = utils::parseServerNodesFromJSON(source, tokens);
        const auto &jsonString =
            utils::serializeToJSONString([&astNodes](auto &writer) {
                gql::json::serializers::parser::writeServerNodes(writer,
                                                                 astNodes);
            });
        std::cout << jsonString << std::endl;
    });

    CLI::App *parseDirectoryCmd = app->add_subcommand(
        "parse-dir", "Parse directory with server and client definitions");
    std::shared_ptr<std::string> serverDir = std::make_shared<std::string>();
    std::shared_ptr<std::string> clientDir = std::make_shared<std::string>();
    std::shared_ptr<std::string> outputServerFile =
        std::make_shared<std::string>();
    std::shared_ptr<std::string> outputClientFile =
        std::make_shared<std::string>();
    std::shared_ptr<bool> validate = std::make_shared<bool>();
    parseDirectoryCmd
        ->add_option("--server-dir", *serverDir, "Server directory")
        ->required();
    parseDirectoryCmd->add_option("--client-dir", *clientDir,
                                  "Server directory");
    parseDirectoryCmd
        ->add_option("--output-server-file", *outputServerFile,
                     "Output server file")
        ->required();
    parseDirectoryCmd
        ->add_option("--output-client-file", *outputClientFile,
                     "Output client file")
        ->required();
    parseDirectoryCmd->add_option("--validate", *validate, "Validate")
        ->required();
    parseDirectoryCmd->callback([serverDir, clientDir, validate,
                                 outputServerFile, outputClientFile]() {
        utils::ensureDirectoryExists(
            std::filesystem::path(*outputServerFile).parent_path());
        utils::ensureDirectoryExists(
            std::filesystem::path(*outputClientFile).parent_path());
        if (*validate) {
            utils::ensureFileExists(*outputServerFile);
            utils::ensureFileExists(*outputClientFile);
        };
        utils::ensureDirectoryExists(*serverDir);
        if (*clientDir != "") {
            utils::ensureDirectoryExists(*clientDir);
        }
        std::vector<server::ast::ASTNode> serverNodes =
            utils::parseNodesFromDirectory<server::ast::ASTNode>(
                *serverDir, utils::parseServerNodesFromGraphql);
        std::vector<client::ast::ASTNode> clientNodes;
        if (*clientDir != "") {
            clientNodes = utils::parseNodesFromDirectory<client::ast::ASTNode>(
                *clientDir, utils::parseClientNodesFromGraphql);
        }
        try {
            const auto &schema =
                gql::parsers::schema::parseSchema(serverNodes, clientNodes);
            const auto &jsonServerString =
                utils::serializeToJSONString([&schema](auto &writer) {
                    gql::json::serializers::schema::writeServerSchema(
                        writer, schema.server);
                });
            const auto &jsonClientString =
                utils::serializeToJSONString([&schema](auto &writer) {
                    gql::json::serializers::schema::writeClientSchema(
                        writer, schema.client);
                });
            if (*validate) {
                const auto &hasServerChanges = utils::doesFileHaveChanges(
                    *outputServerFile, jsonServerString, "Server");
                if (!hasServerChanges) return;
                const auto &hasClientChanges = utils::doesFileHaveChanges(
                    *outputClientFile, jsonClientString, "Client");
                if (!hasClientChanges) return;
                throw CLI::RuntimeError(1);
            };
            std::ofstream serverFile(*outputServerFile, std::ios::trunc);
            serverFile << jsonServerString;
            std::ofstream clientFile(*outputClientFile, std::ios::trunc);
            clientFile << jsonClientString;
        } catch (const shared::ParserError &error) {
            std::cerr << cli::formatting::formatError(error) << std::endl;
            throw CLI::RuntimeError(1);
        } catch (const std::exception &exc) {
            std::cerr << exc.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
    });
};
};  // namespace cli::commands::internal::parser
