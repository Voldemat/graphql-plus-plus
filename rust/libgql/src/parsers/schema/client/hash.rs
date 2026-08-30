use indexmap::IndexMap;

use crate::parsers::schema::{client::ast, shared};

use super::type_registry::TypeRegistry;

pub fn hash_input_type_spec<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    spec: &shared::ast::runtime::InputTypeSpec<S>,
) {
    let name = match spec {
        shared::ast::runtime::InputTypeSpec::Scalar(s) => s,
        shared::ast::runtime::InputTypeSpec::Enum(e) => e,
        shared::ast::runtime::InputTypeSpec::InputType(i) => i,
    }
    .to_str();
    std::hash::Hash::hash(name, hasher);
}

pub fn hash_literal<'s, S: shared::ast::AsStr<'s>, T: std::hash::Hasher>(
    hasher: &mut T,
    value: &shared::ast::runtime::Literal<S>,
) {
    match value {
        shared::ast::runtime::Literal::Null => {
            std::hash::Hash::hash("null", hasher);
        }
        shared::ast::runtime::Literal::Int(i) => {
            std::hash::Hash::hash(i, hasher)
        }
        shared::ast::runtime::Literal::Float(f) => {
            std::hash::Hash::hash(&f.to_string(), hasher)
        }
        shared::ast::runtime::Literal::String(s) => {
            std::hash::Hash::hash(s, hasher)
        }
        shared::ast::runtime::Literal::EnumValue(s) => {
            std::hash::Hash::hash("e", hasher);
            std::hash::Hash::hash(s, hasher)
        }
        shared::ast::runtime::Literal::Boolean(b) => {
            std::hash::Hash::hash(b, hasher)
        }
    }
}

pub fn hash_array_literal<T: std::hash::Hasher>(
    hasher: &mut T,
    value: &shared::ast::runtime::ArrayLiteral,
) {
    match value {
        shared::ast::runtime::ArrayLiteral::Int(ivec) => {
            std::hash::Hash::hash(ivec, hasher)
        }
        shared::ast::runtime::ArrayLiteral::Float(fvec) => {
            std::hash::Hash::hash(
                &fvec.iter().map(|f| f.to_string()).collect::<Vec<_>>(),
                hasher,
            )
        }
        shared::ast::runtime::ArrayLiteral::String(svec) => {
            std::hash::Hash::hash(svec, hasher)
        }
        shared::ast::runtime::ArrayLiteral::Boolean(bvec) => {
            std::hash::Hash::hash(bvec, hasher)
        }
    }
}

pub fn hash_input_field_spec<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    spec: &shared::ast::runtime::NonCallableFieldSpec<
        shared::ast::runtime::InputTypeSpec<S>,
        S,
    >,
) {
    match spec {
        shared::ast::runtime::NonCallableFieldSpec::Literal(literal) => {
            hasher.write_u8(b'l');
            hash_input_type_spec(hasher, &literal.r#type);
            if let Some(default_value) = &literal.default_value {
                hash_literal(hasher, default_value);
            }
        }
        shared::ast::runtime::NonCallableFieldSpec::Array(array) => {
            hasher.write_u8(b'a');
            std::hash::Hash::hash(&array.nullable, hasher);
            hash_input_field_spec(hasher, &array.r#type);
            if let Some(default_value) = &array.default_value {
                hash_array_literal(hasher, default_value);
            }
        }
    }
}

pub fn hash_input_field_definition<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    field: &shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::NonCallableFieldSpec<
            shared::ast::runtime::InputTypeSpec<S>,
            S,
        >,
        S,
    >,
) {
    std::hash::Hash::hash(&field.name, hasher);
    std::hash::Hash::hash(&field.nullable, hasher);
    hash_input_field_spec(hasher, &field.spec)
}

pub fn get_operation_parameters_hash<'s, S: shared::ast::AsStr<'s>>(
    parameters: &IndexMap<
        S,
        shared::ast::runtime::FieldDefinition<
            shared::ast::runtime::NonCallableFieldSpec<
                shared::ast::runtime::InputTypeSpec<S>,
                S,
            >,
            S,
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

fn hash_argument_value<'s, T: std::hash::Hasher, S: shared::ast::AsStr<'s>>(
    hasher: &mut T,
    value: &shared::ast::runtime::ArgumentValue<S>,
) {
    match value {
        shared::ast::runtime::ArgumentValue::Ref(r) => {
            std::hash::Hash::hash(&'r', hasher);
            std::hash::Hash::hash(r, hasher);
        }
        shared::ast::runtime::ArgumentValue::Literal(literal) => {
            std::hash::Hash::hash(&'l', hasher);
            hash_literal(hasher, literal);
        }
    }
}

fn hash_argument<'s, T: std::hash::Hasher, S: shared::ast::AsStr<'s>>(
    hasher: &mut T,
    argument: &shared::ast::runtime::FieldSelectionArgument<S>,
) {
    std::hash::Hash::hash(&argument.name, hasher);
    hash_argument_value(hasher, &argument.value);
}

fn hash_arguments<'s, T: std::hash::Hasher, S: shared::ast::AsStr<'s>>(
    hasher: &mut T,
    arguments: &IndexMap<S, shared::ast::runtime::FieldSelectionArgument<S>>,
) {
    let mut keys = arguments.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys {
        hash_argument(hasher, arguments.get(key).unwrap());
    }
}

fn hash_object_selection_node<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    registry: &TypeRegistry<S>,
    selection: &ast::ObjectSelection<S>,
    recursive: bool,
) {
    match selection {
        ast::ObjectSelection::TypenameField(field) => {
            std::hash::Hash::hash(&'t', hasher);
            std::hash::Hash::hash(&field.alias, hasher);
        }
        ast::ObjectSelection::SpreadSelection(field) => {
            std::hash::Hash::hash(&'s', hasher);
            std::hash::Hash::hash(&field.fragment, hasher);
            if recursive {
                hash_fragment_spec(
                    hasher,
                    registry,
                    &registry.fragments.get(&field.fragment).unwrap().spec,
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
                hash_fragment_spec(hasher, registry, selection, recursive)
            }
        }
    }
}

fn hash_object_fragment_spec<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::ObjectFragmentSpec<S>,
    recursive: bool,
) {
    for selection in fragment_spec.selections.iter() {
        hash_object_selection_node(hasher, registry, selection, recursive);
    }
}

fn hash_interface_fragment_spec<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::InterfaceFragmentSpec<S>,
    recursive: bool,
) {
    for selection in fragment_spec.selections.iter() {
        hash_object_selection_node(hasher, registry, selection, recursive);
    }
}

