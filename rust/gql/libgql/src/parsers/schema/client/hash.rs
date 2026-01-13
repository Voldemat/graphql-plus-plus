use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::parsers::schema::{
    client::ast, server, shared, type_registry::TypeRegistry,
};

pub fn hash_input_type_spec<T: std::hash::Hasher>(
    hasher: &mut T,
    spec: &shared::ast::InputTypeSpec,
) {
    let name = match spec {
        shared::ast::InputTypeSpec::Scalar(s) => s,
        shared::ast::InputTypeSpec::Enum(e) => &e.name,
        shared::ast::InputTypeSpec::InputType(i) => &i.borrow().name,
    };
    std::hash::Hash::hash(name, hasher);
}

pub fn hash_literal<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &shared::ast::Literal,
) {
    match value {
        shared::ast::Literal::Int(i) => std::hash::Hash::hash(i, hasher),
        shared::ast::Literal::Float(f) => {
            std::hash::Hash::hash(&f.to_string(), hasher)
        }
        shared::ast::Literal::String(s) => std::hash::Hash::hash(s, hasher),
        shared::ast::Literal::Boolean(b) => std::hash::Hash::hash(b, hasher),
    }
}

pub fn hash_literal_default_value<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &Option<shared::ast::Literal>,
) {
    match value {
        Some(literal) => hash_literal(hasher, literal),
        None => {}
    }
}

pub fn hash_array_literal<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &shared::ast::ArrayLiteral,
) {
    match value {
        shared::ast::ArrayLiteral::Int(ivec) => {
            std::hash::Hash::hash(ivec, hasher)
        }
        shared::ast::ArrayLiteral::Float(fvec) => std::hash::Hash::hash(
            &fvec.iter().map(|f| f.to_string()).collect::<Vec<_>>(),
            hasher,
        ),
        shared::ast::ArrayLiteral::String(svec) => {
            std::hash::Hash::hash(svec, hasher)
        }
        shared::ast::ArrayLiteral::Boolean(bvec) => {
            std::hash::Hash::hash(bvec, hasher)
        }
    }
}

pub fn hash_array_default_value<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &Option<shared::ast::ArrayLiteral>,
) {
    match value {
        Some(literal) => hash_array_literal(hasher, literal),
        None => {}
    }
}

pub fn hash_input_field_spec<T: std::hash::Hasher>(
    hasher: &mut T,
    spec: &shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
) {
    match spec {
        shared::ast::NonCallableFieldSpec::Literal(literal) => {
            hasher.write_u8(b'l');
            hash_input_type_spec(hasher, &literal.r#type);
            if let Some(default_value) = &literal.default_value {
                hash_literal_default_value(hasher, &default_value);
            }
        }
        shared::ast::NonCallableFieldSpec::Array(array) => {
            hasher.write_u8(b'a');
            std::hash::Hash::hash(&array.nullable, hasher);
            hash_input_type_spec(hasher, &array.r#type);
            if let Some(default_value) = &array.default_value {
                hash_array_default_value(hasher, default_value);
            }
        }
    }
}

pub fn hash_input_field_definition<T: std::hash::Hasher>(
    hasher: &mut T,
    field: &shared::ast::FieldDefinition<
        shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
    >,
) {
    std::hash::Hash::hash(&field.name, hasher);
    std::hash::Hash::hash(&field.nullable, hasher);
    hash_input_field_spec(hasher, &field.spec)
}

pub fn get_operation_parameters_hash(
    parameters: &IndexMap<
        String,
        shared::ast::FieldDefinition<
            shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
        >,
    >,
) -> u64 {
    let mut keys = parameters.keys().collect::<Vec<_>>();
    keys.sort();
    let mut hasher = std::hash::DefaultHasher::new();
    for key in keys {
        hash_input_field_definition(&mut hasher, parameters.get(key).unwrap());
    }
    return std::hash::Hasher::finish(&hasher);
}

fn hash_argument_literal_value<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &shared::ast::ArgumentLiteralValue,
) {
    match value {
        shared::ast::ArgumentLiteralValue::Int(i) => {
            std::hash::Hash::hash(i, hasher);
        }
        shared::ast::ArgumentLiteralValue::Float(f) => {
            std::hash::Hash::hash(&f.to_string(), hasher);
        }
        shared::ast::ArgumentLiteralValue::String(s) => {
            std::hash::Hash::hash(s, hasher);
        }
        shared::ast::ArgumentLiteralValue::Boolean(b) => {
            std::hash::Hash::hash(b, hasher);
        }
        shared::ast::ArgumentLiteralValue::EnumValue(e) => {
            std::hash::Hash::hash(e, hasher);
        }
    }
}

fn hash_argument_value<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &shared::ast::ArgumentValue,
) {
    match value {
        shared::ast::ArgumentValue::Ref(r) => {
            std::hash::Hash::hash(&'r', hasher);
            std::hash::Hash::hash(r, hasher);
        }
        shared::ast::ArgumentValue::Literal(literal) => {
            std::hash::Hash::hash(&'l', hasher);
            hash_argument_literal_value(hasher, literal);
        }
    }
}

