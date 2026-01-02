use crate::parsers::{file, schema::type_registry};

#[derive(Debug, derive_more::From)]
pub enum Error {
    TypeRegistryError(type_registry::Error),
    UnknownFragmentType(file::shared::ast::NameNode),
    UnexpectedConditionalSpreadSelectionNodeOnObjectFragment(file::client::ast::ConditionalSpreadSelectionNode)
}
