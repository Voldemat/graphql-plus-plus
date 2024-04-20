#include "./gql_cli.hpp"

#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <cstdio>
#include <filesystem>
#include <iostream>
#include <memory>
#include <sstream>
#include <string>
#include <utility>

#include "json/serializer.hpp"
#include "lexer/lexer.hpp"
#include "lexer/token.hpp"

std::unique_ptr<CLI::App> createCLIApp() noexcept {
    std::unique_ptr<CLI::App> app = std::make_unique<CLI::App>("Graphql++ cli");
    app->require_subcommand(1);
    CLI::App *lexerCmd = app->add_subcommand("lexer", "Lexer");
    CLI::App *lexerParseCmd = lexerCmd->add_subcommand(
        "parse", "Parse input stream into tokens in json format");
    lexerParseCmd->callback([]() {
        std::string lineInput;
        std::string buffer;
        while (std::getline(std::cin, lineInput)) {
            buffer += lineInput;
            buffer += '\n';
        };
        std::istringstream stream(buffer);
        std::shared_ptr<SourceFile> sourceFile
            = std::make_shared<SourceFile>(std::filesystem::path("in-memory"));
        lexer::Lexer lexer(std::move(stream), sourceFile);
        auto tokens = lexer.getTokens();
        rapidjson::StringBuffer sb;
        rapidjson::PrettyWriter<rapidjson::StringBuffer> writer(sb);
        writer.StartArray();
        for (const auto &token : tokens) {
            json::serializer::writeTokenAsJSON(writer, token);
        };
        writer.EndArray();
        puts(sb.GetString());
    });
    return app;
};