fn hash_argument<T: std::hash::Hasher>(
    hasher: &mut T,
    argument: &shared::ast::FieldSelectionArgument,
) {
    std::hash::Hash::hash(&argument.name, hasher);
    hash_argument_value(hasher, &argument.value);
}

fn hash_arguments<T: std::hash::Hasher>(
    hasher: &mut T,
    arguments: &IndexMap<String, shared::ast::FieldSelectionArgument>,
) {
    let mut keys = arguments.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        hash_argument(hasher, arguments.get(key).unwrap());
    }
}

fn hash_object_selection_node<T: std::hash::Hasher>(
    hasher: &mut T,
    registry: &TypeRegistry,
    selection: &ast::ObjectSelection,
    recursive: bool,
) {
    match selection {
        ast::ObjectSelection::TypenameField(field) => {
            std::hash::Hash::hash(&'t', hasher);
            std::hash::Hash::hash(&field.alias, hasher);
        }
        ast::ObjectSelection::SpreadSelection(field) => {
            std::hash::Hash::hash(&'s', hasher);
            std::hash::Hash::hash(&field.fragment.borrow().name, hasher);
            if recursive {
                hash_fragment_spec(
                    hasher,
                    registry,
                    &field.fragment.borrow().spec,
                    true,
                );
            }
        }
        ast::ObjectSelection::FieldSelection(field) => {
            std::hash::Hash::hash(&'f', hasher);
            std::hash::Hash::hash(&field.name, hasher);
            if field.name != field.alias {
                std::hash::Hash::hash(&field.alias, hasher);
            }
            hash_arguments(hasher, &field.arguments);
            if let Some(selection) = field.selection.as_ref() {
                hash_fragment_spec(
                    hasher,
                    registry,
                    selection.as_ref(),
                    recursive,
                )
            }
        }
    }
}

fn hash_object_fragment_spec<T: std::hash::Hasher>(
    hasher: &mut T,
    registry: &TypeRegistry,
    fragment_spec: &ast::ObjectFragmentSpec<server::ast::ObjectType>,
    recursive: bool,
) {
    for selection in fragment_spec.selections.iter() {
        hash_object_selection_node(hasher, registry, selection, recursive);
    }
}

fn hash_interface_fragment_spec<T: std::hash::Hasher>(
    hasher: &mut T,
    registry: &TypeRegistry,
    fragment_spec: &ast::ObjectFragmentSpec<server::ast::Interface>,
    recursive: bool,
) {
    for selection in fragment_spec.selections.iter() {
        hash_object_selection_node(hasher, registry, selection, recursive);
    }
}

