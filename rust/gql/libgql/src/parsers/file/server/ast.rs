use crate::parsers::file::shared;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum DirectiveLocation {
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

impl TryFrom<&str> for DirectiveLocation {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SCHEMA" => Ok(DirectiveLocation::Schema),
            "SCALAR" => Ok(DirectiveLocation::Scalar),
            "OBJECT" => Ok(DirectiveLocation::Object),
            "FIELD_DEFINITION" => Ok(DirectiveLocation::FieldDefinition),
            "ARGUMENT_DEFINITION" => Ok(DirectiveLocation::ArgumentDefinition),
            "INTERFACE" => Ok(DirectiveLocation::Interface),
            "UNION" => Ok(DirectiveLocation::Union),
            "ENUM" => Ok(DirectiveLocation::Enum),
            "ENUM_VALUE" => Ok(DirectiveLocation::EnumValue),
            "INPUT_OBJECT" => Ok(DirectiveLocation::InputObject),
            "INPUT_FIELD_DEFINITION" => {
                Ok(DirectiveLocation::InputFieldDefinition)
            }
            _ => Err(())
        }
    }
}

pub type DirectiveLocationNode =
    shared::ast::DirectiveLocationNode<DirectiveLocation>;
pub type DirectiveDefinitionNode =
    shared::ast::DirectiveNode<DirectiveLocation>;

#[derive(Debug, serde::Serialize)]
pub struct FieldDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub r#type: shared::ast::TypeNode,
    pub arguments: Vec<shared::ast::InputFieldDefinitionNode>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct InterfaceDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub fields: Vec<FieldDefinitionNode>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct ObjectDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub interfaces: Vec<shared::ast::NameNode>,
    pub fields: Vec<FieldDefinitionNode>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct InputObjectDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub fields: Vec<shared::ast::InputFieldDefinitionNode>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct EnumValueDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub value: shared::ast::NameNode,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct EnumDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub values: Vec<EnumValueDefinitionNode>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct UnionDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
    pub values: Vec<shared::ast::NameNode>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode>,
}

#[derive(Debug, serde::Serialize)]
pub struct ScalarDefinitionNode {
    pub location: shared::ast::NodeLocation,
    pub name: shared::ast::NameNode,
}

#[derive(Debug, derive_more::From, serde::Serialize)]
pub enum TypeDefinitionNode {
    Scalar(ScalarDefinitionNode),
    Union(UnionDefinitionNode),
    Enum(EnumDefinitionNode),
    Input(InputObjectDefinitionNode),
    Object(ObjectDefinitionNode),
    Interface(InterfaceDefinitionNode),
    Directive(DirectiveDefinitionNode),
}

#[derive(serde::Serialize)]
pub struct ExtendTypeNode {
    pub location: shared::ast::NodeLocation,
    pub type_node: ObjectDefinitionNode,
}

#[derive(derive_more::From, serde::Serialize)]
pub enum ASTNode {
    TypeDefinitionNode(TypeDefinitionNode),
    ExtendTypeNode(ExtendTypeNode),
}

impl From<ScalarDefinitionNode> for ASTNode {
    fn from(value: ScalarDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl From<UnionDefinitionNode> for ASTNode {
    fn from(value: UnionDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl From<EnumDefinitionNode> for ASTNode {
    fn from(value: EnumDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl From<InputObjectDefinitionNode> for ASTNode {
    fn from(value: InputObjectDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl From<ObjectDefinitionNode> for ASTNode {
    fn from(value: ObjectDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl From<InterfaceDefinitionNode> for ASTNode {
    fn from(value: InterfaceDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl From<DirectiveDefinitionNode> for ASTNode {
    fn from(value: DirectiveDefinitionNode) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}
