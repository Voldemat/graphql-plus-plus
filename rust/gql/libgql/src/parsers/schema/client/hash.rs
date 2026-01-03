use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::parsers::schema::{client::ast, shared, type_registry::TypeRegistry};

pub fn get_operation_parameters_hash(
    registry: &TypeRegistry,
    parameters: &IndexMap<String, shared::ast::FieldDefinition<shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>>>
    ) -> u64 {
    return 0;
}

pub fn get_fragment_spec_hash(registry: &TypeRegistry,
                                fragment_spec: &ast::FragmentSpec,
                                recursive: bool) -> u64 {
    return 0;
}


pub fn get_used_fragments_from_fragment_spec(
    registry: &TypeRegistry,
    fragment_spec: &ast::FragmentSpec
) -> Vec<Rc<RefCell<ast::Fragment>>> {
    return Vec::new();
}
