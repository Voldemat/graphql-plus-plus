use indexmap::IndexSet;

use crate::parsers::schema::shared;

#[derive(Debug, Clone)]
pub enum ObjectTypeSpec<S = String> {
    ObjectType(S),
    Interface(S),
    Scalar(S),
    Enum(S),
    Union(S),
}

#[derive(Debug, Clone)]
pub struct Union<S = String> {
    pub name: S,
    pub items: IndexSet<S>,
}

#[derive(Debug, Clone)]
pub struct CallableFieldSpec<S = String> {
    pub return_type: shared::ast::NonCallableFieldSpec<ObjectTypeSpec<S>>,
    pub arguments: indexmap::IndexMap<
        S,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec<S>, S>,
    >,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ObjectFieldSpec<S = String> {
    Literal(shared::ast::LiteralFieldSpec<ObjectTypeSpec<S>>),
    Array(shared::ast::ArrayFieldSpec<ObjectTypeSpec<S>>),
    Callable(CallableFieldSpec<S>),
}

impl<S> ObjectFieldSpec<S> {
    pub fn get_return_type(self: &Self) -> &ObjectTypeSpec<S> {
        match self {
            Self::Literal(literal) => &literal.r#type,
            Self::Array(array) => &array.r#type.get_type_spec(),
            Self::Callable(callable) => callable.return_type.get_type_spec(),
        }
    }
}

impl From<shared::ast::NonCallableFieldSpec<ObjectTypeSpec<String>>>
    for ObjectFieldSpec
{
    fn from(
        value: shared::ast::NonCallableFieldSpec<ObjectTypeSpec<String>>,
    ) -> Self {
        match value {
            shared::ast::NonCallableFieldSpec::Array(a) => a.into(),
            shared::ast::NonCallableFieldSpec::Literal(b) => b.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interface<S = String> {
    pub name: S,
    pub fields: indexmap::IndexMap<
        S,
        shared::ast::FieldDefinition<ObjectFieldSpec<S>, S>,
    >,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation<S>>,
}

#[derive(Debug, Clone)]
pub struct ObjectType<S = String> {
    pub name: S,
    pub fields: indexmap::IndexMap<
        S,
        shared::ast::FieldDefinition<ObjectFieldSpec<S>, S>,
    >,
    pub implements: IndexSet<S>,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation<S>>,
}

pub struct ExtendObjectType<S = String> {
    pub r#type: ObjectType<S>,
}
