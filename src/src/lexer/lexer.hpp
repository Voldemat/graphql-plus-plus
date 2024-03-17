#ifndef GRAPHQL_LEXER
#define GRAPHQL_LEXER
#include <_ctype.h>

#include <functional>
#include <optional>
#include <sstream>
#include <string>
#include <variant>

struct GQLVariable {
    std::string name;
};
struct GQLNumberLiteral {
    long value;
};
enum GQL_OP_TYPE { EQUAL };
struct GQLOperator {
    GQL_OP_TYPE type;
};
using GQL_TOKEN = std::variant<GQLVariable, GQLOperator, GQLNumberLiteral>;

class Lexer {
    std::istringstream stream;
    GQLNumberLiteral buildNumber(char start) noexcept;
    GQLVariable buildIdentifier(char start) noexcept;
    std::string readWhilePredicateIsTrue(
        char start, const std::function<bool(char)> predicate);
public:
    Lexer(std::istringstream s);
    std::optional<GQL_TOKEN*> nextToken() noexcept;
};
#endif
