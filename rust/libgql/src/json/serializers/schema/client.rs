use indexmap::IndexMap;
use struson::writer::simple::ValueWriter;

use crate::parsers::schema::{client, shared};

fn write_typename_field<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    field: &client::ast::TypenameField,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "TypenameField")?;
    match &field.alias {
        Some(alias) => writer.write_string_member("alias", alias)?,
        None => writer.write_null_member("alias")?,
    }
    Ok(())
}

fn write_spread_selection<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    field: &client::ast::SpreadSelection,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "SpreadSelection")?;
    writer.write_string_member("fragment", &field.fragment)?;
    Ok(())
}

fn write_union_selection<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    selection: &client::ast::UnionSelection,
) -> Result<(), Box<dyn std::error::Error>> {
    match selection {
        client::ast::UnionSelection::TypenameField(field) => {
            write_typename_field(writer, field)?;
        }
        client::ast::UnionSelection::SpreadSelection(spread) => {
            write_spread_selection(writer, spread)?;
        }
        client::ast::UnionSelection::UnionConditionalSpreadSelection(
            spread,
        ) => {
            writer.write_string_member(
                "_type",
                "UnionConditionalSpreadSelection",
            )?;
            writer.write_string_member("union", &spread.r#type)?;
            writer.write_array_member("selections", |selections_writer| {
                for n_selection in &spread.selection {
                    selections_writer.write_object(|selection_writer| {
                        write_union_selection(selection_writer, n_selection)
                    })?;
                }
                Ok(())
            })?;
        }
        client::ast::UnionSelection::ObjectConditionalSpreadSelection(
            spread,
        ) => {
            writer.write_string_member(
                "_type",
                "ObjectConditionalSpreadSelection",
            )?;
            writer.write_string_member("object", &spread.r#type)?;
            writer.write_object_member("spec", |spec_writer| {
                spec_writer
                    .write_string_member("_type", "ObjectFragmentSpec")?;
                spec_writer.write_string_member("name", &spread.r#type)?;
                spec_writer.write_array_member(
                    "selections",
                    |selections_writer| {
                        for selection in &spread.selections {
                            selections_writer.write_object(
                                |selection_writer| {
                                    write_object_selection(
                                        selection_writer,
                                        selection,
                                    )
                                },
                            )?;
                        }
                        Ok(())
                    },
                )?;
                Ok(())
            })?;
        }
    }
    Ok(())
}

fn write_union_fragment_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &client::ast::UnionFragmentSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "UnionFragmentSpec")?;
    writer.write_string_member("name", &spec.r#type)?;
    writer.write_array_member("selections", |selections_writer| {
        for selection in &spec.selections {
            selections_writer.write_object(|selection_writer| {
                write_union_selection(selection_writer, selection)
            })?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_object_selection<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    selection: &client::ast::ObjectSelection,
) -> Result<(), Box<dyn std::error::Error>> {
    match selection {
        client::ast::ObjectSelection::TypenameField(field) => {
            write_typename_field(writer, field)?;
        }
        client::ast::ObjectSelection::SpreadSelection(spread) => {
            write_spread_selection(writer, spread)?;
        }
        client::ast::ObjectSelection::FieldSelection(field) => {
            writer.write_string_member("_type", "FieldSelection")?;
            writer.write_string_member("name", &field.name)?;
            writer.write_string_member("alias", &field.alias)?;
            writer.write_object_member("arguments", |arguments_writer| {
                super::shared::write_field_selection_arguments(
                    arguments_writer,
                    &field.arguments,
                )
            })?;
            match &field.selection {
                Some(local_selection) => {
                    writer.write_object_member(
                        "selection",
                        |selection_writer| {
                            write_fragment_spec(
                                selection_writer,
                                &local_selection,
                            )
                        },
                    )?;
                }
                None => {
                    writer.write_null_member("selection")?;
                }
            }
        }
    }
    Ok(())
}

fn write_object_fragment_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &client::ast::ObjectFragmentSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "ObjectFragmentSpec")?;
    writer.write_string_member("name", &spec.r#type)?;
    writer.write_array_member("selections", |selections_writer| {
        for selection in &spec.selections {
            selections_writer.write_object(|selection_writer| {
                write_object_selection(selection_writer, selection)
            })?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_interface_fragment_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &client::ast::InterfaceFragmentSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "ObjectFragmentSpec")?;
    writer.write_string_member("name", &spec.r#type)?;
    writer.write_array_member("selections", |selections_writer| {
        for selection in &spec.selections {
            selections_writer.write_object(|selection_writer| {
                write_object_selection(selection_writer, selection)
            })?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_fragment_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &client::ast::FragmentSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match spec {
        client::ast::FragmentSpec::Union(union) => {
            write_union_fragment_spec(writer, union)
        }
        client::ast::FragmentSpec::Object(object) => {
            write_object_fragment_spec(writer, object)
        }
        client::ast::FragmentSpec::Interface(interface) => {
            write_interface_fragment_spec(writer, interface)
        }
    }
}

fn write_fragment<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    fragment: &client::ast::Fragment,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("sourceText", &fragment.source_text)?;
    writer.write_object_member("spec", |spec_writer| {
        write_fragment_spec(spec_writer, &fragment.spec)
    })
}

fn write_fragments<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    fragments: &IndexMap<String, client::ast::Fragment>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_fragments = fragments.clone();
    new_fragments.sort_keys();
    for (name, fragment) in &new_fragments {
        writer.write_object_member(name, |fragment_writer| {
            write_fragment(fragment_writer, &fragment)
        })?
    }
    return Ok(());
}

fn write_operation_parameters<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    parameters: &IndexMap<
        String,
        shared::ast::runtime::FieldDefinition<
            shared::ast::runtime::InputFieldSpec,
        >,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_parameters = parameters.clone();
    new_parameters.sort_keys();
    for (name, parameter) in &new_parameters {
        writer.write_object_member(name, |param_writer| {
            super::shared::write_input_field_definition(param_writer, parameter)
        })?;
    }
    Ok(())
}

fn write_operation<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    operation: &client::ast::Operation,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &operation.name)?;
    writer.write_string_member("type", &operation.r#type.to_string())?;
    writer.write_object_member("parameters", |parameters_writer| {
        write_operation_parameters(parameters_writer, &operation.parameters)
    })?;
    writer.write_object_member("fragmentSpec", |spec_writer| {
        write_fragment_spec(spec_writer, &operation.fragment_spec)
    })?;
    writer.write_string_member("sourceText", &operation.source_text)?;
    writer.write_number_member("parametersHash", operation.parameters_hash)?;
    writer.write_number_member(
        "fragmentSpecHash",
        operation.fragment_spec_hash,
    )?;
    Ok(())
}

fn write_operations<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    operations: &IndexMap<String, client::ast::Operation>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_operations = operations.clone();
    new_operations.sort_keys();
    for (name, operation) in &new_operations {
        writer.write_object_member(name, |operation_writer| {
            write_operation(operation_writer, operation)
        })?
    }
    return Ok(());
}

fn write_directive<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    directive: &client::ast::ClientDirective,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &directive.name)?;
    writer.write_object_member("arguments", |arguments_writer| {
        let mut new_arguments = directive.arguments.clone();
        new_arguments.sort_keys();
        for (name, argument) in &new_arguments {
            arguments_writer.write_object_member(name, |arg_writer| {
                super::shared::write_input_field_definition(
                    arg_writer, argument,
                )
            })?;
        }
        Ok(())
    })?;
    writer.write_array_member("locations", |locations_writer| {
        for location in &directive.locations {
            locations_writer.write_string(&location.to_string())?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_directives<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    directives: &IndexMap<String, client::ast::ClientDirective>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_directives = directives.clone();
    new_directives.sort_keys();
    for (name, directive) in &new_directives {
        writer.write_object_member(name, |directive_writer| {
            write_directive(directive_writer, &directive)
        })?
    }
    return Ok(());
}

pub fn serialize_client_schema(
    schema: &client::type_registry::TypeRegistry,
    pretty: bool,
) -> Result<String, String> {
    let mut io_writer = Vec::<u8>::new();
    let json_writer = struson::writer::JsonStreamWriter::new_custom(
        &mut io_writer,
        struson::writer::WriterSettings {
            pretty_print: pretty,
            escape_all_control_chars: false,
            escape_all_non_ascii: false,
            multi_top_level_value_separator: None,
        },
    );
    struson::writer::simple::ValueWriter::write_object(
        struson::writer::simple::SimpleJsonWriter::from_json_writer(
            json_writer,
        ),
        |schema_writer| {
            schema_writer.write_object_member(
                "fragments",
                |fragments_writer| {
                    write_fragments(fragments_writer, &schema.fragments)
                },
            )?;
            schema_writer.write_object_member(
                "operations",
                |operations_writer| {
                    write_operations(operations_writer, &schema.operations)
                },
            )?;
            schema_writer.write_object_member(
                "directives",
                |directive_writer| {
                    write_directives(directive_writer, &schema.directives)
                },
            )?;
            Ok(())
        },
    )
    .map_err(|e| e.to_string())?;
    return Ok(String::from_utf8(io_writer).unwrap());
}
