use crate::parsers::{file, schema::server::errors};

use super::type_registry::HashMapTypeRegistry;

pub fn parse_definition<'buffer>(
    node: &file::server::ast::UnionDefinitionNode<'buffer>,
    registry: &mut HashMapTypeRegistry,
) -> Result<(), errors::Error<'buffer>> {
    let obj = registry.unions.get_mut(node.name.name).unwrap();
    for item in node.values.iter() {
        if let None = registry.objects.get(item.name) {
            return Err(errors::Error::UnknownObject(item.clone()));
        };
        obj.items.insert(item.name.to_string());
    }
    return Ok(());
}
