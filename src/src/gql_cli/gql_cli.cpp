#include "./gql_cli.hpp"

#include <rapidjson/document.h>
#include <rapidjson/istreamwrapper.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <CLI/Option.hpp>
#include <CLI/Validators.hpp>
#include <cstdio>
#include <exception>
#include <filesystem>
#include <iostream>
#include <memory>
#include <sstream>
#include <string>
#include <utility>

#include "./json/parser.hpp"
#include "./json/serializer.hpp"
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/server/parser.hpp"

std::string getAllStdin() noexcept {
    std::string lineInput;
    std::string buffer;
    while (std::getline(std::cin, lineInput)) {
        buffer += lineInput;
        buffer += '\n';
    };
    return buffer;
};
std::unique_ptr<CLI::App> createCLIApp() noexcept {
    std::unique_ptr<CLI::App> app = std::make_unique<CLI::App>("Graphql++ cli");
    app->require_subcommand(1);
    CLI::App *lexerCmd = app->add_subcommand("lexer", "Lexer");
    CLI::App *lexerParseCmd = lexerCmd->add_subcommand(
        "parse", "Parse input stream into tokens in json format");
    lexerParseCmd->callback([]() {
        const auto buffer = getAllStdin();
        std::istringstream stream(buffer);
        std::shared_ptr<SourceFile> sourceFile
            = std::make_shared<SourceFile>(std::filesystem::path("in-memory"));
        lexer::VectorTokensAccumulator accum;
        lexer::Lexer lexer(std::move(stream), sourceFile, accum);
        auto result = lexer.parse();
        if (result.has_value()) {
            throw result.value();
        };
        const auto tokens = accum.getTokens();
        rapidjson::StringBuffer sb;
        rapidjson::PrettyWriter<rapidjson::StringBuffer> writer(sb);
        writer.StartArray();
        for (const auto &token : tokens) {
            json::serializer::writeTokenAsJSON(writer, token);
        };
        writer.EndArray();
        puts(sb.GetString());
    });
    CLI::App *parserCmd = app->add_subcommand("parser", "Parser");
    CLI::App *parserParseCmd = parserCmd->add_subcommand(
        "parse", "Parse input stream of json serialized tokens into ast tree");
    std::shared_ptr<std::string> sourceFilename
        = std::make_shared<std::string>("check");
    parserParseCmd
        ->add_option("--source-filename", *sourceFilename,
                     "Virtual source filename used in error reports")
        ->required();
    parserParseCmd->callback([sourceFilename]() {
        std::shared_ptr<SourceFile> sourceFile
            = std::make_shared<SourceFile>(*sourceFilename);
        const auto buffer = getAllStdin();
        rapidjson::Document d;
        d.Parse(buffer.c_str());
        auto result = json::parser::parseTokensArray(d, sourceFile);
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
        auto parser = parsers::server::Parser(tokens);
        parsers::server::ast::ASTProgram ast;
        try {
            ast = parser.getAstTree();
        } catch (const std::exception &exc) {
            std::cerr << "Parsing error: " << exc.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
        rapidjson::StringBuffer sb;
        rapidjson::PrettyWriter<rapidjson::StringBuffer> writer(sb);
        json::serializer::ASTJSONWriter astWriter(writer);
        astWriter.writeProgram(ast);
        puts(sb.GetString());
    });
    return app;
};
