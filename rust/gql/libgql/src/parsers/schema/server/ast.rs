use std::{cell::RefCell, rc::Rc};

use crate::parsers::schema::shared;

#[derive(derive_more::From)]
pub enum ObjectTypeSpec {
    ObjectType(Rc<RefCell<ObjectType>>),
    Interface(Rc<RefCell<Interface>>),
    Scalar(Rc<RefCell<shared::ast::Scalar>>),
    Enum(Rc<RefCell<shared::ast::Enum>>),
    Union(Rc<RefCell<Union>>),
}

pub struct Union {
    pub name: String,
    pub items: indexmap::IndexMap<String, Rc<RefCell<ObjectType>>>,
}

pub struct CallableFieldSpec {
    pub return_type: shared::ast::NonCallableFieldSpec<ObjectTypeSpec>,
    pub arguments: indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
}

#[derive(derive_more::From)]
pub enum ObjectFieldSpec {
    Literal(shared::ast::LiteralFieldSpec<ObjectTypeSpec>),
    Array(shared::ast::ArrayFieldSpec<ObjectTypeSpec>),
    Callable(CallableFieldSpec),
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

pub struct Interface {
    pub name: String,
    pub fields: indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<ObjectFieldSpec>>,
    >,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation>,
}

#[derive(Clone)]
pub struct ObjectType {
    pub name: String,
    pub fields: indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<ObjectFieldSpec>>,
    >,
    pub implements: indexmap::IndexMap<String, Rc<RefCell<Interface>>>,
    pub directives: Vec<shared::ast::ServerDirectiveInvocation>,
}

pub struct ExtendObjectType {
    pub r#type: ObjectType,
}

#[derive(derive_more::From)]
pub enum ServerSchemaNode {
    ObjectType(Rc<RefCell<ObjectType>>),
    Interface(Rc<RefCell<Interface>>),
    Scalar(Rc<RefCell<shared::ast::Scalar>>),
    Union(Rc<RefCell<Union>>),
    Enum(Rc<RefCell<shared::ast::Enum>>),
    InputType(Rc<RefCell<shared::ast::InputType>>),
    ServerDirective(Rc<RefCell<shared::ast::ServerDirective>>),
}
