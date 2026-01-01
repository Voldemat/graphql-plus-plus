use std::rc::Rc;

use crate::parsers::file::shared;

pub enum DirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
    VariableDefinition,
}

impl TryFrom<&str> for DirectiveLocation {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "QUERY" => Ok(DirectiveLocation::Query),
            "MUTATION" => Ok(DirectiveLocation::Mutation),
            "SUBSCRIPTION" => Ok(DirectiveLocation::Subscription),
            "FIELD" => Ok(DirectiveLocation::Field),
            "FRAGMENT_DEFINITION" => Ok(DirectiveLocation::FragmentDefinition),
            "FRAGMENT_SPREAD" => Ok(DirectiveLocation::FragmentSpread),
            "INLINE_FRAGMENT" => Ok(DirectiveLocation::InlineFragment),
            "VARIABLE_DEFINITION" => Ok(DirectiveLocation::VariableDefinition),
            _ => Err(()),
        }
    }
}

pub type DirectiveLocationNode =
    shared::ast::DirectiveLocationNode<DirectiveLocation>;
pub type DirectiveDefinition = shared::ast::DirectiveNode<DirectiveLocation>;

pub struct ObjectLiteralFieldSpec {
    pub location: shared::ast::NodeLocation,
    pub selection_name: shared::ast::NameNode,
    pub name: shared::ast::NameNode,
}

pub struct ObjectCallableFieldSpec {
    pub location: shared::ast::NodeLocation,
    pub selection_name: shared::ast::NameNode,
    pub name: shared::ast::NameNode,
    pub arguments: Vec<shared::ast::Argument>,
}

#[derive(derive_more::From)]
pub enum ObjectFieldSpec {
    Literal(ObjectLiteralFieldSpec),
    Callable(ObjectCallableFieldSpec),
}

pub struct FieldSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub field: ObjectFieldSpec,
    pub spec: Option<Rc<FragmentSpec>>,
}

pub struct SpreadSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub fragment_name: shared::ast::NameNode,
}

pub struct ConditionalSpreadSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub type_name: shared::ast::NameNode,
    pub fragment: Rc<FragmentSpec>,
}

#[derive(derive_more::From)]
pub enum SelectionNode {
    FieldSelectionNode(FieldSelectionNode),
    ConditionalSpreadSelectionNode(ConditionalSpreadSelectionNode),
    SpreadSelectionNode(SpreadSelectionNode),
}

pub struct FragmentSpec {
    pub location: shared::ast::NodeLocation,
    pub selections: Vec<SelectionNode>,
}

#[derive(Clone, Copy)]
pub enum OpType {
    Mutation,
    Query,
    Subscription,
}

impl OpType {
    pub fn from_object_name(name: &str) -> Option<Self> {
        match name {
            "Query" => Some(Self::Query),
            "Mutation" => Some(Self::Mutation),
            "Subscription" => Some(Self::Subscription),
            _ => None
        }
    }
}

impl TryFrom<&str> for OpType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "mutation" => Ok(Self::Mutation),
            "query" => Ok(Self::Query),
            "subscription" => Ok(Self::Subscription),
            _ => Err(()),
        }
    }
}

pub struct OperationDefinition {
    pub location: shared::ast::NodeLocation,
    pub r#type: OpType,
    pub name: shared::ast::NameNode,
    pub parameters:
        indexmap::IndexMap<String, shared::ast::InputFieldDefinitionNode>,
    pub fragment: FragmentSpec,
}

pub struct FragmentDefinition {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub type_name: shared::ast::NameNode,
    pub spec: FragmentSpec,
}

#[derive(derive_more::From)]
pub enum ASTNode {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
    Directive(DirectiveDefinition),
}
