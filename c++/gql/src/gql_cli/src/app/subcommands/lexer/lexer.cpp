#include "./lexer.hpp"

#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <format>
#include <iostream>

#include "../../utils.hpp"
#include "libgql/json/serializers/lexer/lexer.hpp"
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/lexer_error.hpp"
#include "libgql/lexer/tokens_accumulators.hpp"

void createLexerSubcommand(CLI::App *app) {
    CLI::App *lexerCmd = app->add_subcommand("lexer", "Lexer");
    CLI::App *lexerParseCmd = lexerCmd->add_subcommand(
        "parse", "Parse input stream into tokens in json format");
    lexerParseCmd->callback([]() {
        const auto buffer = getAllStdin();
        lexer::VectorTokensAccumulator accum;
        lexer::Lexer lexer(buffer, &accum);
        try {
            lexer.parse();
        } catch (const lexer::LexerError &error) {
            std::cerr << std::format("LexerError: {}", error.what()) << std::endl;
            throw CLI::RuntimeError(1);
        };
        const auto tokens = accum.getTokens();
        rapidjson::StringBuffer sb;
        rapidjson::PrettyWriter<rapidjson::StringBuffer> writer(sb);
        json::serializers::lexer::writeTokens(writer, tokens);
        std::cout << sb.GetString() << std::endl;
    });
};
