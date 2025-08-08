#pragma once

#include <filesystem>
#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "rapidjson/stringbuffer.h"
#include "rapidjson/writer.h"

std::string getAllStdin() noexcept;

std::string serializeToJSONString(
    const std::function<void(rapidjson::Writer<rapidjson::StringBuffer> &)>
        &callback);

void ensureFileExists(const std::string &path);

std::string readFile(const std::string &path);
std::string readFile(const std::filesystem::path &path);

std::string readFromFileOrStdin(const std::string &path);

std::vector<lexer::GQLToken> parseTokensFromJSON(const std::string &buffer);
std::vector<lexer::GQLToken> parseTokensFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile);

std::vector<parsers::file::server::ast::ASTNode> parseServerNodesFromJSON(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens);
std::vector<parsers::file::server::ast::ASTNode> parseServerNodesFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens);

void ensureDirectoryExists(const std::string &path);

void ensureTokensPresent(const std::filesystem::path &filepath,
                         const std::vector<lexer::GQLToken> &tokens);

std::vector<std::filesystem::path> graphqlPathsInDirectory(
    const std::filesystem::path &filepath);

std::vector<parsers::file::client::ast::ASTNode> parseClientNodesFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens);

template <typename T>
std::vector<T> parseNodesFromDirectory(
    const std::string &path,
    const std::function<std::vector<T>(
        const std::shared_ptr<parsers::file::shared::ast::SourceFile>
            &sourceFile,
        const std::vector<lexer::GQLToken> &tokens)> &parseCallback) {
    std::vector<T> nodes;
    for (const auto &filepath : graphqlPathsInDirectory(path)) {
        const auto &buffer = readFile(filepath);
        std::shared_ptr<parsers::file::shared::ast::SourceFile> source =
            std::make_shared<parsers::file::shared::ast::SourceFile>(filepath, buffer);
        const auto &tokens = parseTokensFromGraphql(source);
        if (tokens.empty()) continue;
        const auto &astNodes = parseCallback(source, tokens);
        nodes.insert(nodes.end(), astNodes.begin(), astNodes.end());
    };
    return nodes;
};

bool doesFileHaveChanges(
    const std::string& filepath,
    const std::string& newBuffer
);
