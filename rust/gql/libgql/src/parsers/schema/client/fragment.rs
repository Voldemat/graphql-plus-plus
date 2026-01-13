use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server, shared,
        type_registry::{self, TypeRegistry},
    },
};

fn parse_union_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Union>>,
    node: &file::client::ast::SelectionNode,
) -> Result<ast::UnionSelection, errors::Error> {
    match node {
        file::client::ast::SelectionNode::FieldSelectionNode(field) => {
            if is_object_field_spec_is_typename_field(&field.field) {
                return Ok(ast::TypenameField {
                    alias: field.field.get_alias(),
                }
                .into());
            }
            return Err(errors::Error::UnexpectedFieldSelectionNodeOnUnion(
                field.clone(),
            ));
        }
        file::client::ast::SelectionNode::SpreadSelectionNode(spread) => {
            return Ok(parse_union_spread_selection_node(
                registry, r#type, spread,
            )?
            .into());
        }
        file::client::ast::SelectionNode::ConditionalSpreadSelectionNode(c) => {
            let item_type =
                get_type_for_union_conditional_selection(registry, r#type, c)
                    .ok_or_else(|| {
                    errors::Error::NoSuitableTypeForConditionalSpreadSelection {
                        r#type: r#type.clone(),
                        selection: c.clone(),
                    }
                })?;
            match item_type {
                ConditionalSelectionType::Object(object) => {
                    return Ok(ast::ObjectConditionalSpreadSelection {
                        r#type: object.clone(),
                        selection: Rc::new(ast::ObjectFragmentSpec {
                            r#type: object.clone(),
                            selections: parse_object_selections(
                                registry,
                                &object,
                                &c.fragment.selections,
                            )?,
                        }),
                    }
                    .into());
                }
                ConditionalSelectionType::Union(union) => {
                    return Ok(ast::UnionConditionalSpreadSelection {
                        r#type: union.clone(),
                        selection: Rc::new(ast::UnionFragmentSpec {
                            r#type: union.clone(),
                            selections: parse_union_selections(
                                registry,
                                &union,
                                &c.fragment.selections,
                            )?,
                        }),
                    }
                    .into());
                }
            }
        }
    }
}

#[derive(derive_more::From)]
pub enum ConditionalSelectionType {
    Object(Rc<RefCell<server::ast::ObjectType>>),
    Union(Rc<RefCell<server::ast::Union>>),
}

fn get_type_for_union_conditional_selection(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Union>>,
    node: &file::client::ast::ConditionalSpreadSelectionNode,
) -> Option<ConditionalSelectionType> {
    return r#type
        .borrow()
        .items
        .get(&node.type_name.name)
        .map(|object| object.clone().into())
        .or_else(|| {
            registry
                .unions
                .get(&node.type_name.name)
                .filter(|union| {
                    union.borrow().items.keys().all(|object_name| {
                        r#type.borrow().items.contains_key(object_name)
                    })
                })
                .map(|union| union.clone().into())
        });
}

fn parse_union_selections(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Union>>,
    selections: &[file::client::ast::SelectionNode],
) -> Result<Vec<ast::UnionSelection>, errors::Error> {
    return selections
        .iter()
        .map(|selection| {
            parse_union_selection_node(registry, r#type, selection)
        })
        .collect();
}

fn parse_interface_spread_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Interface>>,
    node: &file::client::ast::SpreadSelectionNode,
) -> Result<ast::SpreadSelection, errors::Error> {
    let Some(fragment) = registry.fragments.get(&node.fragment_name.name)
    else {
        return Err(errors::Error::UnknownFragment(node.fragment_name.clone()));
    };
    let has_invalid_type = match &fragment.borrow().spec {
        ast::FragmentSpec::Interface(spec) => !Rc::ptr_eq(&spec.r#type, r#type),
        _ => true,
    };
    if has_invalid_type {
        return Err(errors::Error::InvalidFragmentType {
            selection_node: node.clone(),
            expected_type: r#type.clone().into(),
            fragment: fragment.clone(),
        });
    };
    return Ok(ast::SpreadSelection {
        fragment: fragment.clone(),
    });
}

fn parse_object_spread_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    node: &file::client::ast::SpreadSelectionNode,
) -> Result<ast::SpreadSelection, errors::Error> {
    let Some(fragment) = registry.fragments.get(&node.fragment_name.name)
    else {
        return Err(errors::Error::UnknownFragment(node.fragment_name.clone()));
    };
    let has_invalid_type = match &fragment.borrow().spec {
        ast::FragmentSpec::Object(spec) => !Rc::ptr_eq(&spec.r#type, r#type),
        ast::FragmentSpec::Interface(spec) => r#type
            .borrow()
            .implements
            .contains_key(&spec.r#type.borrow().name),
        _ => true,
    };
    if has_invalid_type {
        return Err(errors::Error::InvalidFragmentType {
            selection_node: node.clone(),
            expected_type: r#type.clone().into(),
            fragment: fragment.clone(),
        });
    };
    return Ok(ast::SpreadSelection {
        fragment: fragment.clone(),
    });
}

fn parse_union_spread_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Union>>,
    node: &file::client::ast::SpreadSelectionNode,
) -> Result<ast::SpreadSelection, errors::Error> {
    let Some(fragment) = registry.fragments.get(&node.fragment_name.name)
    else {
        return Err(errors::Error::UnknownFragment(node.fragment_name.clone()));
    };
    let has_invalid_type = match &fragment.borrow().spec {
        ast::FragmentSpec::Union(spec) => !Rc::ptr_eq(&spec.r#type, r#type),
        ast::FragmentSpec::Interface(spec) => {
            r#type.borrow().items.values().any(|t| {
                !t.borrow()
                    .implements
                    .contains_key(&spec.r#type.borrow().name)
            })
        }
        _ => true,
    };
    if has_invalid_type {
        return Err(errors::Error::InvalidFragmentType {
            selection_node: node.clone(),
            expected_type: r#type.clone().into(),
            fragment: fragment.clone(),
        });
    };
    return Ok(ast::SpreadSelection {
        fragment: fragment.clone(),
    });
}

fn is_object_field_spec_is_typename_field(
    field: &file::client::ast::ObjectFieldSpec,
) -> bool {
    match field {
        file::client::ast::ObjectFieldSpec::Literal(literal) => {
            &literal.name.name == "__typename"
        }
        _ => false,
    }
}

fn fragment_spec_from_field_definition(
    registry: &TypeRegistry,
    field: &Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    spec: &Rc<file::client::ast::FragmentSpec>,
) -> Result<Rc<ast::FragmentSpec>, errors::Error> {
    let type_spec = field.spec.get_return_type();
    match type_spec {
        server::ast::ObjectTypeSpec::ObjectType(object) => Ok(Rc::new(
            ast::ObjectFragmentSpec {
                r#type: object.clone(),
                selections: parse_object_selections(
                    registry,
                    &object,
                    &spec.selections,
                )?,
            }
            .into(),
        )),
        server::ast::ObjectTypeSpec::Interface(interface) => Ok(Rc::new(
            ast::ObjectFragmentSpec {
                r#type: interface.clone(),
                selections: parse_interface_selections(
                    registry,
                    &interface,
                    &spec.selections,
                )?,
            }
            .into(),
        )),
        server::ast::ObjectTypeSpec::Union(union) => Ok(Rc::new(
            ast::UnionFragmentSpec {
                r#type: union.clone(),
                selections: parse_union_selections(
                    registry,
                    &union,
                    &spec.selections,
                )?,
            }
            .into(),
        )),
        _ => Err(errors::Error::UnexpectedSelectionOnLiteralField {
            field: field.clone(),
            spec: spec.clone(),
        }),
    }
}

fn parse_selection_arguments(
    spec: &server::ast::CallableFieldSpec,
    arguments: &[file::shared::ast::Argument],
) -> Result<IndexMap<String, shared::ast::FieldSelectionArgument>, errors::Error>
{
    Ok(arguments
        .iter()
        .map(|arg| parse_selection_argument(spec, arg))
        .collect::<Result<Vec<_>, errors::Error>>()?
        .into_iter()
        .map(|arg| (arg.name.clone(), arg))
        .collect())
}

fn parse_argument_literal_value(
    type_spec: &shared::ast::InputTypeSpec,
    node: &file::shared::ast::LiteralNode,
) -> Result<shared::ast::ArgumentLiteralValue, errors::Error> {
    match node {
        file::shared::ast::LiteralNode::Int(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => s == "Int",
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
                shared::ast::InputTypeSpec::Scalar(s) => s == "Float",
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
                shared::ast::InputTypeSpec::Scalar(s) => s == "Boolean",
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
                shared::ast::InputTypeSpec::Scalar(s) => s == "String",
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::String(i.value.clone()))
            }
        }
        file::shared::ast::LiteralNode::EnumValue(i) => {
            let is_valid = match type_spec {
                shared::ast::InputTypeSpec::Scalar(s) => s == "String",
                _ => false,
            };
            if !is_valid {
                Err(errors::Error::InvalidLiteralForInput {
                    type_spec: type_spec.clone(),
                    node: node.clone(),
                })
            } else {
                Ok(shared::ast::ArgumentLiteralValue::EnumValue(
                    i.value.clone(),
                ))
            }
        }
    }
}

