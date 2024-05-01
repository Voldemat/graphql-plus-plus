#include "./ast.hpp"
#include <string>

std::string parsers::server::ast::astGQLSimpleTypeToString(
    const ASTGQLSimpleType &type) noexcept {
    switch (type) {
        case ASTGQLSimpleType::INT:
            return "Int";
        case ASTGQLSimpleType::FLOAT:
            return "Float";
        case ASTGQLSimpleType::STRING:
            return "String";
        case ASTGQLSimpleType::BOOLEAN:
            return "Boolean";
        case ASTGQLSimpleType::ID:
            return "Id";
    };
};
