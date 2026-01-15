use std::{cell::RefCell, collections::HashSet, rc::Rc};

use indexmap::IndexMap;
use struson::writer::simple::ValueWriter;

use crate::parsers::schema::{
    client::{self, schema::ServerUsesMap},
    server, shared,
};

fn write_object_type_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    t: &server::ast::ObjectTypeSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match t {
        server::ast::ObjectTypeSpec::Scalar { name } => {
            writer.write_string_member("_type", "Scalar")?;
            writer.write_string_member("name", &name)?;
        }
        server::ast::ObjectTypeSpec::ObjectType(o) => {
            let name = &o.borrow().name;
            writer.write_string_member("_type", "ObjectType")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/objects/{}", name),
            )?;
        }
        server::ast::ObjectTypeSpec::Interface(i) => {
            let name = &i.borrow().name;
            writer.write_string_member("_type", "InterfaceType")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/interfaces/{}", name),
            )?;
        }
        server::ast::ObjectTypeSpec::Union(u) => {
            let name = &u.borrow().name;
            writer.write_string_member("_type", "Union")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/unions/{}", name),
            )?;
        }
        server::ast::ObjectTypeSpec::Enum(e) => {
            let name = &e.name;
            writer.write_string_member("_type", "Enum")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/enums/{}", name),
            )?;
        }
    }
    Ok(())
}

fn write_argument_literal_value<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    value: &shared::ast::ArgumentLiteralValue,
) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        shared::ast::ArgumentLiteralValue::Int(i) => {
            writer.write_number_member("value", *i)?;
        }
        shared::ast::ArgumentLiteralValue::Boolean(b) => {
            writer.write_bool_member("value", *b)?;
        }
        shared::ast::ArgumentLiteralValue::Float(f) => {
            writer.write_fp_number_member("value", *f)?;
        }
        shared::ast::ArgumentLiteralValue::String(s) => {
            writer.write_string_member("value", s)?;
        }
        shared::ast::ArgumentLiteralValue::EnumValue(e) => {
            writer.write_string_member("value", e)?;
        }
    }
    Ok(())
}

fn write_argument_value<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    value: &shared::ast::ArgumentValue,
) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        shared::ast::ArgumentValue::Ref(r) => {
            writer.write_string_member("_type", "ref")?;
            writer.write_string_member("name", r)?;
        }
        shared::ast::ArgumentValue::Literal(literal) => {
            writer.write_string_member("_type", "literal")?;
            write_argument_literal_value(writer, literal)?;
        }
    }
    Ok(())
}

fn write_field_selection_arguments<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    arguments: &IndexMap<String, shared::ast::FieldSelectionArgument>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_arguments = arguments.clone();
    new_arguments.sort_keys();
    for (name, arg) in &new_arguments {
        writer.write_object_member(name, |arg_writer| {
            arg_writer.write_string_member("name", name)?;
            arg_writer.write_object_member("value", |value_writer| {
                write_argument_value(value_writer, &arg.value)
            })
        })?;
    }
    Ok(())
}

fn write_literal<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    value: &Option<shared::ast::Literal>,
) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        Some(shared::ast::Literal::Int(i)) => {
            writer.write_number_member("defaultValue", *i)?
        }
        Some(shared::ast::Literal::Float(f)) => {
            writer.write_fp_number_member("defaultValue", *f)?
        }
        Some(shared::ast::Literal::Boolean(b)) => {
            writer.write_bool_member("defaultValue", *b)?
        }
        Some(shared::ast::Literal::String(s)) => {
            writer.write_string_member("defaultValue", s)?
        }
        None => writer.write_null_member("defaultValue")?,
    }
    Ok(())
}

fn write_literal_object_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::LiteralFieldSpec<server::ast::ObjectTypeSpec>,
    skip_invocations: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "literal")?;
    writer.write_object_member("type", |type_writer| {
        write_object_type_spec(type_writer, &spec.r#type)
    })?;
    if let Some(default_value) = &spec.default_value {
        write_literal(writer, default_value)?;
    } else {
        if !skip_invocations {
            writer.write_object_member(
                "invocations",
                |invocations_writer| {
                    let mut new_directive_invocations =
                        spec.directive_invocations.clone();
                    new_directive_invocations.sort_keys();
                    for (name, invocation) in &new_directive_invocations {
                        invocations_writer.write_object_member(
                            &name,
                            |invocation_writer| {
                                invocation_writer.write_object_member(
                                    "arguments",
                                    |arguments_writer| {
                                        write_field_selection_arguments(
                                            arguments_writer,
                                            &invocation.arguments,
                                        )
                                    },
                                )
                            },
                        )?;
                    }
                    Ok(())
                },
            )?;
        }
    }
    Ok(())
}

