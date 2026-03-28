use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server, shared,
    },
};

use super::type_registry::{self, TypeRegistry};

fn parse_union_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::Union<S>,
    node: &file::client::ast::SelectionNode<'buffer>,
) -> Result<ast::UnionSelection<S>, errors::Error<'buffer, S>> {
    match node {
        file::client::ast::SelectionNode::FieldSelectionNode(field) => {
            if is_object_field_spec_is_typename_field(&field.field) {
                return Ok(ast::TypenameField {
                    alias: field.field.get_alias().map(S::from_str),
                }
                .into());
            }
            return Err(errors::Error::UnexpectedFieldSelectionNodeOnUnion(
                field.clone(),
            ));
        }
        file::client::ast::SelectionNode::SpreadSelectionNode(spread) => {
            return Ok(parse_union_spread_selection_node(
                server_registry,
                registry,
                r#type,
                spread,
            )?
            .into());
        }
        file::client::ast::SelectionNode::ConditionalSpreadSelectionNode(c) => {
            let item_type = get_type_for_union_conditional_selection(
                server_registry,
                r#type,
                c,
            )
            .ok_or_else(|| {
                errors::Error::NoSuitableTypeForConditionalSpreadSelection {
                    union_type: r#type.name.clone(),
                    selection: c.clone(),
                }
            })?;
            match item_type {
                ConditionalSelectionType::Object(object) => {
                    return Ok(ast::ObjectConditionalSpreadSelection {
                        r#type: object.clone(),
                        selections: parse_object_selections(
                            server_registry,
                            registry,
                            server_registry
                                .get_object(object.to_str())
                                .unwrap(),
                            &c.fragment.selections,
                        )?,
                    }
                    .into());
                }
                ConditionalSelectionType::Union(union) => {
                    return Ok(ast::UnionConditionalSpreadSelection {
                        r#type: union.clone(),
                        selection: parse_union_selections(
                            server_registry,
                            registry,
                            server_registry.get_union(union.to_str()).unwrap(),
                            &c.fragment.selections,
                        )?,
                    }
                    .into());
                }
            }
        }
    }
}

pub enum ConditionalSelectionType<S = String> {
    Object(S),
    Union(S),
}

fn get_type_for_union_conditional_selection<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    registry: &T,
    r#type: &server::ast::Union<S>,
    node: &file::client::ast::ConditionalSpreadSelectionNode,
) -> Option<ConditionalSelectionType<S>> {
    return r#type
        .items
        .get(node.type_name.name)
        .map(|object| ConditionalSelectionType::Object(object.clone()))
        .or_else(|| {
            registry
                .get_union(node.type_name.name)
                .filter(|union| {
                    union
                        .items
                        .iter()
                        .all(|object_name| r#type.items.contains(object_name))
                })
                .map(|union| {
                    ConditionalSelectionType::Union(union.name.clone())
                })
        });
}

fn parse_union_selections<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::Union<S>,
    selections: &[file::client::ast::SelectionNode<'buffer>],
) -> Result<Vec<ast::UnionSelection<S>>, errors::Error<'buffer, S>> {
    return selections
        .iter()
        .map(|selection| {
            parse_union_selection_node(
                server_registry,
                registry,
                r#type,
                selection,
            )
        })
        .collect();
}

fn parse_interface_spread_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
>(
    registry: &TypeRegistry<S>,
    r#type: &server::ast::Interface<S>,
    node: &file::client::ast::SpreadSelectionNode<'buffer>,
) -> Result<ast::SpreadSelection<S>, errors::Error<'buffer, S>> {
    let Some(fragment) = registry.fragments.get(node.fragment_name.name) else {
        return Err(errors::Error::UnknownFragment(node.fragment_name.clone()));
    };
    let has_invalid_type = match &fragment.spec {
        ast::FragmentSpec::Interface(spec) => spec.r#type == r#type.name,
        _ => true,
    };
    if has_invalid_type {
        return Err(errors::Error::InvalidFragmentType {
            selection_node: node.clone(),
            expected_type: errors::FragmentType::Interface(r#type.name.clone()),
            fragment: fragment.name.clone(),
        });
    };
    return Ok(ast::SpreadSelection {
        fragment: fragment.name.clone(),
    });
}

