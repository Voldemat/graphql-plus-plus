#ifndef GRAPHQL_JSON_SERIALIZER
#define GRAPHQL_JSON_SERIALIZER

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>
#include <rapidjson/reader.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "libgql/lexer/token.hpp"
#include "libgql/parsers/server/ast.hpp"

namespace json {
namespace serializer {
void writeTokenAsJSON(
    rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
        &writer,
    const GQLToken &token);

class ASTJSONWriter {
    rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
        &writer;

public:
    ASTJSONWriter(
        rapidjson::Writer<rapidjson::GenericStringBuffer<rapidjson::UTF8<>>>
            &writer);
    void writeProgram(const parsers::server::ast::ASTProgram &program) noexcept;
    void writeNode(const parsers::server::ast::ASTNode &node) noexcept;
    void writeUnionNode(const parsers::server::ast::ASTUnionNode &node) noexcept;
    void writeEnumNode(const parsers::server::ast::ASTEnumNode &node) noexcept;
    void writeTrivialTypeSpecNode(
        const parsers::server::ast::ASTTrivialTypeSpec &node) noexcept;
    void writeLiteralTypeSpecNode(
        const parsers::server::ast::ASTLiteralTypeSpec &node) noexcept;
    void writeArrayTypeSpecNode(
        const parsers::server::ast::ASTArrayTypeSpec &node) noexcept;
    void writeCallableTypeSpecNode(
        const parsers::server::ast::ASTCallableTypeSpec &node) noexcept;
    void writeTypeSpecNode(const parsers::server::ast::ASTTypeSpec &node) noexcept;
    void writeASTGQLType(const parsers::server::ast::ASTGQLType &type) noexcept;
    void writeTypeDefinitionNode(
        const parsers::server::ast::ASTTypeDefinition &node) noexcept;
    void writeReferenceTypeNode(
        const parsers::server::ast::ASTGQLReferenceType &node) noexcept;
    void writeExtendNode(const parsers::server::ast::ASTExtendNode &node) noexcept;
};
};  // namespace serializer
};  // namespace json
#endif
