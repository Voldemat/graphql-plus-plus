use std::sync::Arc;

use indexmap::IndexMap;

use super::ast;
use crate::parsers::file;

#[derive(Debug)]
pub struct TypeRegistry<S = String> {
    pub fragments: IndexMap<S, ast::Fragment<S>>,
    pub operations: IndexMap<S, ast::Operation<S>>,
    pub directives: IndexMap<S, ast::ClientDirective<S>>,
}

#[derive(Debug)]
pub enum Error<'buffer> {
    UnknownType(file::shared::ast::NameNode<'buffer>),
    UnknownArgument(file::shared::ast::NameNode<'buffer>),
}

impl<'buffer> Error<'buffer> {
    pub fn get_location(
        self: &Self,
    ) -> &file::shared::ast::NodeLocation<'buffer> {
        match self {
            Self::UnknownType(name_node) => &name_node.location,
            Self::UnknownArgument(name_node) => &name_node.location,
        }
    }
    pub fn get_source_file(
        self: &Self,
    ) -> &Arc<file::shared::ast::SourceFile<'buffer>> {
        match self {
            Self::UnknownType(name_node) => &name_node.location.source,
            Self::UnknownArgument(name_node) => &name_node.location.source,
        }
    }
}

impl<S> TypeRegistry<S> {
    pub fn new() -> Self {
        Self {
            fragments: IndexMap::new(),
            operations: IndexMap::new(),
            directives: IndexMap::new(),
        }
    }
}