fn write_array_literal<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    value: &Option<shared::ast::ArrayLiteral>,
) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        Some(arr_value) => {
            writer.write_array_member(
                "defaultValue",
                |array_writer| -> Result<(), Box<dyn std::error::Error>> {
                    match arr_value {
                        shared::ast::ArrayLiteral::Int(i) => Ok(i
                            .iter()
                            .try_for_each(|v| array_writer.write_number(*v))?),

                        shared::ast::ArrayLiteral::Float(f) => {
                            Ok(f.iter().try_for_each(|v| {
                                array_writer.write_fp_number(*v)
                            })?)
                        }
                        shared::ast::ArrayLiteral::Boolean(b) => {
                            Ok(b.iter().try_for_each(|v| {
                                array_writer.write_bool(*v)
                            })?)
                        }
                        shared::ast::ArrayLiteral::String(s) => Ok(s
                            .iter()
                            .try_for_each(|v| array_writer.write_string(v))?),
                    }
                },
            )?;
        }
        None => writer.write_null_member("defaultValue")?,
    };
    Ok(())
}

fn write_array_object_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::ArrayFieldSpec<server::ast::ObjectTypeSpec>,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "array")?;
    writer.write_bool_member("nullable", spec.nullable)?;
    writer.write_object_member("type", |type_writer| {
        write_object_type_spec(type_writer, &spec.r#type)
    })?;
    if let Some(default_value) = &spec.default_value {
        write_array_literal(writer, &default_value)?;
    }
    Ok(())
}

fn write_non_callable_object_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::NonCallableFieldSpec<server::ast::ObjectTypeSpec>,
) -> Result<(), Box<dyn std::error::Error>> {
    match spec {
        shared::ast::NonCallableFieldSpec::Literal(literal) => {
            write_literal_object_field_spec(writer, literal, true)
        }
        shared::ast::NonCallableFieldSpec::Array(array) => {
            write_array_object_field_spec(writer, array)
        }
    }
}

fn write_object_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &server::ast::ObjectFieldSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match spec {
        server::ast::ObjectFieldSpec::Literal(literal) => {
            write_literal_object_field_spec(writer, literal, false)?;
        }
        server::ast::ObjectFieldSpec::Array(array) => {
            write_array_object_field_spec(writer, array)?;
        }
        server::ast::ObjectFieldSpec::Callable(callable) => {
            writer.write_string_member("_type", "callable")?;
            writer.write_object_member("returnType", |r_writer| {
                write_non_callable_object_field_spec(
                    r_writer,
                    &callable.return_type,
                )
            })?;
            writer.write_object_member("arguments", |arguments_writer| {
                let mut arguments = callable.arguments.clone();
                arguments.sort_keys();
                for (name, arg) in &arguments {
                    arguments_writer.write_object_member(
                        name,
                        |arg_writer| {
                            write_input_field_definition(arg_writer, arg)
                        },
                    )?;
                }
                Ok(())
            })?;
        }
    }
    Ok(())
}

fn write_object_field_definition<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    field_definition: &shared::ast::FieldDefinition<
        server::ast::ObjectFieldSpec,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_bool_member("nullable", field_definition.nullable)?;
    writer.write_object_member("spec", |spec_writer| {
        write_object_field_spec(spec_writer, &field_definition.spec)
    })?;
    Ok(())
}

