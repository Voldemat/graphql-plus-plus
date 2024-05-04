#include "./lexer.hpp"

#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <iostream>
#include <sstream>
#include <utility>

#include "gql_cli/json/serializer.hpp"
#include "gql_cli/utils.hpp"
#include "libgql/lexer/lexer.hpp"

void createLexerSubcommand(CLI::App *app) {
    CLI::App *lexerCmd = app->add_subcommand("lexer", "Lexer");
    CLI::App *lexerParseCmd = lexerCmd->add_subcommand(
        "parse", "Parse input stream into tokens in json format");
    lexerParseCmd->callback([]() {
        const auto buffer = getAllStdin();
        std::istringstream stream(buffer);
        lexer::VectorTokensAccumulator accum;
        lexer::Lexer lexer(std::move(stream), &accum);
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
