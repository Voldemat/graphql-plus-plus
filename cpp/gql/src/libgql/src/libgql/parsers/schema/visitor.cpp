#include "./visitor.hpp"

#include <memory>
#include <ranges>
#include <variant>

#include "./schema.hpp"
#include "libgql/parsers/schema/client_ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"
#include "utils.hpp"

namespace gql::parsers::schema::visitor {
void visitFieldSelection(const ASTVisitorHooks &hooks,
                         const ast::FieldSelection &selection) {
    if (hooks.visitFieldSelection.has_value()) {
        hooks.visitFieldSelection.value()(selection);
    };
    for (const auto &argument : selection.arguments | std::views::values) {
        if (hooks.visitFieldSelectionArgument.has_value()) {
            hooks.visitFieldSelectionArgument.value()(argument);
        };
        if (hooks.visitArgumentValue.has_value()) {
            hooks.visitArgumentValue.value()(argument.value);
        };
        std::visit(
            utils::overloaded{
                [&hooks](const ast::ArgumentRefValue &value) -> void {
                    if (hooks.visitArgumentRefValue.has_value()) {
                        hooks.visitArgumentRefValue.value()(value);
                    };
                },
                [&hooks](const ast::ArgumentLiteralValue &value) -> void {
                    if (hooks.visitArgumentLiteralValue.has_value()) {
                        hooks.visitArgumentLiteralValue.value()(value);
                    };
                },
            },
            argument.value);
    };
    if (selection.selection.has_value()) {
        const auto &fSelection = *selection.selection.value().get();
        visitFragmentSpec(hooks, fSelection);
    };
};

void visitObjectFieldSpec(const ASTVisitorHooks &hooks,
                          const ast::ObjectFieldSpec &spec);
void visitFieldDefinition_ObjectFieldSpec(
    const ASTVisitorHooks &hooks,
    const std::shared_ptr<ast::FieldDefinition<ast::ObjectFieldSpec>> &field) {
    if (hooks.visitFieldDefinition_ObjectFieldSpec.has_value()) {
        hooks.visitFieldDefinition_ObjectFieldSpec.value()(field);
    };
    visitObjectFieldSpec(hooks, field->spec);
};

template <typename T>
void visitObjectSelection(const ASTVisitorHooks &hooks,
                          const ast::ObjectSelection &objectSelection,
                          const std::shared_ptr<T> &object) {
    if (hooks.visitObjectSelection.has_value()) {
        hooks.visitObjectSelection.value()(objectSelection);
    };
    std::visit(utils::overloaded{
                   [&hooks](const ast::TypenameField &field) -> void {
                       if (hooks.visitTypenameField.has_value()) {
                           hooks.visitTypenameField.value()(field);
                       };
                   },
                   [&hooks](const ast::SpreadSelection &field) -> void {
                       if (hooks.visitSpreadSelection.has_value()) {
                           hooks.visitSpreadSelection.value()(field);
                       };
                       if (hooks.visitFragment.has_value()) {
                           hooks.visitFragment.value()(field.fragment);
                       };
                       visitFragmentSpec(hooks, field.fragment->spec);
                   },
                   [&hooks, &object](const ast::FieldSelection &field) -> void {
                       visitFieldSelection(hooks, field);
                       const auto &f = object->fields[field.name];
                       visitFieldDefinition_ObjectFieldSpec(hooks, f);
                   },
               },
               objectSelection);
};

void visitObjectFragmentSpec_ObjectType(
    const ASTVisitorHooks &hooks,
    const ast::ObjectFragmentSpec<ast::ObjectType> &spec);
void visitUnionSelection(const ASTVisitorHooks &hooks,
                         const ast::UnionSelection &selection) {
    if (hooks.visitUnionSelection.has_value()) {
        hooks.visitUnionSelection.value()(selection);
    };
    std::visit(
        utils::overloaded{
            [&hooks](const ast::TypenameField &field) -> void {
                if (hooks.visitTypenameField.has_value()) {
                    hooks.visitTypenameField.value()(field);
                };
            },
            [&hooks](const ast::SpreadSelection &field) -> void {
                if (hooks.visitSpreadSelection.has_value()) {
                    hooks.visitSpreadSelection.value()(field);
                };
                if (hooks.visitFragment.has_value()) {
                    hooks.visitFragment.value()(field.fragment);
                };
                visitFragmentSpec(hooks, field.fragment->spec);
            },
            [&hooks](
                const ast::ObjectConditionalSpreadSelection &field) -> void {
                if (hooks.visitObjectConditionalSpreadSelection.has_value()) {
                    hooks.visitObjectConditionalSpreadSelection.value()(field);
                };
                visitObjectFragmentSpec_ObjectType(hooks,
                                                   *field.selection.get());
            },
            [](const ast::UnionConditionalSpreadSelection &) {} },
        selection);
};

void visitObjectFragmentSpec_ObjectType(
    const ASTVisitorHooks &hooks,
    const ast::ObjectFragmentSpec<ast::ObjectType> &spec) {
    if (hooks.visitObjectFragmentSpec_ObjectType.has_value()) {
        hooks.visitObjectFragmentSpec_ObjectType.value()(spec);
    };
    if (hooks.visitObjectType.has_value()) {
        hooks.visitObjectType.value()(spec.type);
    };
    for (const auto &selection : spec.selections) {
        visitObjectSelection(hooks, selection, spec.type);
    };
};

void visitObjectFragmentSpec_Interface(
    const ASTVisitorHooks &hooks,
    const ast::ObjectFragmentSpec<ast::Interface> &spec) {
    if (hooks.visitObjectFragmentSpec_Interface.has_value()) {
        hooks.visitObjectFragmentSpec_Interface.value()(spec);
    };
    for (const auto &selection : spec.selections) {
        visitObjectSelection(hooks, selection, spec.type);
    };
};

void visitFieldDefinition_InputFieldSpec(
    const ASTVisitorHooks &hooks,
    const ast::FieldDefinition<ast::InputFieldSpec> &field);
void visitInputType(const ASTVisitorHooks &hooks,
                    const std::shared_ptr<ast::InputType> &input) {
    if (hooks.visitInputType.has_value()) {
        hooks.visitInputType.value()(input);
    };
    for (const auto &field : input->fields | std::views::values) {
        visitFieldDefinition_InputFieldSpec(hooks, field);
    };
};

void visitInputTypeSpec(const ASTVisitorHooks &hooks,
                        const ast::InputTypeSpec &spec) {
    if (hooks.visitInputTypeSpec.has_value()) {
        hooks.visitInputTypeSpec.value()(spec);
    };
    std::visit(utils::overloaded{
                   [&hooks](const std::shared_ptr<ast::Scalar> &scalar) {
                       if (hooks.visitScalar.has_value()) {
                           hooks.visitScalar.value()(scalar);
                       };
                   },
                   [&hooks](const std::shared_ptr<ast::Enum> &enumType) {
                       if (hooks.visitEnum.has_value()) {
                           hooks.visitEnum.value()(enumType);
                       };
                   },
                   [&hooks](const std::shared_ptr<ast::InputType> &input) {
                       visitInputType(hooks, input);
                   } },
               spec);
};

void visitObjectTypeSpec(const ASTVisitorHooks &hooks,
                         const ast::ObjectTypeSpec &spec) {
    if (hooks.visitObjectTypeSpec.has_value()) {
        hooks.visitObjectTypeSpec.value()(spec);
    };
    std::visit(utils::overloaded{
                   [&hooks](const std::shared_ptr<ast::Scalar> &scalar) {
                       if (hooks.visitScalar.has_value()) {
                           hooks.visitScalar.value()(scalar);
                       };
                   },
                   [&hooks](const std::shared_ptr<ast::Enum> &enumType) {
                       if (hooks.visitEnum.has_value()) {
                           hooks.visitEnum.value()(enumType);
                       };
                   },
                   [&hooks](const std::shared_ptr<ast::ObjectType> &object) {
                       if (hooks.visitObjectType.has_value()) {
                           hooks.visitObjectType.value()(object);
                       };
                   },
                   [&hooks](const std::shared_ptr<ast::Interface> &interface) {
                       if (hooks.visitInterface.has_value()) {
                           hooks.visitInterface.value()(interface);
                       };
                   },
                   [&hooks](const std::shared_ptr<ast::Union> &unionType) {
                       if (hooks.visitUnion.has_value()) {
                           hooks.visitUnion.value()(unionType);
                       };
                   } },
               spec);
};

void visitInputFieldSpec(const ASTVisitorHooks &hooks,
                         const ast::InputFieldSpec &spec) {
    if (hooks.visitNonCallableFieldSpec_InputTypeSpec.has_value()) {
        hooks.visitNonCallableFieldSpec_InputTypeSpec.value()(spec);
    };
    std::visit(
        utils::overloaded{
            [&hooks](const ast::LiteralFieldSpec<ast::InputTypeSpec> &literal) {
                if (hooks.visitLiteralFieldSpec_InputTypeSpec.has_value()) {
                    hooks.visitLiteralFieldSpec_InputTypeSpec.value()(literal);
                };
                visitInputTypeSpec(hooks, literal.type);
            },
            [&hooks](const ast::ArrayFieldSpec<ast::InputTypeSpec> &array) {
                if (hooks.visitArrayFieldSpec_InputTypeSpec.has_value()) {
                    hooks.visitArrayFieldSpec_InputTypeSpec.value()(array);
                };
                visitInputTypeSpec(hooks, array.type);
            },
        },
        spec);
};

void visitNonCallableFieldSpec_ObjectTypeSpec(
    const ASTVisitorHooks &hooks,
    const ast::NonCallableFieldSpec<ast::ObjectTypeSpec> &spec) {
    if (hooks.visitNonCallableFieldSpec_ObjectTypeSpec.has_value()) {
        hooks.visitNonCallableFieldSpec_ObjectTypeSpec.value()(spec);
    };
    std::visit(
        utils::overloaded{
            [&hooks](
                const ast::LiteralFieldSpec<ast::ObjectTypeSpec> &literal) {
                if (hooks.visitLiteralFieldSpec_ObjectTypeSpec.has_value()) {
                    hooks.visitLiteralFieldSpec_ObjectTypeSpec.value()(literal);
                };
                visitObjectTypeSpec(hooks, literal.type);
            },
            [&hooks](const ast::ArrayFieldSpec<ast::ObjectTypeSpec> &array) {
                if (hooks.visitArrayFieldSpec_ObjectTypeSpec.has_value()) {
                    hooks.visitArrayFieldSpec_ObjectTypeSpec.value()(array);
                };
                visitObjectTypeSpec(hooks, array.type);
            },
        },
        spec);
};

void visitObjectFieldSpec(const ASTVisitorHooks &hooks,
                          const ast::ObjectFieldSpec &spec) {
    if (hooks.visitObjectFieldSpec.has_value()) {
        hooks.visitObjectFieldSpec.value()(spec);
    };
    std::visit(
        utils::overloaded{
            [&hooks](
                const ast::LiteralFieldSpec<ast::ObjectTypeSpec> &literal) {
                if (hooks.visitLiteralFieldSpec_ObjectTypeSpec.has_value()) {
                    hooks.visitLiteralFieldSpec_ObjectTypeSpec.value()(literal);
                };
                visitObjectTypeSpec(hooks, literal.type);
            },
            [&hooks](const ast::ArrayFieldSpec<ast::ObjectTypeSpec> &array) {
                if (hooks.visitArrayFieldSpec_ObjectTypeSpec.has_value()) {
                    hooks.visitArrayFieldSpec_ObjectTypeSpec.value()(array);
                };
                visitObjectTypeSpec(hooks, array.type);
            },
            [&hooks](const ast::CallableFieldSpec &spec) {
                if (hooks.visitCallableFieldSpec.has_value()) {
                    hooks.visitCallableFieldSpec.value()(spec);
                };
                for (const auto &argument :
                     spec.arguments | std::views::values) {
                    visitFieldDefinition_InputFieldSpec(hooks, *argument.get());
                };
                visitNonCallableFieldSpec_ObjectTypeSpec(hooks,
                                                         spec.returnType);
            },
        },
        spec);
};

void visitFieldDefinition_InputFieldSpec(
    const ASTVisitorHooks &hooks,
    const ast::FieldDefinition<ast::InputFieldSpec> &field) {
    if (hooks.visitFieldDefinition_InputFieldSpec.has_value()) {
        hooks.visitFieldDefinition_InputFieldSpec.value()(field);
    };
    visitInputFieldSpec(hooks, field.spec);
};

void visitUnion(const ASTVisitorHooks &hooks,
                const std::shared_ptr<ast::Union> &unionType) {
    if (hooks.visitUnion.has_value()) {
        hooks.visitUnion.value()(unionType);
    };
    for (const auto &object : unionType->items | std::views::values) {
        if (hooks.visitObjectType.has_value()) {
            hooks.visitObjectType.value()(object);
        };
        for (const auto &field : object->fields | std::views::values) {
            visitObjectFieldSpec(hooks, field->spec);
        };
    };
};

void visitFragmentSpec(const ASTVisitorHooks &hooks,
                       const ast::FragmentSpec &fragmentSpec) {
    if (hooks.visitFragmentSpec.has_value()) {
        hooks.visitFragmentSpec.value()(fragmentSpec);
    };
    std::visit(
        utils::overloaded{
            [&hooks](const ast::ObjectFragmentSpec<ast::ObjectType> &spec) {
                visitObjectFragmentSpec_ObjectType(hooks, spec);
            },
            [&hooks](const ast::ObjectFragmentSpec<ast::Interface> &spec) {
                visitObjectFragmentSpec_Interface(hooks, spec);
            },
            [&hooks](const ast::UnionFragmentSpec &spec) {
                if (hooks.visitUnionFragmentSpec.has_value()) {
                    hooks.visitUnionFragmentSpec.value()(spec);
                };
                visitUnion(hooks, spec.type);
                for (const auto &selection : spec.selections) {
                    visitUnionSelection(hooks, selection);
                };
            } },
        fragmentSpec);
};

void visitClientSchema(const ASTVisitorHooks &hooks,
                       const ClientSchema &schema) {
    for (const auto &directive : schema.directives | std::views::values) {
        if (hooks.visitClientDirective.has_value()) {
            hooks.visitClientDirective.value()(directive);
        };
        if (hooks.visitClientDirectiveLocation.has_value()) {
            for (const auto &location : directive->locations) {
                hooks.visitClientDirectiveLocation.value()(location);
            };
        };
    };
    for (const auto &fragment : schema.fragments | std::views::values) {
        if (hooks.visitFragment.has_value()) {
            hooks.visitFragment.value()(fragment);
        };
        visitFragmentSpec(hooks, fragment->spec);
    };

    for (const auto &operation : schema.operations | std::views::values) {
        if (hooks.visitOperation.has_value()) {
            hooks.visitOperation.value()(operation);
        };
        for (const auto &parameter :
             operation->parameters | std::views::values) {
            visitFieldDefinition_InputFieldSpec(hooks, parameter);
        };
        visitFragmentSpec(hooks, operation->fragmentSpec);
    };
};

void visitServerSchema(const ASTVisitorHooks &hooks,
                       const ServerSchema &schema) {};
};  // namespace gql::parsers::schema::visitor
