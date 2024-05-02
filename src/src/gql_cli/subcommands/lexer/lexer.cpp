#include "./lexer.hpp"

#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <filesystem>
#include <iostream>
#include <memory>
#include <sstream>
#include <utility>

#include "gql_cli/json/serializer.hpp"
#include "gql_cli/utils.hpp"
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/token.hpp"

void createLexerSubcommand(CLI::App *app) {
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
        std::cout << sb.GetString() << std::endl;
    });
};
