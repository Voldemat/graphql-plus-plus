#include "./parser.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <algorithm>
#include <exception>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <iterator>
#include <limits>
#include <memory>
#include <ranges>
#include <sstream>
#include <string>
#include <vector>

#include "gql_cli/utils.hpp"
#include "libgql/json/parsers/lexer/lexer.hpp"
#include "libgql/json/parsers/shared.hpp"
#include "libgql/json/serializers/parser/parser.hpp"
#include "libgql/json/serializers/schema/schema.hpp"
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/lexer_error.hpp"
#include "libgql/lexer/location.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/tokens_accumulators.hpp"
#include "libgql/parsers/client/ast.hpp"
#include "libgql/parsers/client/parser.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/server/parser.hpp"
#include "libgql/parsers/shared/shared.hpp"

using namespace parsers;

rapidjson::Writer<rapidjson::StringBuffer> createWriter(
    const bool &pretty, rapidjson::StringBuffer &buffer) {
    if (!pretty) return rapidjson::Writer(buffer);
    return rapidjson::PrettyWriter(buffer);
};

std::string formatLine(const std::string &line, const unsigned int &currentLine,
                       const lexer::Location &location,
                       const shared::ParserError &exc) {
    std::string linestr = std::to_string(currentLine);
    std::string buffer = std::format("{}: {}\n", linestr, line);
    if (currentLine == location.line) {
        std::string underline;
        for (unsigned int i = 0; i < location.start + 2 + linestr.size(); i++) {
            underline += " ";
        };
        for (unsigned int i = location.start; i < location.end + 1; i++) {
            underline += "~";
        };
        underline += std::format(" Error: {}\n", exc.what());
        buffer += underline;
    };
    return buffer;
};

std::string formatError(const shared::ParserError &exc) {
    std::string buffer =
        std::format("{}\n", exc.getSource()->filepath.string());
    const lexer::Location &location = exc.getLocation();
    unsigned int firstLineToShow =
        std::clamp((int)location.line - 4, 1, std::numeric_limits<int>::max());
    unsigned int lastLineToShow = location.line + 4;
    std::string line;
    unsigned int currentLine = 1;
    std::istringstream stream = (std::istringstream)exc.getSource()->buffer;
    while (std::getline(stream, line)) {
        if (firstLineToShow <= currentLine && currentLine <= lastLineToShow) {
            buffer += formatLine(line, currentLine, location, exc);
        };
        currentLine += 1;
    };
    return buffer;
};

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
        std::vector<lexer::GQLToken> tokens;
        try {
            tokens = json::parsers::lexer::parseTokensArray(d.GetArray());
        } catch (const json::parsers::shared::ParsingError &error) {
            std::cerr << error.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (tokens.size() == 0) {
            std::cerr << "No tokens was provided in array" << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::shared_ptr<shared::ast::SourceFile> source =
            std::make_shared<shared::ast::SourceFile>();
        auto parser = server::Parser(tokens, source);
        server::ast::FileNodes ast;
        try {
            ast = parser.parse();
        } catch (const std::exception &exc) {
            std::cerr << "Parsing error: " << exc.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
        rapidjson::StringBuffer sb;
        rapidjson::PrettyWriter<rapidjson::StringBuffer> writer(sb);
        json::serializers::parser::writeFileNodes(writer, ast);
        std::cout << sb.GetString() << std::endl;
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
        if (!std::filesystem::exists(*serverDir)) {
            std::cerr << std::format("Directory \"{}\" does not exists",
                                     *serverDir)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (!std::filesystem::is_directory(
                std::filesystem::status(*serverDir))) {
            std::cerr << std::format("Path {} is not directory", *serverDir)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (!std::filesystem::exists(*clientDir)) {
            std::cerr << std::format("Directory \"{}\" does not exists",
                                     *clientDir)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (!std::filesystem::is_directory(
                std::filesystem::status(*clientDir))) {
            std::cerr << std::format("Path {} is not directory", *clientDir)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::vector<server::ast::FileNodes> astList;
        for (const std::filesystem::path &filepath :
             std::filesystem::recursive_directory_iterator(*serverDir) |
                 std::views::filter(
                     [](const std::filesystem::directory_entry &entry) {
                         return entry.path().extension() == ".graphql";
                     }) |
                 std::views::transform(
                     [](const std::filesystem::directory_entry &entry) {
                         return entry.path();
                     })) {
            std::ifstream file(filepath);
            std::stringstream fileStream;
            fileStream << file.rdbuf();
            std::string buffer = fileStream.str();
            std::shared_ptr<shared::ast::SourceFile> source =
                std::make_shared<shared::ast::SourceFile>(filepath, buffer);
            lexer::VectorTokensAccumulator tokensAccumulator;
            lexer::Lexer lexer(buffer, &tokensAccumulator);
            try {
                lexer.parse();
            } catch (const lexer::LexerError &error) {
                std::cerr << "LexerParserError: " << error.what() << std::endl;
                throw CLI::RuntimeError(1);
            };
            const auto &tokens = tokensAccumulator.getTokens();
            server::Parser parser(tokens, source);
            try {
                const auto &ast = parser.parse();
                astList.push_back(ast);
            } catch (const shared::ParserError &exc) {
                std::cerr << formatError(exc) << std::endl;
                throw CLI::RuntimeError(1);
            };
        };
        std::vector<client::ast::ClientDefinition> operations;
        for (const auto &filepath :
             std::filesystem::recursive_directory_iterator(*clientDir) |
                 std::views::filter(
                     [](const std::filesystem::directory_entry &entry) {
                         return entry.path().extension() == ".graphql";
                     }) |
                 std::views::transform(
                     [](const std::filesystem::directory_entry &entry) {
                         return entry.path();
                     })) {
            std::ifstream file(filepath);
            std::stringstream fileStream;
            fileStream << file.rdbuf();
            std::string buffer = fileStream.str();
            std::shared_ptr<shared::ast::SourceFile> source =
                std::make_shared<shared::ast::SourceFile>(filepath, buffer);
            lexer::VectorTokensAccumulator tokensAccumulator;
            lexer::Lexer lexer(buffer, &tokensAccumulator);
            try {
                lexer.parse();
            } catch (const lexer::LexerError &error) {
                std::cerr << std::format("LexerParserError({}): {}",
                                         source->filepath.filename().string(),
                                         error.what())
                          << std::endl;
                throw CLI::RuntimeError(1);
            };
            const auto &tokens = tokensAccumulator.getTokens();
            if (tokens.empty()) continue;
            client::Parser parser(tokens, source);
            try {
                for (const auto &el : parser.parse()) {
                    operations.push_back(el);
                };
            } catch (const shared::ParserError &exc) {
                std::cerr << formatError(exc) << std::endl;
                throw CLI::RuntimeError(1);
            };
        };
        try {
            const auto &schema =
                parsers::schema::parseSchema(astList, operations);
            rapidjson::StringBuffer buffer;
            rapidjson::Writer writer(buffer);
            json::serializers::schema::writeSchemaNodes(writer, schema);
            std::cout << buffer.GetString() << std::endl;
        } catch (const shared::ParserError &error) {
            std::cerr << formatError(error) << std::endl;
            throw CLI::RuntimeError(1);
        } catch (const std::exception &exc) {
            std::cerr << exc.what() << std::endl;
            throw CLI::RuntimeError(1);
        };
    });
};