fn parse_object_spread_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
>(
    registry: &TypeRegistry<S>,
    r#type: &server::ast::ObjectType<S>,
    node: &file::client::ast::SpreadSelectionNode<'buffer>,
) -> Result<ast::SpreadSelection<S>, errors::Error<'buffer, S>> {
    let Some(fragment) = registry.fragments.get(node.fragment_name.name) else {
        return Err(errors::Error::UnknownFragment(node.fragment_name.clone()));
    };
    let has_invalid_type = match &fragment.spec {
        ast::FragmentSpec::Object(spec) => spec.r#type != r#type.name,
        ast::FragmentSpec::Interface(spec) => {
            r#type.implements.contains(&spec.r#type)
        }
        _ => true,
    };
    if has_invalid_type {
        return Err(errors::Error::InvalidFragmentType {
            selection_node: node.clone(),
            expected_type: errors::FragmentType::Object(r#type.name.clone()),
            fragment: fragment.name.clone(),
        });
    };
    return Ok(ast::SpreadSelection {
        fragment: fragment.name.clone(),
    });
}

fn parse_union_spread_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::Union<S>,
    node: &file::client::ast::SpreadSelectionNode<'buffer>,
) -> Result<ast::SpreadSelection<S>, errors::Error<'buffer, S>> {
    let Some(fragment) = registry.fragments.get(node.fragment_name.name) else {
        return Err(errors::Error::UnknownFragment(node.fragment_name.clone()));
    };
    let has_invalid_type = match &fragment.spec {
        ast::FragmentSpec::Union(spec) => spec.r#type != r#type.name,
        ast::FragmentSpec::Interface(spec) => r#type.items.iter().any(|t| {
            !server_registry
                .get_object(t.to_str())
                .unwrap()
                .implements
                .contains(&spec.r#type)
        }),
        _ => true,
    };
    if has_invalid_type {
        return Err(errors::Error::InvalidFragmentType {
            selection_node: node.clone(),
            expected_type: errors::FragmentType::Union(r#type.name.clone()),
            fragment: fragment.name.clone(),
        });
    };
    return Ok(ast::SpreadSelection {
        fragment: fragment.name.clone(),
    });
}

fn is_object_field_spec_is_typename_field(
    field: &file::client::ast::ObjectFieldSpec,
) -> bool {
    match field {
        file::client::ast::ObjectFieldSpec::Literal(literal) => {
            literal.name.name == "__typename"
        }
        _ => false,
    }
}

fn fragment_spec_from_field_definition<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    field: &shared::ast::FieldDefinition<server::ast::ObjectFieldSpec<S>, S>,
    spec: &file::client::ast::FragmentSpec<'buffer>,
) -> Result<ast::FragmentSpec<S>, errors::Error<'buffer, S>> {
    let type_spec = field.spec.get_return_type();
    match type_spec {
        server::ast::ObjectTypeSpec::ObjectType(object) => {
            Ok(ast::ObjectFragmentSpec {
                r#type: object.clone(),
                selections: parse_object_selections(
                    server_registry,
                    registry,
                    server_registry.get_object(object.to_str()).unwrap(),
                    &spec.selections,
                )?,
            }
            .into())
        }
        server::ast::ObjectTypeSpec::Interface(interface) => {
            Ok(ast::ObjectFragmentSpec {
                r#type: interface.clone(),
                selections: parse_interface_selections(
                    server_registry,
                    registry,
                    server_registry.get_interface(interface.to_str()).unwrap(),
                    &spec.selections,
                )?,
            }
            .into())
        }
        server::ast::ObjectTypeSpec::Union(union) => {
            Ok(ast::UnionFragmentSpec {
                r#type: union.clone(),
                selections: parse_union_selections(
                    server_registry,
                    registry,
                    server_registry.get_union(union.to_str()).unwrap(),
                    &spec.selections,
                )?,
            }
            .into())
        }
        _ => Err(errors::Error::UnexpectedSelectionOnLiteralField {
            field: field.clone(),
            spec: spec.clone(),
        }),
    }
}