fn parse_selection_argument(
    spec: &server::ast::CallableFieldSpec,
    argument: &file::shared::ast::Argument,
) -> Result<shared::ast::FieldSelectionArgument, errors::Error> {
    let Some(t) = spec.arguments.get(&argument.name.name) else {
        return Err(type_registry::Error::UnknownArgument(
            argument.name.clone(),
        )
        .into());
    };
    let type_spec = t.spec.get_type_spec();
    return Ok(shared::ast::FieldSelectionArgument {
        name: argument.name.name.clone(),
        value: match &argument.value {
            file::shared::ast::ArgumentValue::NameNode(node) => {
                shared::ast::ArgumentValue::Ref(node.name.clone()).into()
            }
            file::shared::ast::ArgumentValue::LiteralNode(node) => {
                parse_argument_literal_value(type_spec, node)?.into()
            }
        },
        r#type: t.clone(),
    });
}

fn parse_object_field_selection_node<T: Clone + Into<errors::FieldType>>(
    registry: &TypeRegistry,
    r#type: &T,
    fields: &IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    >,
    node: &file::client::ast::FieldSelectionNode,
) -> Result<ast::FieldSelection, errors::Error> {
    let field_name = node.field.get_name();
    let field_type =
        fields
            .get(&field_name.name)
            .ok_or(errors::Error::UnknownField {
                r#type: r#type.clone().into(),
                field: field_name.clone(),
            })?;
    return Ok(ast::FieldSelection {
        name: node.field.get_name().name.clone(),
        alias: node.field.get_selection_name().name.clone(),
        selection: node
            .spec
            .as_ref()
            .map(|spec| fragment_spec_from_field_definition(registry, field_type, spec))
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

fn parse_object_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    node: &file::client::ast::SelectionNode,
) -> Result<ast::ObjectSelection, errors::Error> {
    match node {
        file::client::ast::SelectionNode::SpreadSelectionNode(s) => {
            Ok(parse_object_spread_selection_node(registry, r#type, s)?.into())
        }
        file::client::ast::SelectionNode::FieldSelectionNode(f) => {
            if is_object_field_spec_is_typename_field(&f.field) {
                return Ok(ast::TypenameField {
                    alias: f.field.get_alias(),
                }
                .into());
            }
            Ok(parse_object_field_selection_node(
                registry,
                r#type,
                &r#type.borrow().fields,
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

fn parse_interface_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Interface>>,
    node: &file::client::ast::SelectionNode,
) -> Result<ast::ObjectSelection, errors::Error> {
    match node {
        file::client::ast::SelectionNode::SpreadSelectionNode(s) => {
            Ok(parse_interface_spread_selection_node(registry, r#type, s)?
                .into())
        }
        file::client::ast::SelectionNode::FieldSelectionNode(f) => {
            Ok(parse_object_field_selection_node(
                registry,
                r#type,
                &r#type.borrow().fields,
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

fn parse_object_selections(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    selections: &[file::client::ast::SelectionNode],
) -> Result<Vec<ast::ObjectSelection>, errors::Error> {
    return selections
        .iter()
        .map(|s| parse_object_selection_node(registry, r#type, s))
        .collect();
}

fn parse_interface_selections(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Interface>>,
    selections: &[file::client::ast::SelectionNode],
) -> Result<Vec<ast::ObjectSelection>, errors::Error> {
    return selections
        .iter()
        .map(|s| parse_interface_selection_node(registry, r#type, s))
        .collect();
}

pub fn parse_selections(
    registry: &TypeRegistry,
    spec: &mut ast::FragmentSpec,
    selections: &[file::client::ast::SelectionNode],
) -> Result<(), errors::Error> {
    match spec {
        ast::FragmentSpec::Union(u) => {
            u.selections =
                parse_union_selections(registry, &u.r#type, selections)?;
            return Ok(());
        }
        ast::FragmentSpec::Object(o) => {
            o.selections =
                parse_object_selections(registry, &o.r#type, selections)?;
            return Ok(());
        }
        ast::FragmentSpec::Interface(i) => {
            i.selections =
                parse_interface_selections(registry, &i.r#type, selections)?;
            return Ok(());
        }
    }
}

pub fn parse(
    registry: &mut TypeRegistry,
    node: &file::client::ast::FragmentDefinition,
) -> Result<(), errors::Error> {
    let fragment_rc = registry.fragments.get(&node.name.name).unwrap();
    let mut fragment = fragment_rc.borrow_mut();
    parse_selections(registry, &mut fragment.spec, &node.spec.selections)?;
    return Ok(());
}