fn hash_union_selection_node<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    registry: &TypeRegistry<S>,
    selection: &ast::UnionSelection<S>,
    recursive: bool,
) {
    match selection {
        ast::UnionSelection::TypenameField(field) => {
            std::hash::Hash::hash(&'t', hasher);
            std::hash::Hash::hash(&field.alias, hasher);
        }
        ast::UnionSelection::SpreadSelection(field) => {
            std::hash::Hash::hash(&'s', hasher);
            std::hash::Hash::hash(&field.fragment, hasher);
            if recursive {
                hash_fragment_spec(
                    hasher,
                    registry,
                    &registry.fragments.get(&field.fragment).unwrap().spec,
                    true,
                );
            }
        }
        ast::UnionSelection::UnionConditionalSpreadSelection(_) => {}
        ast::UnionSelection::ObjectConditionalSpreadSelection(s) => {
            std::hash::Hash::hash("oc", hasher);
            std::hash::Hash::hash(&s.r#type, hasher);
            for selection in s.selections.iter() {
                hash_object_selection_node(
                    hasher, registry, selection, recursive,
                );
            }
        }
    }
}

fn hash_union_fragment_spec<
    's,
    T: std::hash::Hasher,
    S: shared::ast::AsStr<'s>,
>(
    hasher: &mut T,
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::UnionFragmentSpec<S>,
    recursive: bool,
) {
    for selection in fragment_spec.selections.iter() {
        hash_union_selection_node(hasher, registry, selection, recursive);
    }
}

fn hash_fragment_spec<'s, T: std::hash::Hasher, S: shared::ast::AsStr<'s>>(
    hasher: &mut T,
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::FragmentSpec<S>,
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

pub fn get_fragment_spec_hash<'s, S: shared::ast::AsStr<'s>>(
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::FragmentSpec<S>,
    recursive: bool,
) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    hash_fragment_spec(&mut hasher, registry, fragment_spec, recursive);
    return std::hash::Hasher::finish(&hasher);
}

pub fn get_used_fragments_from_object_fragment_spec<
    's,
    S: shared::ast::AsStr<'s>,
>(
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::ObjectFragmentSpec<S>,
) -> Vec<S> {
    fragment_spec
        .selections
        .iter()
        .map(|selection| {
            get_used_fragments_from_object_selection(registry, selection)
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn get_used_fragments_from_object_selection<
    's,
    S: shared::ast::AsStr<'s>,
>(
    registry: &TypeRegistry<S>,
    selection: &ast::ObjectSelection<S>,
) -> Vec<S> {
    match selection {
        ast::ObjectSelection::SpreadSelection(s) => {
            vec![s.fragment.clone()]
        }
        ast::ObjectSelection::FieldSelection(f) => {
            f.selection.as_ref().map_or(Vec::new(), |s| {
                get_used_fragments_from_fragment_spec(registry, &s)
            })
        }
        _ => Vec::new(),
    }
}

pub fn get_used_fragments_from_interface_fragment_spec<
    's,
    S: shared::ast::AsStr<'s>,
>(
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::InterfaceFragmentSpec<S>,
) -> Vec<S> {
    fragment_spec
        .selections
        .iter()
        .map(|selection| {
            get_used_fragments_from_object_selection(registry, selection)
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn get_used_fragments_from_fragment_spec<'s, S: shared::ast::AsStr<'s>>(
    registry: &TypeRegistry<S>,
    fragment_spec: &ast::FragmentSpec<S>,
) -> Vec<S> {
    match fragment_spec {
        ast::FragmentSpec::Object(object) => {
            get_used_fragments_from_object_fragment_spec(registry, object)
        }
        ast::FragmentSpec::Interface(interface) => {
            get_used_fragments_from_interface_fragment_spec(registry, interface)
        }
        ast::FragmentSpec::Union(union) => union
            .selections
            .iter()
            .map(|selection| match selection {
                ast::UnionSelection::SpreadSelection(s) => {
                    vec![s.fragment.clone()]
                }
                ast::UnionSelection::ObjectConditionalSpreadSelection(c) => c
                    .selections
                    .iter()
                    .map(|selection| {
                        get_used_fragments_from_object_selection(
                            registry, selection,
                        )
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .flatten()
            .collect::<Vec<_>>(),
    }
}