fn parse_selection_arguments<'buffer, S: shared::ast::AsStr<'buffer>>(
    spec: &server::ast::CallableFieldSpec<S>,
    arguments: &[file::shared::ast::Argument<'buffer>],
) -> Result<
    IndexMap<S, shared::ast::FieldSelectionArgument<S>>,
    errors::Error<'buffer, S>,
> {
    Ok(arguments
        .iter()
        .map(|arg| parse_selection_argument(spec, arg))
        .collect::<Result<Vec<_>, errors::Error<'buffer, S>>>()?
        .into_iter()
        .map(|arg| (arg.name.clone(), arg))
        .collect())
}

fn parse_argument_literal_value<'buffer, S: shared::ast::AsStr<'buffer>>(
    type_spec: &shared::ast::InputTypeSpec<S>,
    node: &file::shared::ast::LiteralNode<'buffer>,
) -> Result<shared::ast::ArgumentLiteralValue<S>, errors::Error<'buffer, S>> {
    match node {
        file::shared::ast::LiteralNode::Int(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => s.to_str() == "Int",
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::Int(i.value))
            }
        }
        file::shared::ast::LiteralNode::Float(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => s.to_str() == "Float",
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::Float(i.value))
            }
        }
        file::shared::ast::LiteralNode::Boolean(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => {
                    s.to_str() == "Boolean"
                }
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::Boolean(i.value))
            }
        }
        file::shared::ast::LiteralNode::String(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => s.to_str() == "String",
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::String(S::from_str(
                    i.value,
                )))
            }
        }
        file::shared::ast::LiteralNode::EnumValue(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => s.to_str() == "String",
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::EnumValue(S::from_str(
                    i.value,
                )))
            }
        }
    }
}

fn parse_selection_argument<'buffer, S: shared::ast::AsStr<'buffer>>(
    spec: &server::ast::CallableFieldSpec<S>,
    argument: &file::shared::ast::Argument<'buffer>,
) -> Result<shared::ast::FieldSelectionArgument<S>, errors::Error<'buffer, S>> {
    let Some(t) = spec.arguments.get(argument.name.name) else {
        return Err(type_registry::Error::UnknownArgument(
            argument.name.clone(),
        )
        .into());
    };
    let type_spec = t.spec.get_type_spec();
    return Ok(shared::ast::FieldSelectionArgument {
        name: S::from_str(argument.name.name),
        value: match &argument.value {
            file::shared::ast::ArgumentValue::NameNode(node) => {
                shared::ast::ArgumentValue::Ref(S::from_str(node.name)).into()
            }
            file::shared::ast::ArgumentValue::LiteralNode(node) => {
                parse_argument_literal_value(type_spec, node)?.into()
            }
        },
        r#type: t.clone(),
    });
}

fn parse_object_field_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: errors::FieldType<S>,
    fields: &IndexMap<
        S,
        shared::ast::FieldDefinition<server::ast::ObjectFieldSpec<S>, S>,
    >,
    node: &file::client::ast::FieldSelectionNode<'buffer>,
) -> Result<ast::FieldSelection<S>, errors::Error<'buffer, S>> {
    let field_name = node.field.get_name();
    let field_type =
        fields
            .get(field_name.name)
            .ok_or(errors::Error::UnknownField {
                r#type: r#type,
                field: field_name.clone(),
            })?;
    return Ok(ast::FieldSelection {
        name: S::from_str(node.field.get_name().name),
        alias: S::from_str(node.field.get_selection_name().name),
        selection: node
            .spec
            .as_ref()
            .map(|spec| {
                fragment_spec_from_field_definition(
                    server_registry,
                    registry,
                    field_type,
                    spec,
                )
            })
            .transpose()?,
        arguments: match &node.field {
            file::client::ast::ObjectFieldSpec::Literal(_) => {
                Ok(IndexMap::new())
            }
            file::client::ast::ObjectFieldSpec::Callable(callable) => {
                let spec = match &field_type.spec {
                    server::ast::ObjectFieldSpec::Literal(_)
                    | server::ast::ObjectFieldSpec::Array(_) => {
                        Err(errors::Error::UnexpectedCallableField {
                            field_type: field_type.clone(),
                            definition: callable.clone(),
                        })
                    }
                    server::ast::ObjectFieldSpec::Callable(c) => Ok(c),
                }?;
                parse_selection_arguments(&spec, &callable.arguments)
            }
        }?,
    });
}

