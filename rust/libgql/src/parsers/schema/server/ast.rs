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

impl<'s1, S: shared::ast::AsStr<'s1>> ObjectTypeSpec<S> {
    pub fn clone_with_string_type<'s2, NS: shared::ast::AsStr<'s2>>(
        self: &'s1 Self,
    ) -> ObjectTypeSpec<NS>
    where
        's1: 's2,
    {
        match self {
            Self::ObjectType(s) => {
                ObjectTypeSpec::ObjectType(NS::from_str(s.to_str()))
            }
            Self::Scalar(s) => ObjectTypeSpec::Scalar(NS::from_str(s.to_str())),
            Self::Enum(s) => ObjectTypeSpec::Enum(NS::from_str(s.to_str())),
            Self::Union(u) => ObjectTypeSpec::Union(NS::from_str(u.to_str())),
            Self::Interface(i) => {
                ObjectTypeSpec::Interface(NS::from_str(i.to_str()))
            }
        }
    }
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
impl<'s1, S: shared::ast::AsStr<'s1>> CallableFieldSpec<S> {
    pub fn clone_with_string_type<'s2, NS: shared::ast::AsStr<'s2>>(
        self: &'s1 Self,
    ) -> CallableFieldSpec<NS>
    where
        's1: 's2,
    {
        CallableFieldSpec {
            return_type: self.return_type.clone_with_string_type(ObjectTypeSpec::clone_with_string_type),
            arguments: self.arguments.iter().map(|(key, argument)| {
                (NS::from_str(key.to_str()), argument.clone_with_string_type(
                    |s| s.clone_with_string_type(shared::ast::InputTypeSpec::clone_with_string_type)
                ))
            }).collect()
        }
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ObjectFieldSpec<S = String> {
    Literal(shared::ast::LiteralFieldSpec<ObjectTypeSpec<S>>),
    Array(shared::ast::ArrayFieldSpec<ObjectTypeSpec<S>>),
    Callable(CallableFieldSpec<S>),
}

impl<'s1, S: shared::ast::AsStr<'s1>> ObjectFieldSpec<S> {
    pub fn clone_with_string_type<'s2, NS: shared::ast::AsStr<'s2>>(
        self: &'s1 Self,
    ) -> ObjectFieldSpec<NS>
    where
        's1: 's2,
    {
        match self {
            Self::Literal(l) => {
                ObjectFieldSpec::Literal(l.clone_with_string_type(
                    ObjectTypeSpec::clone_with_string_type,
                ))
            }
            Self::Array(a) => ObjectFieldSpec::Array(a.clone_with_string_type(
                ObjectTypeSpec::clone_with_string_type,
            )),
            Self::Callable(c) => {
                ObjectFieldSpec::Callable(c.clone_with_string_type())
            }
        }
    }

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
