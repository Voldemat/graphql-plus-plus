use std::sync::{Arc, RwLock};

use crate::parsers::{
    file,
    schema::{
        server::{ast, errors},
        type_registry::TypeRegistry,
    },
};

pub fn parse_definition<'buffer>(
    node: &file::server::ast::UnionDefinitionNode<'buffer>,
    registry: &TypeRegistry,
) -> Result<Arc<RwLock<ast::Union>>, errors::Error<'buffer>> {
    let obj_rc = registry.unions.get(node.name.name).unwrap();
    let mut obj = obj_rc.write().unwrap();
    for item in node.values.iter() {
        let Some(object) = registry.objects.get(item.name) else {
            return Err(errors::Error::UnknownObject(item.clone()));
        };
        obj.items.insert(item.name.to_string(), object.clone());
    }
    return Ok(obj_rc.clone());
}
