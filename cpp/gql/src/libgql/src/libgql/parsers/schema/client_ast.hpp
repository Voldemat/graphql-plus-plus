#pragma once

#include <cstddef>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "../file/client/ast.hpp"
#include "libgql/parsers/schema/server_ast.hpp"
#include "libgql/parsers/schema/shared_ast.hpp"

namespace gql::parsers::schema::ast {

struct Fragment;
struct FieldSelection;
struct ObjectConditionalSpreadSelection;
struct UnionConditionalSpreadSelection;
struct SpreadSelection {
    std::shared_ptr<Fragment> fragment;
};

struct TypenameField {
    std::optional<std::string> alias;
};
using UnionSelection = std::variant<TypenameField, SpreadSelection,
                                    ObjectConditionalSpreadSelection,
                                    UnionConditionalSpreadSelection>;
using ObjectSelection =
    std::variant<TypenameField, SpreadSelection, FieldSelection>;

struct UnionFragmentSpec {
    std::shared_ptr<Union> type;
    std::vector<UnionSelection> selections;
};

template <typename T>
struct ObjectFragmentSpec {
    std::shared_ptr<T> type;
    std::vector<ObjectSelection> selections;
};

struct ObjectConditionalSpreadSelection {
    std::shared_ptr<ObjectType> type;
    std::shared_ptr<ObjectFragmentSpec<ObjectType>> selection;
};

struct UnionConditionalSpreadSelection {
    std::shared_ptr<Union> type;
    std::shared_ptr<UnionFragmentSpec> selection;
};

using FragmentSpec =
    std::variant<UnionFragmentSpec, ObjectFragmentSpec<ObjectType>,
                 ObjectFragmentSpec<Interface>>;


struct FieldSelection {
    std::string name;
    std::string alias;
    std::map<std::string, FieldSelectionArgument> arguments;
    std::optional<std::shared_ptr<FragmentSpec>> selection;
};

struct Fragment {
    std::string name;
    FragmentSpec spec;
    std::string sourceText;
};

struct Operation {
    file::client::ast::OpType type;
    std::string name;
    std::map<std::string, FieldDefinition<InputFieldSpec>> parameters;
    FragmentSpec fragmentSpec;
    std::string sourceText;
    std::size_t hash;
};

struct ClientDirective {
    std::string name;
    std::map<std::string, std::shared_ptr<FieldDefinition<InputFieldSpec>>>
        arguments;
    std::vector<file::client::ast::DirectiveLocation> locations;
};


using ClientSchemaNode =
    std::variant<std::shared_ptr<Fragment>, std::shared_ptr<Operation>, std::shared_ptr<ClientDirective>>;

};  // namespace parsers::schema::ast
