use std::{cell::RefCell, sync::Arc};

use super::errors;
use crate::parsers::{
    file::server::ast::InputObjectDefinitionNode,
    schema::{shared, type_registry::TypeRegistry},
};

pub fn parse_definition(
    input: &InputObjectDefinitionNode,
    registry: &mut TypeRegistry,
) -> Result<Arc<RefCell<shared::ast::InputType>>, errors::Error> {
    let obj_rc = registry.inputs.get(&input.name.name).unwrap();
    let mut obj = obj_rc.borrow_mut();
    obj.fields =
        shared::input::parse_field_definitions(&input.fields, registry)?;
    return Ok(obj_rc.clone());
}
