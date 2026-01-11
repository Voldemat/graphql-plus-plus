use std::rc::Rc;

use crate::parsers::file::shared;

#[derive(Debug, Clone, Copy, serde::Serialize)]
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

impl std::fmt::Display for DirectiveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into_str())
    }
}

impl DirectiveLocation {
    fn into_str(self: &Self) -> &str {
        match self {
            Self::Query => "QUERY",
            Self::Mutation => "MUTATION",
            Self::Subscription => "SUBSCRIPTION",
            Self::Field => "FIELD",
            Self::FragmentDefinition => "FRAGMENT_DEFINITION",
            Self::FragmentSpread => "FRAGMENT_SPREAD",
            Self::InlineFragment => "INLINE_FRAGMENT",
            Self::VariableDefinition => "VARIABLE_DEFINITION",
        }
    }
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct ObjectLiteralFieldSpec {
    pub location: shared::ast::NodeLocation,
    pub selection_name: shared::ast::NameNode,
    pub name: shared::ast::NameNode,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ObjectCallableFieldSpec {
    pub location: shared::ast::NodeLocation,
    pub selection_name: shared::ast::NameNode,
    pub name: shared::ast::NameNode,
    pub arguments: Vec<shared::ast::Argument>,
}

#[derive(Debug, Clone, derive_more::From, serde::Serialize)]
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

fn serialize_option_rc_fragment_spec<S: serde::Serializer>(
    v: &Option<Rc<FragmentSpec>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    serde::Serialize::serialize(&v.as_ref().map(|a| a.as_ref()), s)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FieldSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub field: ObjectFieldSpec,
    #[serde(serialize_with = "serialize_option_rc_fragment_spec")]
    pub spec: Option<Rc<FragmentSpec>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpreadSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub fragment_name: shared::ast::NameNode,
}

fn serialize_fragment_rc<S: serde::Serializer>(
    v: &Rc<FragmentSpec>,
    s: S,
) -> Result<S::Ok, S::Error> {
    serde::Serialize::serialize(v.as_ref(), s)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ConditionalSpreadSelectionNode {
    pub location: shared::ast::NodeLocation,
    pub type_name: shared::ast::NameNode,
    #[serde(serialize_with = "serialize_fragment_rc")]
    pub fragment: Rc<FragmentSpec>,
}

#[derive(Debug, derive_more::From, serde::Serialize)]
pub enum SelectionNode {
    FieldSelectionNode(FieldSelectionNode),
    ConditionalSpreadSelectionNode(ConditionalSpreadSelectionNode),
    SpreadSelectionNode(SpreadSelectionNode),
}

#[derive(Debug, serde::Serialize)]
pub struct FragmentSpec {
    pub location: shared::ast::NodeLocation,
    pub selections: Vec<SelectionNode>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum OpType {
    Mutation,
    Query,
    Subscription,
}

impl std::fmt::Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into_str())
    }
}

impl OpType {
    pub fn into_str(self: &Self) -> &str {
        match self {
            Self::Query => "QUERY",
            Self::Mutation => "MUTATION",
            Self::Subscription => "SUBSCRIPTION",
        }
    }
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

#[derive(serde::Serialize)]
pub struct OperationDefinition {
    pub location: shared::ast::NodeLocation,
    pub r#type: OpType,
    pub name: shared::ast::NameNode,
    pub parameters: Vec<shared::ast::InputFieldDefinitionNode>,
    pub fragment: FragmentSpec,
}

#[derive(serde::Serialize)]
pub struct FragmentDefinition {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub type_name: shared::ast::NameNode,
    pub spec: FragmentSpec,
}

#[derive(derive_more::From, serde::Serialize)]
pub enum ASTNode {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
    Directive(DirectiveDefinition),
}
