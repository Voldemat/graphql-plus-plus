use std::sync::{Arc, RwLock};

use super::errors;
use crate::parsers::{
    file::server::ast::InputObjectDefinitionNode,
    schema::{shared, type_registry::TypeRegistry},
};

pub fn parse_definition<'buffer>(
    input: &InputObjectDefinitionNode<'buffer>,
    registry: &mut TypeRegistry,
) -> Result<Arc<RwLock<shared::ast::InputType>>, errors::Error<'buffer>> {
    let obj_rc = registry.inputs.get(input.name.name).unwrap();
    let mut obj = obj_rc.write().unwrap();
    obj.fields =
        shared::input::parse_field_definitions(&input.fields, registry)?;
    return Ok(obj_rc.clone());
}
