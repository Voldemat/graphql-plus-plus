#include "./parser.hpp"

#include <rapidjson/document.h>
#include <rapidjson/prettywriter.h>
#include <rapidjson/stringbuffer.h>

#include <CLI/App.hpp>
#include <CLI/Error.hpp>
#include <algorithm>
#include <exception>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <iterator>
#include <memory>
#include <ranges>
#include <sstream>
#include <string>
#include <vector>

#include "gql_cli/json/parser.hpp"
#include "gql_cli/json/serializer.hpp"
#include "gql_cli/utils.hpp"
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/server/ast.hpp"
#include "libgql/parsers/server/parser.hpp"

using namespace parsers::server;

std::string formatLine(const std::string &line, const unsigned int &currentLine,
                       const Location &location, const ParserError& exc){
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

std::string formatError(const ParserError &exc,
                        const std::shared_ptr<ast::SourceFile> &source) {
    std::string buffer;
    const Location &location = exc.getLocation();
    unsigned int firstLineToShow = std::max(location.line - 4, (unsigned int)1);
    unsigned int lastLineToShow = location.line + 4;
    std::string line;
    unsigned int currentLine = 1;
    std::istringstream stream = (std::istringstream)source->buffer;
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
        std::shared_ptr<ast::SourceFile> source =
            std::make_shared<ast::SourceFile>();
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

    CLI::App *parseDirectoryCmd = app->add_subcommand(
        "parse-dir", "Parse directory with server definitions");
    std::shared_ptr<std::string> directory = std::make_shared<std::string>();
    parseDirectoryCmd->add_option("--dir", *directory, "Directory")->required();
    parseDirectoryCmd->callback([directory]() {
        if (!std::filesystem::exists(*directory)) {
            std::cerr << std::format("Directory \"{}\" does not exists",
                                     *directory)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (!std::filesystem::is_directory(
                std::filesystem::status(*directory))) {
            std::cerr << std::format("Path {} is not directory", *directory)
                      << std::endl;
            throw CLI::RuntimeError(1);
        };
        std::vector<ast::FileNodes> astList;
        for (const std::filesystem::path &filepath :
             std::filesystem::recursive_directory_iterator(*directory) |
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
            std::shared_ptr<ast::SourceFile> source =
                std::make_shared<ast::SourceFile>(filepath, buffer);
            lexer::VectorTokensAccumulator tokensAccumulator;
            lexer::Lexer lexer((std::istringstream)buffer, &tokensAccumulator);
            const auto &error = lexer.parse();
            if (error.has_value()) {
                std::cerr << "LexerParserError: " << error.value().what()
                          << std::endl;
                throw CLI::RuntimeError(1);
            };
            const auto &tokens = tokensAccumulator.getTokens();
            Parser parser(tokens, source);
            try {
                const auto &ast = parser.parse();
                astList.push_back(ast);
            } catch (const ParserError &exc) {
                std::cerr << formatError(exc, source) << std::endl;
                throw CLI::RuntimeError(1);
            };
        };
        parsers::schema::ASTSchema astSchema(astList);
        parsers::schema::Schema schema(astSchema);
    });
};
