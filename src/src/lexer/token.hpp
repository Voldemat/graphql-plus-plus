#ifndef GRAPHQL_TOKEN
#define GRAPHQL_TOKEN

#include <filesystem>
#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <variant>

struct SourceFile {
    const std::filesystem::path filepath;
};
struct Location {
    std::shared_ptr<SourceFile> source;
    unsigned int line = 1;
    unsigned int start = -1;
    unsigned int end = -1;
};
bool operator==(const Location& self, const Location &another) noexcept;
std::ostream& operator<<(std::ostream& os, const Location& self) noexcept;

enum class SimpleTokenType {
    EQUAL = 1,
    LEFT_PAREN = 2,
    RIGHT_PAREN = 3,
    LEFT_BRACE = 4,
    RIGHT_BRACE = 5,
    BANG = 6,
    SEMICOLON = 7,
    COLON = 8,
    COMMA = 9,
    VSLASH = 10
};
enum class ComplexTokenType {
    IDENTIFIER = 1,
    STRING = 2,
    NUMBER = 3
};

using GQLTokenType = std::variant<SimpleTokenType, ComplexTokenType>;

std::optional<GQLTokenType> gqlTokenTypeFromString(std::string t) noexcept;
std::string gqlTokenTypeToString(GQLTokenType type) noexcept;

struct GQLToken {
    GQLTokenType type;
    std::string lexeme;
    Location location;
};
bool operator==(const GQLToken& self, const GQLToken &token) noexcept;
std::ostream &operator<<(std::ostream &os, const GQLToken &self);
std::ostream &operator<<(std::ostream &os, const GQLTokenType &type);

#endif
