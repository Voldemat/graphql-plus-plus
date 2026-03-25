use indexmap::IndexSet;

use crate::parsers::schema::shared;


#[derive(Debug, Clone)]
pub enum ObjectTypeSpec {
    ObjectType(String),
    Interface(String),
    Scalar(String),
    Enum(String),
    Union(String),
}

#[derive(Debug, Clone)]
pub struct Union {
    pub name: String,
    pub items: IndexSet<String>,
}

#[derive(Debug, Clone)]
pub struct CallableFieldSpec {
    pub return_type: shared::ast::NonCallableFieldSpec<ObjectTypeSpec>,
    pub arguments: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ObjectFieldSpec {
    Literal(shared::ast::LiteralFieldSpec<ObjectTypeSpec>),
    Array(shared::ast::ArrayFieldSpec<ObjectTypeSpec>),
    Callable(CallableFieldSpec),
}

impl ObjectFieldSpec {
    pub fn get_return_type(self: &Self) -> &ObjectTypeSpec {
        match self {
            Self::Literal(literal) => &literal.r#type,
            Self::Array(array) => &array.r#type.get_type_spec(),
            Self::Callable(callable) => callable.return_type.get_type_spec(),
        }
    }
}

impl From<shared::ast::NonCallableFieldSpec<ObjectTypeSpec>>
    for ObjectFieldSpec
{
    fn from(value: shared::ast::NonCallableFieldSpec<ObjectTypeSpec>) -> Self {
        match value {
            shared::ast::NonCallableFieldSpec::Array(a) => a.into(),
            shared::ast::NonCallableFieldSpec::Literal(b) => b.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub fields: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<ObjectFieldSpec>,
    >,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation>,
}

#[derive(Debug, Clone)]
pub struct ObjectType {
    pub name: String,
    pub fields: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<ObjectFieldSpec>,
    >,
    pub implements: IndexSet<String>,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation>,
}

pub struct ExtendObjectType {
    pub r#type: ObjectType,
}