fn write_object<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    object: &server::ast::ObjectType,
    fields_map: Option<HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &object.name)?;
    writer.write_object_member("implements", |implements_writer| {
        let mut implements = object.implements.clone();
        implements.sort_keys();
        for interface_name in implements.keys() {
            implements_writer.write_object_member(
                interface_name,
                |i_writer| {
                    i_writer.write_string_member("name", interface_name)?;
                    i_writer.write_string_member(
                        "$ref",
                        &format!("#/server/interfaces/{}", interface_name),
                    )?;
                    Ok(())
                },
            )?;
        }
        Ok(())
    })?;
    writer.write_object_member("fields", |fields_writer| {
        let mut fields = object.fields.clone();
        fields.sort_keys();
        for field in fields.values() {
            if let Some(map) = &fields_map
                && !map.contains(&field.name)
            {
                continue;
            }
            fields_writer.write_object_member(&field.name, |field_writer| {
                write_object_field_definition(field_writer, &field)?;
                Ok(())
            })?
        }
        Ok(())
    })?;
    Ok(())
}

fn write_objects<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    objects: &IndexMap<String, Rc<RefCell<server::ast::ObjectType>>>,
    server_uses_map: &Option<ServerUsesMap>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_objects = objects.clone();
    new_objects.sort_keys();
    for obj in new_objects.values() {
        let mut fields_map: Option<HashSet<String>> = None;
        let name = &obj.borrow().name;
        if let Some(map) = server_uses_map {
            if name == "Query" && map.queries.len() != 0 {
                fields_map = Some(map.queries.clone());
            } else if name == "Mutation" && map.mutations.len() != 0 {
                fields_map = Some(map.mutations.clone());
            } else if name == "Subscription" && map.subscriptions.len() != 0 {
                fields_map = Some(map.subscriptions.clone());
            } else if !map.objects.contains(name) {
                continue;
            };
        };
        writer.write_object_member(&name, |object_writer| {
            write_object(object_writer, &obj.borrow(), fields_map)
        })?
    }
    return Ok(());
}

fn write_interface<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    interface: &server::ast::Interface,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &interface.name)?;
    writer.write_object_member("fields", |fields_writer| {
        let mut fields = interface.fields.clone();
        fields.sort_keys();
        for field in fields.values() {
            fields_writer.write_object_member(&field.name, |field_writer| {
                write_object_field_definition(field_writer, &field)
            })?
        }
        Ok(())
    })
}

fn write_interfaces<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    interfaces: &IndexMap<String, Rc<RefCell<server::ast::Interface>>>,
    uses_hashset: &Option<&HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_interfaces = interfaces.clone();
    new_interfaces.sort_keys();
    for interface in new_interfaces.values().filter(|interface| {
        uses_hashset
            .map_or(true, |hashset| hashset.contains(&interface.borrow().name))
    }) {
        writer.write_object_member(
            &interface.borrow().name,
            |interface_writer| {
                write_interface(interface_writer, &interface.borrow())
            },
        )?
    }
    return Ok(());
}

fn write_input_type_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    t: &shared::ast::InputTypeSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match t {
        shared::ast::InputTypeSpec::Scalar(name) => {
            writer.write_string_member("_type", "Scalar")?;
            writer.write_string_member("name", &name)?;
        }
        shared::ast::InputTypeSpec::InputType(i) => {
            let name = &i.borrow().name;
            writer.write_string_member("_type", "InputType")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/inputs/{}", name),
            )?;
        }
        shared::ast::InputTypeSpec::Enum(e) => {
            let name = &e.name;
            writer.write_string_member("_type", "Enum")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/enums/{}", name),
            )?;
        }
    }
    Ok(())
}

fn write_literal_input_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "literal")?;
    writer.write_object_member("type", |type_writer| {
        write_input_type_spec(type_writer, &spec.r#type)
    })?;
    write_literal(writer, spec.default_value.as_ref().unwrap())?;
    Ok(())
}

fn write_array_input_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "array")?;
    writer.write_bool_member("nullable", spec.nullable)?;
    writer.write_object_member("type", |type_writer| {
        write_input_type_spec(type_writer, &spec.r#type)
    })?;
    write_array_literal(writer, spec.default_value.as_ref().unwrap_or(&None))?;
    Ok(())
}

fn write_input_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::InputFieldSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match spec {
        shared::ast::InputFieldSpec::Literal(literal) => {
            write_literal_input_field_spec(writer, literal)
        }
        shared::ast::InputFieldSpec::Array(array) => {
            write_array_input_field_spec(writer, array)
        }
    }
}

fn write_input_field_definition<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    field_definition: &shared::ast::FieldDefinition<
        shared::ast::InputFieldSpec,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_bool_member("nullable", field_definition.nullable)?;
    writer.write_object_member("spec", |spec_writer| {
        write_input_field_spec(spec_writer, &field_definition.spec)
    })?;
    Ok(())
}

