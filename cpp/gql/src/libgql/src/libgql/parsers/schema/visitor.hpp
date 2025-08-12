#pragma once

#include <functional>
#include <memory>
#include <optional>

#include "./schema.hpp"
#include "libgql/parsers/file/client/ast.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"

namespace gql::parsers::schema::visitor {
template <typename T>
using ASTVisitorHook = std::optional<std::function<void(const T &)>>;

struct ASTVisitorHooks {
    ASTVisitorHook<std::shared_ptr<ast::Fragment>> visitFragment;
    ASTVisitorHook<ast::FragmentSpec> visitFragmentSpec;
    ASTVisitorHook<ast::UnionFragmentSpec> visitUnionFragmentSpec;
    ASTVisitorHook<ast::UnionSelection> visitUnionSelection;
    ASTVisitorHook<ast::TypenameField> visitTypenameField;
    ASTVisitorHook<ast::SpreadSelection> visitSpreadSelection;
    ASTVisitorHook<ast::ObjectConditionalSpreadSelection>
        visitObjectConditionalSpreadSelection;
    ASTVisitorHook<std::shared_ptr<ast::ObjectType>> visitObjectType;
    ASTVisitorHook<ast::ObjectFragmentSpec<ast::ObjectType>>
        visitObjectFragmentSpec_ObjectType;
    ASTVisitorHook<ast::ObjectFragmentSpec<ast::Interface>>
        visitObjectFragmentSpec_Interface;
    ASTVisitorHook<std::shared_ptr<ast::Interface>> visitInterface;
    ASTVisitorHook<ast::ObjectSelection> visitObjectSelection;
    ASTVisitorHook<ast::FieldSelection> visitFieldSelection;
    ASTVisitorHook<ast::FieldSelectionArgument> visitFieldSelectionArgument;
    ASTVisitorHook<ast::ArgumentValue> visitArgumentValue;
    ASTVisitorHook<ast::ArgumentRefValue> visitArgumentRefValue;
    ASTVisitorHook<ast::ArgumentLiteralValue> visitArgumentLiteralValue;
    ASTVisitorHook<std::shared_ptr<ast::ClientDirective>> visitClientDirective;
    ASTVisitorHook<file::client::ast::DirectiveLocation>
        visitClientDirectiveLocation;
    ASTVisitorHook<ast::FieldDefinition<ast::InputFieldSpec>>
        visitFieldDefinition_InputFieldSpec;
    ASTVisitorHook<ast::FieldDefinition<ast::ObjectFieldSpec>>
        visitFieldDefinition_ObjectFieldSpec;
    ASTVisitorHook<ast::NonCallableFieldSpec<ast::InputTypeSpec>>
        visitNonCallableFieldSpec_InputTypeSpec;
    ASTVisitorHook<ast::LiteralFieldSpec<ast::InputTypeSpec>>
        visitLiteralFieldSpec_InputTypeSpec;
    ASTVisitorHook<ast::ArrayFieldSpec<ast::InputTypeSpec>>
        visitArrayFieldSpec_InputTypeSpec;
    ASTVisitorHook<ast::InputTypeSpec> visitInputTypeSpec;
    ASTVisitorHook<ast::NonCallableFieldSpec<ast::ObjectTypeSpec>>
        visitNonCallableFieldSpec_ObjectTypeSpec;
    ASTVisitorHook<ast::LiteralFieldSpec<ast::ObjectTypeSpec>>
        visitLiteralFieldSpec_ObjectTypeSpec;
    ASTVisitorHook<ast::ArrayFieldSpec<ast::ObjectTypeSpec>>
        visitArrayFieldSpec_ObjectTypeSpec;
    ASTVisitorHook<ast::ObjectFieldSpec> visitObjectFieldSpec;
    ASTVisitorHook<ast::CallableFieldSpec> visitCallableFieldSpec;
    ASTVisitorHook<ast::ObjectTypeSpec> visitObjectTypeSpec;
    ASTVisitorHook<std::shared_ptr<ast::Operation>> visitOperation;
    ASTVisitorHook<std::shared_ptr<ast::InputType>> visitInputType;
    ASTVisitorHook<std::shared_ptr<ast::Scalar>> visitScalar;
    ASTVisitorHook<std::shared_ptr<ast::Enum>> visitEnum;
    ASTVisitorHook<std::shared_ptr<ast::Union>> visitUnion;
};

void visitClientSchema(const ASTVisitorHooks &hooks, const ClientSchema &schema);
void visitServerSchema(const ASTVisitorHooks &hooks, const ServerSchema &schema);
};  // namespace gql::parsers::schema::visitor
