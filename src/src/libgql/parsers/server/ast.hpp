#ifndef GRAPHQL_PARSERS_SERVER_AST
#define GRAPHQL_PARSERS_SERVER_AST

#include <map>
#include <optional>
#include <string>
#include <variant>
#include <vector>

namespace parsers {
namespace server {
namespace ast {

enum class ASTGQLSimpleType { ID, STRING, INT, FLOAT, BOOLEAN };
struct ASTGQLReferenceType {
    std::string name;
};
using ASTGQLType = std::variant<ASTGQLSimpleType, ASTGQLReferenceType>;

struct ASTStringLiteral {
    std::string value;
};

struct ASTFloatLiteral {
    float value;
};

struct ASTIntLiteral {
    int value;
};

struct ASTBooleanLiteral {
    bool value;
};

using ASTLiteral = std::variant<ASTStringLiteral, ASTFloatLiteral,
                                ASTIntLiteral, ASTBooleanLiteral>;

using ASTStringArrayLiteral = std::vector<ASTStringLiteral>;
using ASTFloatArrayLiteral = std::vector<ASTFloatLiteral>;
using ASTIntArrayLiteral = std::vector<ASTIntLiteral>;
using ASTBooleanArrayLiteral = std::vector<ASTBooleanLiteral>;

using ASTArrayLiteral
    = std::variant<ASTStringArrayLiteral, ASTFloatArrayLiteral,
                   ASTIntArrayLiteral, ASTBooleanArrayLiteral>;

struct ASTTrivialTypeSpec {
    ASTGQLType type;
    bool nullable = true;
    std::optional<ASTLiteral> defaultValue;
};

struct ASTArrayTypeSpec {
    ASTTrivialTypeSpec type;
    bool nullable = true;
    std::optional<ASTArrayLiteral> defaultValue;
};

using ASTLiteralTypeSpec = std::variant<ASTTrivialTypeSpec, ASTArrayTypeSpec>;

struct ASTCallableTypeSpec {
    ASTLiteralTypeSpec returnType;
    std::map<std::string, ASTLiteralTypeSpec> arguments;
};

using ASTTypeSpec = std::variant<ASTLiteralTypeSpec, ASTCallableTypeSpec>;

struct ASTInterfaceDefinition {
    std::string name;
    std::map<std::string, ASTTypeSpec> fields;
};
struct ASTInputDefinition {
    std::string name;
    std::map<std::string, ASTTypeSpec> fields;

    ASTInputDefinition(const ASTInterfaceDefinition& node): name {node.name}, fields {node.fields} {};
};
struct ASTGQLTypeDefinition {
    std::string name;
    std::map<std::string, ASTTypeSpec> fields;
    std::optional<std::string> implements;
};

using ASTTypeDefinition = std::variant<
    ASTInterfaceDefinition,
    ASTInputDefinition,
    ASTGQLTypeDefinition
>;

struct ASTExtendNode {
    ASTTypeDefinition type;
};

struct ASTEnumNode {
    std::string name;
    std::vector<std::string> items;
};

struct ASTUnionNode {
    std::string name;
    std::vector<ASTGQLReferenceType> items;
};

using ASTNode = std::variant<ASTTypeDefinition, ASTTrivialTypeSpec,
                             ASTExtendNode, ASTUnionNode, ASTEnumNode>;

struct ASTProgram {
    std::vector<ASTNode> nodes;
};
std::string astGQLSimpleTypeToString(const ASTGQLSimpleType &type) noexcept;
};  // namespace ast
};  // namespace server
};  // namespace parsers
#endif
