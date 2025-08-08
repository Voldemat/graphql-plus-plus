#include "./utils.hpp"

#include <CLI/Error.hpp>
#include <exception>
#include <filesystem>
#include <format>
#include <fstream>
#include <functional>
#include <iostream>
#include <iterator>
#include <memory>
#include <ranges>
#include <string>
#include <vector>

#include "app/formatting/error.hpp"
#include "libgql/json/parsers/lexer/lexer.hpp"
#include "libgql/json/parsers/shared.hpp"
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/lexer_error.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/lexer/tokens_accumulators.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/client/parser.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/server/parser.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/file/shared/parser_error.hpp"
#include "rapidjson/document.h"
#include "rapidjson/stringbuffer.h"
#include "rapidjson/writer.h"

std::string getAllStdin() noexcept {
    std::string lineInput;
    std::string buffer;
    while (std::getline(std::cin, lineInput)) {
        buffer += lineInput;
        buffer += '\n';
    };
    return buffer;
};

std::string serializeToJSONString(
    const std::function<void(rapidjson::Writer<rapidjson::StringBuffer> &)>
        &callback) {
    rapidjson::StringBuffer buffer;
    rapidjson::Writer writer(buffer);
    callback(writer);
    return buffer.GetString();
};

void ensureFileExists(const std::string &path) {
    if (!std::filesystem::exists(path)) {
        std::cerr << std::format("File {} does not exists", path) << std::endl;
        throw CLI::RuntimeError(1);
    };
};

std::string readFile(const std::string &path) {
    std::ifstream file(path);
    return std::string((std::istreambuf_iterator<char>(file)),
                       std::istreambuf_iterator<char>());
};

std::string readFile(const std::filesystem::path &path) {
    std::ifstream file(path);
    return std::string((std::istreambuf_iterator<char>(file)),
                       std::istreambuf_iterator<char>());
};

std::string readFromFileOrStdin(const std::string &path) {
    if (path == "-") {
        return getAllStdin();
    }
    ensureFileExists(path);
    return readFile(path);
};

std::vector<lexer::GQLToken> parseTokensFromJSON(const std::string &buffer) {
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
    return tokens;
};

std::vector<lexer::GQLToken> parseTokensFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile) {
    lexer::VectorTokensAccumulator tokensAccumulator;
    lexer::Lexer lexer(sourceFile->buffer, &tokensAccumulator);
    try {
        lexer.parse();
    } catch (const lexer::LexerError &error) {
        std::cerr << std::format("LexerParserError({}): {}",
                                 sourceFile->filepath.filename().string(),
                                 error.what())
                  << std::endl;
        throw CLI::RuntimeError(1);
    };
    return tokensAccumulator.getTokens();
};

std::vector<parsers::file::server::ast::ASTNode> parseServerNodesFromJSON(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens) {
    auto parser = parsers::file::server::Parser(tokens, sourceFile);
    std::vector<parsers::file::server::ast::ASTNode> astNodes;
    try {
        return parser.parse();
    } catch (const std::exception &exc) {
        std::cerr << "Parsing error: " << exc.what() << std::endl;
        throw CLI::RuntimeError(1);
    };
};

std::vector<parsers::file::server::ast::ASTNode> parseServerNodesFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens) {
    parsers::file::server::Parser parser(tokens, sourceFile);
    try {
        return parser.parse();
    } catch (const parsers::file::shared::ParserError &exc) {
        std::cerr << formatError(exc) << std::endl;
        throw CLI::RuntimeError(1);
    };
};

void ensureDirectoryExists(const std::string &path) {
    if (!std::filesystem::exists(path)) {
        std::cerr << std::format("Directory \"{}\" does not exists", path)
                  << std::endl;
        throw CLI::RuntimeError(1);
    };
    if (!std::filesystem::is_directory(std::filesystem::status(path))) {
        std::cerr << std::format("Path {} is not directory", path) << std::endl;
        throw CLI::RuntimeError(1);
    };
};

void ensureTokensPresent(const std::filesystem::path &filepath,
                         const std::vector<lexer::GQLToken> &tokens) {
    if (tokens.size() == 0) {
        std::cerr << "No tokens in file: " << filepath.filename() << std::endl;
        throw CLI::RuntimeError(1);
    }
};

std::vector<std::filesystem::path> graphqlPathsInDirectory(
    const std::filesystem::path &dirpath) {
    return std::filesystem::recursive_directory_iterator(dirpath) |
           std::views::filter(
               [](const std::filesystem::directory_entry &entry) {
                   return entry.path().extension() == ".graphql";
               }) |
           std::views::transform(
               [](const std::filesystem::directory_entry &entry) {
                   return entry.path();
               }) |
           std::ranges::to<std::vector>();
};

std::vector<parsers::file::client::ast::ASTNode> parseClientNodesFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens) {
    parsers::file::client::Parser parser(tokens, sourceFile);
    try {
        return parser.parse();
    } catch (const parsers::file::shared::ParserError &exc) {
        std::cerr << formatError(exc) << std::endl;
        throw CLI::RuntimeError(1);
    };
};

bool doesFileHaveChanges(
    const std::string& filepath,
    const std::string& newBuffer
) {
    const auto& oldBuffer = readFile(filepath);
    if (oldBuffer == newBuffer) {
        std::cout << "Schema is up to date" << std::endl;
        return false;
    };
    std::cout << "Schema is not up to date" << std::endl;
    return true;
};
