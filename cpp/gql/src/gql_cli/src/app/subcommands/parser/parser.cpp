#include "./parser.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <exception>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

#include "../../utils.hpp"
#include "app/formatting/error.hpp"
#include "libgql/json/serializers/parser/parser.hpp"
#include "libgql/json/serializers/schema/schema.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "libgql/parsers/schema/schema.hpp"

using namespace parsers::file;

void createParserSubcommand(CLI::App *app) {
    CLI::App *parserCmd = app->add_subcommand("parser", "Parser");
    CLI::App *parserParseCmd =
        parserCmd->add_subcommand("parse", "Parse ast tree");
    std::shared_ptr<std::string> sourceFilename =
        std::make_shared<std::string>();
    parserParseCmd
        ->add_option("--source-filename", *sourceFilename, "Source filename")
        ->required();
    parserParseCmd->callback([sourceFilename]() {
        const auto &buffer = readFromFileOrStdin(*sourceFilename);
        const auto &tokens = parseTokensFromJSON(buffer);
        const auto &source = std::make_shared<shared::ast::SourceFile>();
        const auto &astNodes = parseServerNodesFromJSON(source, tokens);
        const auto &jsonString =
            serializeToJSONString([&astNodes](auto &writer) {
                json::serializers::parser::writeServerNodes(writer, astNodes);
            });
        std::cout << jsonString << std::endl;
    });

    CLI::App *parseDirectoryCmd = app->add_subcommand(
        "parse-dir", "Parse directory with server and client definitions");
    std::shared_ptr<std::string> serverDir = std::make_shared<std::string>();
    std::shared_ptr<std::string> clientDir = std::make_shared<std::string>();
    parseDirectoryCmd
        ->add_option("--server-dir", *serverDir, "Server directory")
        ->required();
    parseDirectoryCmd
        ->add_option("--client-dir", *clientDir, "Server directory")
        ->required();
    parseDirectoryCmd->callback([serverDir, clientDir]() {
        ensureDirectoryExists(*serverDir);
        ensureDirectoryExists(*clientDir);
        std::vector<server::ast::ASTNode> serverNodes =
            parseNodesFromDirectory<server::ast::ASTNode>(
                *serverDir, parseServerNodesFromGraphql);
        std::vector<client::ast::ASTNode> clientNodes =
            parseNodesFromDirectory<client::ast::ASTNode>(
                *clientDir, parseClientNodesFromGraphql);
        try {
            const auto &schema =
                parsers::schema::parseSchema(serverNodes, clientNodes);
            const auto &jsonString =
                serializeToJSONString([&schema](auto &writer) {
                    json::serializers::schema::writeSchema(writer, schema);
                });
            std::cout << jsonString << std::endl;
        } catch (const shared::ParserError &error) {
            std::cerr << formatError(error) << std::endl;
            throw CLI::RuntimeError(1);
        } catch (const std::exception &exc) {
            std::cerr << exc.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
    });
};
