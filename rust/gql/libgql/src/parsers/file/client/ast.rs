use std::rc::Rc;

use crate::parsers::file::shared;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct ObjectLiteralFieldSpec {
    pub location: shared::ast::NodeLocation,
    pub selection_name: shared::ast::NameNode,
    pub name: shared::ast::NameNode,
}

#[derive(Debug, Clone)]
pub struct ObjectCallableFieldSpec {
    pub location: shared::ast::NodeLocation,
    pub selection_name: shared::ast::NameNode,
    pub name: shared::ast::NameNode,
    pub arguments: Vec<shared::ast::Argument>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ObjectFieldSpec {
    Literal(ObjectLiteralFieldSpec),
    Callable(ObjectCallableFieldSpec),
}

impl ObjectFieldSpec {
    pub fn get_name(self: &Self) -> &shared::ast::NameNode {
        match self {
        Self::Literal(literal) => &literal.name,
        Self::Callable(callable) => &callable.name,
        }
    }

    pub fn get_selection_name(self: &Self) -> &shared::ast::NameNode {
        match self {
        Self::Literal(literal) => &literal.selection_name,
        Self::Callable(callable) => &callable.selection_name,
        }
    }

    pub fn get_alias(self: &Self) -> Option<String> {
        match self {
        Self::Literal(literal) => {
            if literal.name.name == literal.selection_name.name {
                return None;
            }
            return Some(literal.selection_name.name.clone());
        }
        Self::Callable(callable) => {
            if callable.name.name == callable.selection_name.name {
                return None;
            }
            return Some(callable.selection_name.name.clone());
        }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub field: ObjectFieldSpec,
    pub spec: Option<Rc<FragmentSpec>>,
}

#[derive(Debug, Clone)]
pub struct SpreadSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub fragment_name: shared::ast::NameNode,
}

#[derive(Debug, Clone)]
pub struct ConditionalSpreadSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub type_name: shared::ast::NameNode,
    pub fragment: Rc<FragmentSpec>,
}

#[derive(Debug, derive_more::From)]
pub enum SelectionNode {
    FieldSelectionNode(FieldSelectionNode),
    ConditionalSpreadSelectionNode(ConditionalSpreadSelectionNode),
    SpreadSelectionNode(SpreadSelectionNode),
}

#[derive(Debug)]
pub struct FragmentSpec {
    pub location: shared::ast::NodeLocation,
    pub selections: Vec<SelectionNode>,
}

#[derive(Debug, Clone, Copy)]
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
            _ => None,
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
    pub parameters: Vec<shared::ast::InputFieldDefinitionNode>,
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
