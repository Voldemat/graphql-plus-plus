#ifndef GRAPHQL_PARSER
#define GRAPHQL_PARSER

#include <map>
#include <optional>
#include <sstream>
#include <string>
#include <variant>
#include <vector>

#include "lexer/token.hpp"

#define EXTRACT_OR_RETURN(VARIANT, ERROR_TYPE, VALUE_TYPE, \
                          VARIABLE_DECLARATION)            \
    if (!std::holds_alternative<ERROR_TYPE>(VARIANT)) {    \
        return std::get<ERROR_TYPE>(VARIANT);              \
    };                                                     \
    VARIABLE_DECLARATION = std::get<VALUE_TYPE>(VARIANT)

#define RETURN_ON_WRONG_TOKEN_TYPE(VARIABLE, EXPECTED_TYPE)     \
    if (VARIABLE.type != EXPECTED_TYPE) {                       \
        return ParserError::wrongType(VARIABLE, EXPECTED_TYPE); \
    }

#define RETURN_IF_EMPTY(VARIABLE, PREV_TOKEN) \
    if (!VARIABLE.has_value()) return ParserError::createEOF(PREV_TOKEN)
namespace parser {
enum class ASTGQLType { STRING, INT, FLOAT, BOOLEAN };
enum class ASTNodeKind { TYPE_OBJECT_DEFINITION, TYPE_SPEC };
enum class ASTGQLKeyword { TYPE, MUTATION, QUERY, INPUT };
struct ASTTypeSpec {
    const static ASTNodeKind kind = ASTNodeKind::TYPE_SPEC;
    const ASTGQLType type;
    const bool nullable;
};

class ASTTypeDefinition {
    const static ASTNodeKind kind = ASTNodeKind::TYPE_OBJECT_DEFINITION;
    const std::string name;
    const std::map<std::string, ASTTypeSpec> fields;
};

using ASTNode = std::variant<ASTTypeDefinition, ASTTypeSpec>;

struct ASTProgram {
    const std::vector<ASTNode> nodes;
};

struct ParserError {
    const GQLToken token;
    const std::string error;

    const static ParserError createEOF(const GQLToken token) noexcept {
        return { .token = token, .error = "EOF" };
    };

    const static ParserError wrongType(
        const GQLToken token, const GQLTokenType expectedType) noexcept {
        return { .token = token,
                 .error = (std::stringstream("Expected ")
                           << expectedType << " type, got " << token.type)
                              .str() };
    };

    const static ParserError identifierIsKeyword(
        const GQLToken token) noexcept {
        return { .token = token,
                 .error = token.lexeme + " is reserved keyword" };
    };
};

class Parser {
    unsigned int index = -1;
    const std::vector<GQLToken> tokens;
public:
    Parser(const std::vector<GQLToken> tokens) noexcept;
    const ASTProgram getAstTree() noexcept;
};
const bool isKeyword(const std::string lexeme) noexcept;
};  // namespace parser
#endif
