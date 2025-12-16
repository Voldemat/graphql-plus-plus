use std::rc::Rc;

use crate::parsers::file::shared;

enum DirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
    VariableDefinition,
}

type DirectiveLocationNode =
    shared::ast::DirectiveLocationNode<DirectiveLocation>;
type DirectiveDefinition = shared::ast::DirectiveNode<DirectiveLocation>;

struct ObjectLiteralFieldSpec {
    location: shared::ast::NodeLocation,
    selection_name: shared::ast::NameNode,
    name: shared::ast::NameNode,
}

struct ObjectCallableFieldSpec {
    location: shared::ast::NodeLocation,
    selection_name: shared::ast::NameNode,
    name: shared::ast::NameNode,
    arguments: Vec<shared::ast::Argument>,
}

enum ObjectFieldSpec {
    Literal(ObjectLiteralFieldSpec),
    Callable(ObjectCallableFieldSpec),
}

struct FieldSelectionNode {
    location: shared::ast::NodeLocation,
    field: ObjectFieldSpec,
    spec: Option<Rc<FragmentSpec>>,
}

struct SpreadSelectionNode {
    location: shared::ast::NodeLocation,
    fragment_name: shared::ast::NameNode,
}

struct ConditionalSpreadSelectionNode {
    location: shared::ast::NodeLocation,
    type_name: shared::ast::NameNode,
    fragment: Rc<FragmentSpec>,
}

enum SelectionNode {
    FieldSelectionNode(FieldSelectionNode),
    ConditionalSpreadSelectionNode(ConditionalSpreadSelectionNode),
    SpreadSelectionNode(SpreadSelectionNode),
}

struct FragmentSpec {
    location: shared::ast::NodeLocation,
    selections: Vec<SelectionNode>,
}

enum OpType {
    Mutation,
    Query,
    Subscription,
}

struct OperationDefinition {
    location: shared::ast::NodeLocation,
    r#type: OpType,
    name: shared::ast::NameNode,
    parameters:
        indexmap::IndexMap<String, shared::ast::InputValueDefinitionNode>,
    fragment: FragmentSpec,
}

struct FragmentDefinition {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    type_name: shared::ast::NameNode,
    spec: FragmentSpec,
}

enum ASTNode {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
    Directive(DirectiveDefinition),
}