fn write_input<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    input: &shared::ast::InputType,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &input.name)?;
    writer.write_object_member("fields", |fields_writer| {
        let mut fields = input.fields.clone();
        fields.sort_keys();
        for field in fields.values() {
            fields_writer.write_object_member(&field.name, |field_writer| {
                write_input_field_definition(field_writer, &field)
            })?
        }
        Ok(())
    })
}

fn write_inputs<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    inputs: &IndexMap<String, Rc<RefCell<shared::ast::InputType>>>,
    uses_hashset: &Option<&HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_inputs = inputs.clone();
    new_inputs.sort_keys();
    for input in new_inputs.values().filter(|input| {
        uses_hashset
            .map_or(true, |hashset| hashset.contains(&input.borrow().name))
    }) {
        writer.write_object_member(&input.borrow().name, |input_writer| {
            write_input(input_writer, &input.borrow())?;
            return Ok(());
        })?
    }
    return Ok(());
}

fn write_union<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    union: &server::ast::Union,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &union.name)?;
    writer.write_object_member("items", |items_writer| {
        let mut keys = union.items.keys().collect::<Vec<_>>();
        keys.sort();
        for name in keys {
            items_writer.write_string_member(
                name,
                &format!("#/server/objects/{}", name),
            )?;
        }
        Ok(())
    })
}

fn write_unions<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    unions: &IndexMap<String, Rc<RefCell<server::ast::Union>>>,
    uses_hashset: &Option<&HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_unions = unions.clone();
    new_unions.sort_keys();
    for union in new_unions.values().filter(|union| {
        uses_hashset
            .map_or(true, |hashset| hashset.contains(&union.borrow().name))
    }) {
        writer.write_object_member(&union.borrow().name, |union_writer| {
            write_union(union_writer, &union.borrow())
        })?
    }
    return Ok(());
}

fn write_enum<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    enum_type: &shared::ast::Enum,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &enum_type.name)?;
    writer.write_array_member("values", |values_writer| {
        for name in &enum_type.values {
            values_writer.write_string(name)?;
        }
        Ok(())
    })
}

fn write_enums<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    enums: &IndexMap<String, Rc<shared::ast::Enum>>,
    uses_hashset: &Option<&HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_enums = enums.clone();
    new_enums.sort_keys();
    for enum_type in new_enums.values().filter(|enum_type| {
        uses_hashset.map_or(true, |hashset| hashset.contains(&enum_type.name))
    }) {
        writer.write_object_member(&enum_type.name, |enum_writer| {
            write_enum(enum_writer, &enum_type)
        })?
    }
    return Ok(());
}

