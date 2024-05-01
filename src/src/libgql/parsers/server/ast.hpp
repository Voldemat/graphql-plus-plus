#ifndef GRAPHQL_PARSERS_SERVER_AST
#define GRAPHQL_PARSERS_SERVER_AST

#include <map>
#include <string>
#include <variant>
#include <vector>

namespace parsers {
namespace server {
namespace ast {
enum class ASTGQLSimpleType { STRING, INT, FLOAT, BOOLEAN };
std::string astGQLSimpleTypeToString(const ASTGQLSimpleType &type) noexcept;
struct ASTGQLReferenceType {
    std::string name;
};

using ASTGQLType = std::variant<ASTGQLSimpleType, ASTGQLReferenceType>;

struct ASTTrivialTypeSpec {
    ASTGQLType type;
    bool nullable;
};

struct ASTArrayTypeSpec {
    ASTTrivialTypeSpec type;
    bool nullable;
};

using ASTLiteralTypeSpec = std::variant<ASTTrivialTypeSpec, ASTArrayTypeSpec>;

struct ASTCallableTypeSpec {
    ASTLiteralTypeSpec returnType;
    std::map<std::string, ASTLiteralTypeSpec> arguments;
};

using ASTTypeSpec = std::variant<ASTLiteralTypeSpec, ASTCallableTypeSpec>;

struct ASTTypeDefinition {
    std::string name;
    std::map<std::string, ASTTypeSpec> fields;
    bool isInput;
};

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
};  // namespace ast
};  // namespace server
};  // namespace parsers
#endif
