use std::sync::{Arc, RwLock};

use crate::parsers::schema::shared;

#[derive(derive_more::From)]
pub enum ObjectTypeSpec {
    ObjectType(Arc<RwLock<ObjectType>>),
    Interface(Arc<RwLock<Interface>>),
    Scalar { name: String },
    Enum(Arc<shared::ast::Enum>),
    Union(Arc<RwLock<Union>>),
}

impl std::fmt::Debug for ObjectTypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ObjectType(arg0) => f
                .debug_tuple("ObjectType")
                .field(&arg0.read().unwrap().name)
                .finish(),
            Self::Interface(arg0) => f
                .debug_tuple("Interface")
                .field(&arg0.read().unwrap().name)
                .finish(),
            Self::Scalar { name } => {
                f.debug_struct("Scalar").field("name", name).finish()
            }
            Self::Enum(arg0) => {
                f.debug_tuple("Enum").field(&arg0.name).finish()
            }
            Self::Union(arg0) => f
                .debug_tuple("Union")
                .field(&arg0.read().unwrap().name)
                .finish(),
        }
    }
}

#[derive(Debug)]
pub struct Union {
    pub name: String,
    pub items: indexmap::IndexMap<String, Arc<RwLock<ObjectType>>>,
}

#[derive(Debug)]
pub struct CallableFieldSpec {
    pub return_type: shared::ast::NonCallableFieldSpec<ObjectTypeSpec>,
    pub arguments: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
}

#[derive(Debug, derive_more::From)]
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
        Arc<shared::ast::FieldDefinition<ObjectFieldSpec>>,
    >,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation>,
}

#[derive(Debug, Clone)]
pub struct ObjectType {
    pub name: String,
    pub fields: indexmap::IndexMap<
        String,
        Arc<shared::ast::FieldDefinition<ObjectFieldSpec>>,
    >,
    pub implements: indexmap::IndexMap<String, Arc<RwLock<Interface>>>,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation>,
}

pub struct ExtendObjectType {
    pub r#type: ObjectType,
}

#[derive(derive_more::From)]
pub enum ServerSchemaNode {
    ObjectType(Arc<RwLock<ObjectType>>),
    Interface(Arc<RwLock<Interface>>),
    Scalar(String),
    Union(Arc<RwLock<Union>>),
    Enum(Arc<shared::ast::Enum>),
    InputType(Arc<RwLock<shared::ast::InputType>>),
    ServerDirective(Arc<RwLock<shared::ast::ServerDirective>>),
}