fn write_server_directive<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    directive: &shared::ast::ServerDirective,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("name", &directive.name)?;
    writer.write_array_member("locations", |locations_writer| {
        for location in &directive.locations {
            locations_writer.write_string(&location.to_string())?;
        }
        Ok(())
    })?;
    writer.write_object_member("arguments", |arguments_writer| {
        let mut new_arguments = directive.arguments.clone();
        new_arguments.sort_keys();
        for (name, argument) in &new_arguments {
            arguments_writer.write_object_member(name, |arg_writer| {
                write_input_field_definition(arg_writer, argument)
            })?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_server_directives<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    directives: &IndexMap<String, Rc<RefCell<shared::ast::ServerDirective>>>,
    uses_hashset: &Option<&HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_directives = directives.clone();
    new_directives.sort_keys();
    for (name, directive) in new_directives.iter().filter(|(name, _)| {
        uses_hashset.map_or(true, |hashset| hashset.contains(*name))
    }) {
        writer.write_object_member(name, |directive_writer| {
            write_server_directive(directive_writer, &directive.borrow())
        })?
    }
    return Ok(());
}

pub fn serialize_server_schema(
    schema: &server::schema::Schema,
    client_schema: Option<&client::schema::ClientSchema>,
) -> Result<String, String> {
    let server_uses_map =
        client_schema.map(|schema| schema.get_server_uses_map());
    let mut io_writer = Vec::<u8>::new();
    struson::writer::simple::ValueWriter::write_object(
        struson::writer::simple::SimpleJsonWriter::new(&mut io_writer),
        |schema_writer| {
            schema_writer.write_object_member("objects", |objects_writer| {
                write_objects(objects_writer, &schema.objects, &server_uses_map)
            })?;
            schema_writer.write_object_member(
                "interfaces",
                |interfaces_writer| {
                    write_interfaces(
                        interfaces_writer,
                        &schema.interfaces,
                        &server_uses_map.as_ref().map(|m| &m.interfaces),
                    )
                },
            )?;
            schema_writer.write_object_member("inputs", |inputs_writer| {
                write_inputs(
                    inputs_writer,
                    &schema.inputs,
                    &server_uses_map.as_ref().map(|m| &m.inputs),
                )
            })?;
            schema_writer.write_array_member("scalars", |scalars_writer| {
                let mut new_scalars = schema.scalars.clone();
                new_scalars.sort();
                for scalar in &new_scalars {
                    if let Some(map) = &server_uses_map
                        && !map.scalars.contains(scalar)
                    {
                        continue;
                    }
                    scalars_writer.write_string(scalar)?;
                }
                Ok(())
            })?;
            schema_writer.write_object_member("enums", |enums_writer| {
                write_enums(
                    enums_writer,
                    &schema.enums,
                    &server_uses_map.as_ref().map(|m| &m.enums),
                )
            })?;
            schema_writer.write_object_member("unions", |union_writer| {
                write_unions(
                    union_writer,
                    &schema.unions,
                    &server_uses_map.as_ref().map(|m| &m.unions),
                )
            })?;
            schema_writer.write_object_member(
                "directives",
                |directives_writer| {
                    write_server_directives(
                        directives_writer,
                        &schema.directives,
                        &server_uses_map.as_ref().map(|m| &m.directives),
                    )
                },
            )?;
            Ok(())
        },
    )
    .map_err(|e| e.to_string())?;
    return Ok(String::from_utf8(io_writer).unwrap());
}

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
    writer.write_string_member("fragment", &field.fragment.borrow().name)?;
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
            writer
                .write_string_member("union", &spread.r#type.borrow().name)?;
            writer.write_array_member("selections", |selections_writer| {
                for n_selection in &spread.selection.selections {
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
            writer
                .write_string_member("object", &spread.r#type.borrow().name)?;
            writer.write_object_member("spec", |spec_writer| {
                write_object_fragment_spec(spec_writer, &spread.selection)
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
    writer.write_string_member("name", &spec.r#type.borrow().name)?;
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
                write_field_selection_arguments(
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
    spec: &client::ast::ObjectFragmentSpec<server::ast::ObjectType>,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "ObjectFragmentSpec")?;
    writer.write_string_member("name", &spec.r#type.borrow().name)?;
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
    spec: &client::ast::ObjectFragmentSpec<server::ast::Interface>,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "ObjectFragmentSpec")?;
    writer.write_string_member("name", &spec.r#type.borrow().name)?;
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
    fragments: &IndexMap<String, Rc<RefCell<client::ast::Fragment>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_fragments = fragments.clone();
    new_fragments.sort_keys();
    for (name, fragment) in &new_fragments {
        writer.write_object_member(name, |fragment_writer| {
            write_fragment(fragment_writer, &fragment.borrow())
        })?
    }
    return Ok(());
}

fn write_operation_parameters<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    parameters: &IndexMap<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_parameters = parameters.clone();
    new_parameters.sort_keys();
    for (name, parameter) in &new_parameters {
        writer.write_object_member(name, |param_writer| {
            write_input_field_definition(param_writer, parameter)
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
    operations: &IndexMap<String, Rc<RefCell<client::ast::Operation>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_operations = operations.clone();
    new_operations.sort_keys();
    for (name, operation) in &new_operations {
        writer.write_object_member(name, |operation_writer| {
            write_operation(operation_writer, &operation.borrow())
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
                write_input_field_definition(arg_writer, argument)
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
    directives: &IndexMap<String, Rc<client::ast::ClientDirective>>,
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
    schema: &client::schema::ClientSchema,
) -> Result<String, String> {
    let mut io_writer = Vec::<u8>::new();
    struson::writer::simple::ValueWriter::write_object(
        struson::writer::simple::SimpleJsonWriter::new(&mut io_writer),
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
