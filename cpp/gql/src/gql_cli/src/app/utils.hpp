#pragma once

#include <CLI/Error.hpp>
#include <filesystem>
#include <functional>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

#include "config/config.hpp"
#include "libgql/lexer/token.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/file/server/ast.hpp"
#include "libgql/parsers/file/shared/ast.hpp"
#include "libgql/parsers/schema/schema.hpp"
#include "libgql/parsers/schema/type_registry.hpp"
#include "rapidjson/stringbuffer.h"
#include "rapidjson/writer.h"

namespace cli::utils {
std::string getAllStdin() noexcept;

std::string serializeToJSONString(
    const std::function<void(rapidjson::Writer<rapidjson::StringBuffer> &)>
        &callback);

void ensureFileExists(const std::string &path);

std::string readFile(const std::string &path);
std::string readFile(const std::filesystem::path &path);

std::string readFromFileOrStdin(const std::string &path);

std::vector<lexer::GQLToken> parseTokensFromJSON(const std::string &buffer);
std::expected<std::vector<lexer::GQLToken>, std::string> parseTokensFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile);

std::vector<parsers::file::server::ast::ASTNode> parseServerNodesFromJSON(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens);
std::expected<std::vector<parsers::file::server::ast::ASTNode>, std::string> parseServerNodesFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens);

void ensureDirectoryExists(const std::string &path);

void ensureTokensPresent(const std::filesystem::path &filepath,
                         const std::vector<lexer::GQLToken> &tokens);

std::vector<std::filesystem::path> graphqlPathsInDirectory(
    const std::filesystem::path &filepath);

std::expected<std::vector<parsers::file::client::ast::ASTNode>, std::string> parseClientNodesFromGraphql(
    const std::shared_ptr<parsers::file::shared::ast::SourceFile> &sourceFile,
    const std::vector<lexer::GQLToken> &tokens);

template <typename T>
std::vector<T> parseNodesFromDirectory(
    const std::string &path,
    const std::function<std::expected<std::vector<T>, std::string>(
        const std::shared_ptr<parsers::file::shared::ast::SourceFile>
            &sourceFile,
        const std::vector<lexer::GQLToken> &tokens)> &parseCallback) {
    std::vector<T> nodes;
    for (const auto &filepath : graphqlPathsInDirectory(path)) {
        const auto &buffer = readFile(filepath);
        std::shared_ptr<parsers::file::shared::ast::SourceFile> source =
            std::make_shared<parsers::file::shared::ast::SourceFile>(filepath, buffer);
        const auto &tokens = parseTokensFromGraphql(source);
        if (!tokens.has_value()) {
            std::cerr << tokens.error() << std::endl;
            throw CLI::RuntimeError(1);
        };
        if (tokens->empty()) continue;
        const auto &astNodes = parseCallback(source, tokens.value());
        if (!astNodes.has_value()) {
            std::cerr << astNodes.has_value() << std::endl;
            throw CLI::RuntimeError(1);
        };
        nodes.insert(nodes.end(), astNodes->begin(), astNodes->end());
    };
    return nodes;
};

bool doesFileHaveChanges(
    const std::string& filepath,
    const std::string& newBuffer,
    const std::string& schemaName
);

std::expected<parsers::schema::ServerSchema,
              std::vector<std::string> >
loadServerSchemaFromInputs(parsers::schema::TypeRegistry &registry,
                               const config::InputsConfig &config,
                               const std::filesystem::path &configDirPath);
std::expected<parsers::schema::ClientSchema,
              std::vector<std::string> >
loadClientSchemaFromInputs(parsers::schema::TypeRegistry &registry,
                               const config::InputsConfig &config,
                               const std::filesystem::path &configDirPath);
};
