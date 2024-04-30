#ifndef GRAPHQL_JSON_SERIALIZER
#define GRAPHQL_JSON_SERIALIZER

#include <rapidjson/document.h>
#include <rapidjson/encodings.h>
#include <rapidjson/reader.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "lexer/token.hpp"
#include "parsers/server/parser.hpp"

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
    void writeProgram(const parsers::server::ASTProgram &program) noexcept;
    void writeNode(const parsers::server::ASTNode &node) noexcept;
    void writeUnionNode(const parsers::server::ASTUnionNode &node) noexcept;
    void writeEnumNode(const parsers::server::ASTEnumNode &node) noexcept;
    void writeTrivialTypeSpecNode(const parsers::server::ASTTrivialTypeSpec &node) noexcept;
    void writeLiteralTypeSpecNode(const parsers::server::ASTLiteralTypeSpec &node) noexcept;
    void writeArrayTypeSpecNode(const parsers::server::ASTArrayTypeSpec &node) noexcept;
    void writeCallableTypeSpecNode(const parsers::server::ASTCallableTypeSpec &node) noexcept;
    void writeTypeSpecNode(const parsers::server::ASTTypeSpec &node) noexcept;
    void writeASTGQLType(const parsers::server::ASTGQLType &type) noexcept;
    void writeTypeDefinitionNode(
        const parsers::server::ASTTypeDefinition &node) noexcept;
    void writeReferenceTypeNode(
        const parsers::server::ASTGQLReferenceType &node) noexcept;
    void writeExtendNode(const parsers::server::ASTExtendNode &node) noexcept;
};
};  // namespace serializer
};  // namespace json
#endif
