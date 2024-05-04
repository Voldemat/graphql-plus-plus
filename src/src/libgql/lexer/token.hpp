#ifndef GRAPHQL_TOKEN
#define GRAPHQL_TOKEN

#include <filesystem>
#include <format>
#include <iostream>
#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <variant>

struct SourceFile {
    std::filesystem::path filepath;
    SourceFile(const std::filesystem::path path): filepath{path} {};
    SourceFile(const SourceFile &) = delete;
    SourceFile(SourceFile&&) = delete;
    SourceFile& operator=(SourceFile&& other) = delete;
    SourceFile &operator=(SourceFile const &) = delete;
    ~SourceFile() = default;
};
struct Location {
    std::shared_ptr<SourceFile> source;
    unsigned int line = 1;
    unsigned int start = -1;
    unsigned int end = -1;

    explicit operator std::string() const {
        return std::format("{}:{} {}:{}", source->filepath.filename().string(),
                           line, start, end);
    };
};
bool operator==(const Location &self, const Location &another) noexcept;
std::ostream &operator<<(std::ostream &os, const Location &self) noexcept;

enum class SimpleTokenType {
    EQUAL,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    BANG,
    SEMICOLON,
    COLON,
    COMMA,
    VSLASH,
    LEFT_BRACKET,
    RIGHT_BRACKET
};
enum class ComplexTokenType { IDENTIFIER, STRING, NUMBER, BOOLEAN };

using GQLTokenType = std::variant<SimpleTokenType, ComplexTokenType>;

std::optional<GQLTokenType> gqlTokenTypeFromString(std::string t) noexcept;
std::string gqlTokenTypeToString(GQLTokenType type) noexcept;

struct GQLToken {
    GQLTokenType type;
    std::string lexeme;
    Location location;
};
bool operator==(const GQLToken &self, const GQLToken &token) noexcept;
std::ostream &operator<<(std::ostream &os, const GQLToken &self);
std::ostream &operator<<(std::ostream &os, const GQLTokenType &type);

#endif