fn hash_union_selection_node<T: std::hash::Hasher>(
    hasher: &mut T,
    registry: &TypeRegistry,
    selection: &ast::UnionSelection,
    recursive: bool,
) {
    match selection {
        ast::UnionSelection::TypenameField(field) => {
            std::hash::Hash::hash(&'t', hasher);
            std::hash::Hash::hash(&field.alias, hasher);
        }
        ast::UnionSelection::SpreadSelection(field) => {
            std::hash::Hash::hash(&'s', hasher);
            std::hash::Hash::hash(&field.fragment.borrow().name, hasher);
            if recursive {
                hash_fragment_spec(
                    hasher,
                    registry,
                    &field.fragment.borrow().spec,
                    true,
                );
            }
        }
        ast::UnionSelection::UnionConditionalSpreadSelection(_) => {}
        ast::UnionSelection::ObjectConditionalSpreadSelection(s) => {
            std::hash::Hash::hash("oc", hasher);
            std::hash::Hash::hash(&s.r#type.borrow().name, hasher);
            hash_object_fragment_spec(
                hasher,
                registry,
                &s.selection,
                recursive,
            );
        }
    }
}

fn hash_union_fragment_spec<T: std::hash::Hasher>(
    hasher: &mut T,
    registry: &TypeRegistry,
    fragment_spec: &ast::UnionFragmentSpec,
    recursive: bool,
) {
    for selection in fragment_spec.selections.iter() {
        hash_union_selection_node(hasher, registry, selection, recursive);
    }
}

fn hash_fragment_spec<T: std::hash::Hasher>(
    hasher: &mut T,
    registry: &TypeRegistry,
    fragment_spec: &ast::FragmentSpec,
    recursive: bool,
) {
    match fragment_spec {
        ast::FragmentSpec::Object(object) => {
            hash_object_fragment_spec(hasher, registry, object, recursive);
        }
        ast::FragmentSpec::Interface(interface) => {
            hash_interface_fragment_spec(
                hasher, registry, interface, recursive,
            );
        }
        ast::FragmentSpec::Union(union) => {
            hash_union_fragment_spec(hasher, registry, union, recursive);
        }
    }
}

pub fn get_fragment_spec_hash(
    registry: &TypeRegistry,
    fragment_spec: &ast::FragmentSpec,
    recursive: bool,
) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    hash_fragment_spec(&mut hasher, registry, fragment_spec, recursive);
    return std::hash::Hasher::finish(&hasher);
}

pub fn get_used_fragments_from_object_fragment_spec<T>(
    registry: &TypeRegistry,
    fragment_spec: &ast::ObjectFragmentSpec<T>,
) -> Vec<Rc<RefCell<ast::Fragment>>> {
    fragment_spec
        .selections
        .iter()
        .map(|selection| match selection {
            ast::ObjectSelection::SpreadSelection(s) => {
                vec![s.fragment.clone()]
            }
            ast::ObjectSelection::FieldSelection(f) => {
                f.selection.as_ref().map_or(Vec::new(), |s| {
                    get_used_fragments_from_fragment_spec(registry, &s)
                })
            }
            _ => Vec::new(),
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn get_used_fragments_from_fragment_spec(
    registry: &TypeRegistry,
    fragment_spec: &ast::FragmentSpec,
) -> Vec<Rc<RefCell<ast::Fragment>>> {
    match fragment_spec {
        ast::FragmentSpec::Object(object) => {
            get_used_fragments_from_object_fragment_spec(registry, object)
        }
        ast::FragmentSpec::Interface(interface) => {
            get_used_fragments_from_object_fragment_spec(registry, interface)
        }
        ast::FragmentSpec::Union(union) => union
            .selections
            .iter()
            .map(|selection| match selection {
                ast::UnionSelection::SpreadSelection(s) => {
                    vec![s.fragment.clone()]
                }
                ast::UnionSelection::ObjectConditionalSpreadSelection(c) => {
                    get_used_fragments_from_object_fragment_spec(
                        registry,
                        &c.selection,
                    )
                }
                _ => Vec::new(),
            })
            .flatten()
            .collect::<Vec<_>>(),
    }
}