fn parse_object_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::ObjectType<S>,
    node: &file::client::ast::SelectionNode<'buffer>,
) -> Result<ast::ObjectSelection<S>, errors::Error<'buffer, S>> {
    match node {
        file::client::ast::SelectionNode::SpreadSelectionNode(s) => {
            Ok(parse_object_spread_selection_node(registry, r#type, s)?.into())
        }
        file::client::ast::SelectionNode::FieldSelectionNode(f) => {
            if is_object_field_spec_is_typename_field(&f.field) {
                return Ok(ast::TypenameField {
                    alias: f.field.get_alias().map(S::from_str),
                }
                .into());
            }
            Ok(parse_object_field_selection_node(
                server_registry,
                registry,
                errors::FieldType::Object(r#type.name.clone()),
                &r#type.fields,
                f,
            )?
            .into())
        }
        file::client::ast::SelectionNode::ConditionalSpreadSelectionNode(s) => {
            Err(errors::Error::UnexpectedConditionalSpreadSelectionNode(
                s.clone(),
            ))
        }
    }
}

fn parse_interface_selection_node<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::Interface<S>,
    node: &file::client::ast::SelectionNode<'buffer>,
) -> Result<ast::ObjectSelection<S>, errors::Error<'buffer, S>> {
    match node {
        file::client::ast::SelectionNode::SpreadSelectionNode(s) => {
            Ok(parse_interface_spread_selection_node(registry, r#type, s)?
                .into())
        }
        file::client::ast::SelectionNode::FieldSelectionNode(f) => {
            Ok(parse_object_field_selection_node(
                server_registry,
                registry,
                errors::FieldType::Interface(r#type.name.clone()),
                &r#type.fields,
                f,
            )?
            .into())
        }
        file::client::ast::SelectionNode::ConditionalSpreadSelectionNode(s) => {
            Err(errors::Error::UnexpectedConditionalSpreadSelectionNode(
                s.clone(),
            ))
        }
    }
}

fn parse_object_selections<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::ObjectType<S>,
    selections: &[file::client::ast::SelectionNode<'buffer>],
) -> Result<Vec<ast::ObjectSelection<S>>, errors::Error<'buffer, S>> {
    return selections
        .iter()
        .map(|s| {
            parse_object_selection_node(server_registry, registry, r#type, s)
        })
        .collect();
}

fn parse_interface_selections<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    r#type: &server::ast::Interface<S>,
    selections: &[file::client::ast::SelectionNode<'buffer>],
) -> Result<Vec<ast::ObjectSelection<S>>, errors::Error<'buffer, S>> {
    return selections
        .iter()
        .map(|s| {
            parse_interface_selection_node(server_registry, registry, r#type, s)
        })
        .collect();
}

pub fn parse_selections<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &TypeRegistry<S>,
    spec: &mut ast::FragmentSpec<S>,
    selections: &[file::client::ast::SelectionNode<'buffer>],
) -> Result<(), errors::Error<'buffer, S>> {
    match spec {
        ast::FragmentSpec::Union(u) => {
            u.selections = parse_union_selections(
                server_registry,
                registry,
                server_registry.get_union(u.r#type.to_str()).unwrap(),
                selections,
            )?;
            return Ok(());
        }
        ast::FragmentSpec::Object(o) => {
            o.selections = parse_object_selections(
                server_registry,
                registry,
                server_registry.get_object(o.r#type.to_str()).unwrap(),
                selections,
            )?;
            return Ok(());
        }
        ast::FragmentSpec::Interface(i) => {
            i.selections = parse_interface_selections(
                server_registry,
                registry,
                server_registry.get_interface(i.r#type.to_str()).unwrap(),
                selections,
            )?;
            return Ok(());
        }
    }
}

pub fn parse<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &mut TypeRegistry<S>,
    node: &file::client::ast::FragmentDefinition<'buffer>,
) -> Result<(), errors::Error<'buffer, S>> {
    let mut fragment = registry.fragments.swap_remove(node.name.name).unwrap();
    parse_selections(
        server_registry,
        registry,
        &mut fragment.spec,
        &node.spec.selections,
    )?;
    registry
        .fragments
        .insert(S::from_str(node.name.name), fragment);
    return Ok(());
}
