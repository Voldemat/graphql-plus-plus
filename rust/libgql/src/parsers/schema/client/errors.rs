use crate::parsers::{
    file,
    schema::{server, shared},
};

use super::{ast::FragmentSpecTypeTag, type_registry};

#[derive(Debug)]
pub enum FragmentType<S = String> {
    Object(S),
    Interface(S),
    Union(S),
}

impl<S> FragmentType<S> {
    fn to_type_tag(self: &Self) -> FragmentSpecTypeTag {
        match self {
            Self::Object(_) => FragmentSpecTypeTag::Object,
            Self::Interface(_) => FragmentSpecTypeTag::Interface,
            Self::Union(_) => FragmentSpecTypeTag::Union,
        }
    }
}

#[derive(Debug)]
pub enum FieldType<S = String> {
    Object(S),
    Interface(S),
}

#[derive(Debug)]
pub enum Error<'buffer, S: shared::ast::AsStr<'buffer>> {
    TypeRegistryError(type_registry::Error<'buffer>),
    ServerTypeRegistryError(server::type_registry::Error<'buffer>),
    UnknownFragmentType(file::shared::ast::NameNode<'buffer>),
    UnexpectedConditionalSpreadSelectionNode(
        file::client::ast::ConditionalSpreadSelectionNode<'buffer>,
    ),
    UnknownFragment(file::shared::ast::NameNode<'buffer>),
    InvalidFragmentType {
        selection_node: file::client::ast::SpreadSelectionNode<'buffer>,
        selection_fragment_type: FragmentSpecTypeTag,
        expected_type: FragmentType<S>,
        fragment: S,
    },
    UnknownField {
        r#type: FieldType<S>,
        field: file::shared::ast::NameNode<'buffer>,
    },
    UnexpectedCallableField {
        field_type: shared::ast::runtime::FieldDefinition<
            server::ast::ObjectFieldSpec<S>,
            S,
        >,
        definition: file::client::ast::ObjectCallableFieldSpec<'buffer>,
    },
    UnexpectedFieldSelectionNodeOnUnion(
        file::client::ast::FieldSelectionNode<'buffer>,
    ),
    NoSuitableTypeForConditionalSpreadSelection {
        selection: file::client::ast::ConditionalSpreadSelectionNode<'buffer>,
        union_type: S,
    },
    UnexpectedSelectionOnLiteralField {
        spec: file::client::ast::FragmentSpec<'buffer>,
        field: shared::ast::runtime::FieldDefinition<
            server::ast::ObjectFieldSpec<S>,
            S,
        >,
    },
    InvalidLiteralForInput {
        type_spec: shared::ast::runtime::InputTypeSpec<S>,
        node: file::shared::ast::LiteralNode<'buffer>,
    },
    FragmentNameCollision(file::shared::ast::NameNode<'buffer>),
    OperationNameCollision(file::shared::ast::NameNode<'buffer>),
    DirectiveNameCollision(file::shared::ast::NameNode<'buffer>),
}

impl<'buffer, S: shared::ast::AsStr<'buffer>> std::fmt::Display
    for Error<'buffer, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeRegistryError(error) => error.fmt(f),
            Self::ServerTypeRegistryError(error) => error.fmt(f),
            Self::UnknownFragmentType(name_node) => f.write_fmt(format_args!(
                "Unknown fragment type: {}",
                name_node.name
            )),
            Self::FragmentNameCollision(node) => f.write_fmt(format_args!(
                "Fragment with {} name already exists",
                node.name
            )),
            Self::OperationNameCollision(node) => f.write_fmt(format_args!(
                "Operation with {} name already exists",
                node.name
            )),
            Self::DirectiveNameCollision(node) => f.write_fmt(format_args!(
                "Directive with {} name already exists",
                node.name
            )),
            Self::UnexpectedConditionalSpreadSelectionNode(_) => f
                .write_fmt(format_args!(
                    "Unexpected conditional spread selection"
                )),
            Self::UnknownFragment(name_node) => f.write_fmt(format_args!(
                "Unknown fragment: {}",
                name_node.name
            )),
            Self::InvalidFragmentType {
                selection_node,
                selection_fragment_type,
                expected_type,
                ..
            } => f.write_fmt(format_args!(
                "Invalid spread with fragment {} of type {:?}, while expected {:?}",
                selection_node.fragment_name.name,
                selection_fragment_type,
                expected_type.to_type_tag()
            )),
            Self::UnknownField { field, .. } =>
                f.write_fmt(format_args!("Unknown field {}", field.name)),
            Self::UnexpectedCallableField { field_type, .. } =>
                f.write_fmt(format_args!("Unexpected callable field, while field is {}", field_type))
            ,
            Self::UnexpectedFieldSelectionNodeOnUnion(_) =>
                f.write_str("Unexpected field selection on union fragment")
            ,
            Self::NoSuitableTypeForConditionalSpreadSelection {
                ..
            } => f.write_str("No suitable type for conditional spread selection"),
            Self::UnexpectedSelectionOnLiteralField { .. } =>
                f.write_str("Unexpected selection on literal field"),
            Self::InvalidLiteralForInput { type_spec, node } =>
                f.write_fmt(format_args!("Invalid literal {} for input with type {}", node.get_location().get_source_slice(), type_spec)),
        }
    }
}

impl<'buffer, S: shared::ast::AsStr<'buffer>> Error<'buffer, S> {
    pub fn get_location(
        self: &Self,
    ) -> &file::shared::ast::NodeLocation<'buffer> {
        match self {
            Self::TypeRegistryError(error) => error.get_location(),
            Self::ServerTypeRegistryError(error) => error.get_location(),
            Self::UnknownFragmentType(name_node) => &name_node.location,
            Self::FragmentNameCollision(node) => &node.location,
            Self::OperationNameCollision(node) => &node.location,
            Self::DirectiveNameCollision(node) => &node.location,
            Self::UnexpectedConditionalSpreadSelectionNode(node) => {
                &node.location
            }
            Self::UnknownFragment(name_node) => &name_node.location,
            Self::InvalidFragmentType { selection_node, .. } => {
                &selection_node.location
            }
            Self::UnknownField { field, .. } => &field.location,
            Self::UnexpectedCallableField { definition, .. } => {
                &definition.location
            }
            Self::UnexpectedFieldSelectionNodeOnUnion(node) => &node.location,
            Self::NoSuitableTypeForConditionalSpreadSelection {
                selection,
                ..
            } => &selection.location,
            Self::UnexpectedSelectionOnLiteralField { spec, .. } => {
                &spec.location
            }
            Self::InvalidLiteralForInput { node, .. } => node.get_location(),
        }
    }
}

impl<'buffer, S: shared::ast::AsStr<'buffer>>
    From<type_registry::Error<'buffer>> for Error<'buffer, S>
{
    fn from(value: type_registry::Error<'buffer>) -> Self {
        return Self::TypeRegistryError(value);
    }
}

impl<'buffer, S: shared::ast::AsStr<'buffer>>
    From<server::type_registry::Error<'buffer>> for Error<'buffer, S>
{
    fn from(value: server::type_registry::Error<'buffer>) -> Self {
        return Self::ServerTypeRegistryError(value);
    }
}
