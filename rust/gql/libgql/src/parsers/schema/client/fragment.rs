use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server, shared,
        type_registry::TypeRegistry,
    },
};

fn parse_union_selections(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Union>>,
    selections: &[file::client::ast::SelectionNode],
) -> Result<Vec<ast::UnionSelection>, errors::Error> {
    todo!();
}

fn parse_object_spread_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    node: &file::client::ast::SpreadSelectionNode
) -> Result<ast::SpreadSelection, errors::Error> {
    todo!();
}

fn parse_object_field_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    node: &file::client::ast::FieldSelectionNode
) -> Result<ast::FieldSelection, errors::Error> {
    todo!();
}

fn parse_object_selection_node(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    node: &file::client::ast::SelectionNode
) -> Result<ast::ObjectSelection, errors::Error> {
    match node {
    file::client::ast::SelectionNode::SpreadSelectionNode(s) =>
        Ok(parse_object_spread_selection_node(registry, r#type, s)?.into()),
    file::client::ast::SelectionNode::FieldSelectionNode(f) =>
        Ok(parse_object_field_selection_node(registry, r#type, f)?.into()),
    file::client::ast::SelectionNode::ConditionalSpreadSelectionNode(s) =>
        Err(errors::Error::UnexpectedConditionalSpreadSelectionNodeOnObjectFragment (
            s.clone()
        ))
    }
}

fn parse_object_selections(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::ObjectType>>,
    selections: &[file::client::ast::SelectionNode],
) -> Result<Vec<ast::ObjectSelection>, errors::Error> {
    todo!();
}

fn parse_interface_selections(
    registry: &TypeRegistry,
    r#type: &Rc<RefCell<server::ast::Interface>>,
    selections: &[file::client::ast::SelectionNode],
) -> Result<Vec<ast::ObjectSelection>, errors::Error> {
    todo!();
}

fn parse_selections(
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
    return Ok(())
}
