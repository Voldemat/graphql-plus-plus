#include "./parser.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <exception>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <iterator>
#include <memory>
#include <string>
#include <vector>

#include "gql_cli/json/parser.hpp"
#include "gql_cli/json/serializer.hpp"
#include "gql_cli/utils.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/server/parser.hpp"

using namespace parsers::server;

void createParserSubcommand(CLI::App *app) {
    CLI::App *parserCmd = app->add_subcommand("parser", "Parser");
    CLI::App *parserParseCmd
        = parserCmd->add_subcommand("parse", "Parse ast tree");
    std::shared_ptr<std::string> sourceFilename
        = std::make_shared<std::string>();
    parserParseCmd
        ->add_option("--source-filename", *sourceFilename, "Source filename")
        ->required();
    parserParseCmd->callback([sourceFilename]() {
        std::string buffer;
        if (*sourceFilename == "-") {
            *sourceFilename = "in-memory";
            buffer = getAllStdin();
        } else {
            if (!std::filesystem::exists(*sourceFilename)) {
                std::cerr << std::format("File {} does not exists",
                                         *sourceFilename)
                          << std::endl;
                throw CLI::RuntimeError(1);
            };
            std::ifstream file(*sourceFilename);
            buffer = std::string((std::istreambuf_iterator<char>(file)),
                                 std::istreambuf_iterator<char>());
        };
        rapidjson::Document d;
        d.Parse(buffer.c_str());
        auto result = json::parser::parseTokensArray(d);
        if (!result.has_value()) {
            std::string error = result.error();
            std::cerr << error << std::endl;
            throw CLI::RuntimeError(1);
        };
        auto tokens = result.value();
        if (tokens.size() == 0) {
            std::cerr << "Warning: No tokens was provided in array"
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::shared_ptr<ast::SourceFile> source = std::make_shared<ast::SourceFile>();
        auto parser = Parser(tokens, source);
        ast::FileNodes ast;
        try {
            ast = parser.parse();
        } catch (const std::exception &exc) {
            std::cerr << "Parsing error: " << exc.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
        rapidjson::StringBuffer sb;
        rapidjson::PrettyWriter<rapidjson::StringBuffer> writer(sb);
        json::serializer::ASTJSONWriter astWriter(&writer);
        std::cout << sb.GetString() << std::endl;
    });
};
