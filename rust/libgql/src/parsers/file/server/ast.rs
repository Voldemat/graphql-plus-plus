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

impl std::fmt::Display for DirectiveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into_str())
    }
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
            _ => Err(()),
        }
    }
}

impl DirectiveLocation {
    pub fn into_str(self: &Self) -> &'static str {
        match self {
            Self::Schema => "SCHEMA",
            Self::Scalar => "SCALAR",
            Self::Object => "OBJECT",
            Self::FieldDefinition => "FIELD_DEFINITION",
            Self::ArgumentDefinition => "ARGUMENT_DEFINITION",
            Self::Interface => "INTERFACE",
            Self::Union => "UNION",
            Self::Enum => "ENUM",
            Self::EnumValue => "ENUM_VALUE",
            Self::InputObject => "INPUT_OBJECT",
            Self::InputFieldDefinition => "INPUT_FIELD_DEFINITION",
        }
    }
}

pub type DirectiveLocationNode<'buffer> =
    shared::ast::DirectiveLocationNode<'buffer, DirectiveLocation>;
pub type DirectiveDefinitionNode<'buffer> =
    shared::ast::DirectiveNode<'buffer, DirectiveLocation>;

#[derive(Debug, serde::Serialize)]
pub struct FieldDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
    pub r#type: shared::ast::TypeNode<'buffer>,
    pub arguments: Vec<shared::ast::InputFieldDefinitionNode<'buffer>>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct InterfaceDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
    pub fields: Vec<FieldDefinitionNode<'buffer>>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct ObjectDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
    pub interfaces: Vec<shared::ast::NameNode<'buffer>>,
    pub fields: Vec<FieldDefinitionNode<'buffer>>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct InputObjectDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
    pub fields: Vec<shared::ast::InputFieldDefinitionNode<'buffer>>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct EnumValueDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub value: shared::ast::NameNode<'buffer>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct EnumDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
    pub values: Vec<EnumValueDefinitionNode<'buffer>>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct UnionDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
    pub values: Vec<shared::ast::NameNode<'buffer>>,
    pub directives: Vec<shared::ast::DirectiveInvocationNode<'buffer>>,
}

#[derive(Debug, serde::Serialize)]
pub struct ScalarDefinitionNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub name: shared::ast::NameNode<'buffer>,
}

#[derive(Debug, derive_more::From, serde::Serialize)]
pub enum TypeDefinitionNode<'buffer> {
    Scalar(ScalarDefinitionNode<'buffer>),
    Union(UnionDefinitionNode<'buffer>),
    Enum(EnumDefinitionNode<'buffer>),
    Input(InputObjectDefinitionNode<'buffer>),
    Object(ObjectDefinitionNode<'buffer>),
    Interface(InterfaceDefinitionNode<'buffer>),
    Directive(DirectiveDefinitionNode<'buffer>),
}

#[derive(Debug, serde::Serialize)]
pub struct ExtendTypeNode<'buffer> {
    pub location: shared::ast::NodeLocation<'buffer>,
    pub type_node: ObjectDefinitionNode<'buffer>,
}

#[derive(Debug, derive_more::From, serde::Serialize)]
pub enum ASTNode<'buffer> {
    TypeDefinitionNode(TypeDefinitionNode<'buffer>),
    ExtendTypeNode(ExtendTypeNode<'buffer>),
}

impl<'buffer> From<ScalarDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: ScalarDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl<'buffer> From<UnionDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: UnionDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl<'buffer> From<EnumDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: EnumDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl<'buffer> From<InputObjectDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: InputObjectDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl<'buffer> From<ObjectDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: ObjectDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl<'buffer> From<InterfaceDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: InterfaceDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}

impl<'buffer> From<DirectiveDefinitionNode<'buffer>> for ASTNode<'buffer> {
    fn from(value: DirectiveDefinitionNode<'buffer>) -> Self {
        return Self::TypeDefinitionNode(value.into());
    }
}
