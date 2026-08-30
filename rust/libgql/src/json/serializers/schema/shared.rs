use indexmap::IndexMap;
use struson::writer::simple::ValueWriter;

use crate::parsers::schema::shared;

pub fn write_input_type_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    t: &shared::ast::runtime::InputTypeSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match t {
        shared::ast::runtime::InputTypeSpec::Scalar(name) => {
            writer.write_string_member("_type", "Scalar")?;
            writer.write_string_member("name", &name)?;
        }
        shared::ast::runtime::InputTypeSpec::InputType(name) => {
            writer.write_string_member("_type", "InputType")?;
            writer.write_string_member("name", name)?;
            writer.write_string_member(
                "$ref",
                &format!("#/server/inputs/{}", name),
            )?;
        }
        shared::ast::runtime::InputTypeSpec::Enum(name) => {
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

pub fn write_literal<'a, J: struson::writer::JsonWriter>(
    member_writer: struson::writer::simple::MemberValueWriter<'a, J>,
    value: &shared::ast::runtime::Literal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    member_writer.write_object(|writer| match value {
        shared::ast::runtime::Literal::Null => writer
            .write_string_member("_type", "null")
            .map_err(|e| e.into()),
        shared::ast::runtime::Literal::Int(i) => {
            writer.write_string_member("_type", "int")?;
            writer
                .write_number_member("value", *i)
                .map_err(|e| e.into())
        }
        shared::ast::runtime::Literal::Float(f) => {
            writer.write_string_member("_type", "float")?;
            writer
                .write_fp_number_member("value", *f)
                .map_err(|e| e.into())
        }
        shared::ast::runtime::Literal::Boolean(b) => {
            writer.write_string_member("_type", "boolean")?;
            writer.write_bool_member("value", *b).map_err(|e| e.into())
        }
        shared::ast::runtime::Literal::String(s) => {
            writer.write_string_member("_type", "string")?;
            writer.write_string_member("value", s).map_err(|e| e.into())
        }
        shared::ast::runtime::Literal::EnumValue(s) => {
            writer.write_string_member("_type", "enum-value")?;
            writer.write_string_member("value", s).map_err(|e| e.into())
        }
    })
}

pub fn write_literal_input_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::runtime::LiteralFieldSpec<
        shared::ast::runtime::InputTypeSpec,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "literal")?;
    writer.write_object_member("type", |type_writer| {
        write_input_type_spec(type_writer, &spec.r#type)
    })?;
    if let Some(default_value) = spec.default_value.as_ref() {
        writer.write_member("defaultValue", |member_writer| {
            write_literal(member_writer, default_value)
        })?;
    }
    Ok(())
}

pub fn write_non_callable_input_field_spec<
    'a,
    J: struson::writer::JsonWriter,
>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::runtime::NonCallableFieldSpec<
        shared::ast::runtime::InputTypeSpec,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    match spec {
        shared::ast::runtime::NonCallableFieldSpec::Literal(literal) => {
            write_literal_input_field_spec(writer, literal)
        }
        shared::ast::runtime::NonCallableFieldSpec::Array(array) => {
            write_array_input_field_spec(writer, array)
        }
    }
}

pub fn write_array_literal<'a, J: struson::writer::JsonWriter>(
    writer: struson::writer::simple::MemberValueWriter<'a, J>,
    value: &shared::ast::runtime::ArrayLiteral,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_array(
        |array_writer| -> Result<(), Box<dyn std::error::Error>> {
            match value {
                shared::ast::runtime::ArrayLiteral::Int(i) => Ok(i
                    .iter()
                    .try_for_each(|v| array_writer.write_number(*v))?),

                shared::ast::runtime::ArrayLiteral::Float(f) => Ok(f
                    .iter()
                    .try_for_each(|v| array_writer.write_fp_number(*v))?),
                shared::ast::runtime::ArrayLiteral::Boolean(b) => Ok(b
                    .iter()
                    .try_for_each(|v| array_writer.write_bool(*v))?),
                shared::ast::runtime::ArrayLiteral::String(s) => {
                    Ok(s.iter()
                        .try_for_each(|v| array_writer.write_string(v))?)
                }
            }
        },
    )
}

pub fn write_array_input_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::runtime::ArrayFieldSpec<
        shared::ast::runtime::InputTypeSpec,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_string_member("_type", "array")?;
    writer.write_bool_member("nullable", spec.nullable)?;
    writer.write_object_member("type", |type_writer| {
        write_non_callable_input_field_spec(type_writer, &spec.r#type)
    })?;
    if let Some(default_value) = spec.default_value.as_ref() {
        writer.write_member("defaultValue", |default_value_writer| {
            write_array_literal(default_value_writer, default_value)
        })?;
    }
    Ok(())
}

fn write_input_field_spec<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    spec: &shared::ast::runtime::InputFieldSpec,
) -> Result<(), Box<dyn std::error::Error>> {
    match spec {
        shared::ast::runtime::InputFieldSpec::Literal(literal) => {
            write_literal_input_field_spec(writer, literal)
        }
        shared::ast::runtime::InputFieldSpec::Array(array) => {
            write_array_input_field_spec(writer, array)
        }
    }
}

pub fn write_input_field_definition<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    field_definition: &shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::InputFieldSpec,
    >,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_bool_member("nullable", field_definition.nullable)?;
    writer.write_object_member("spec", |spec_writer| {
        write_input_field_spec(spec_writer, &field_definition.spec)
    })?;
    Ok(())
}

pub fn write_argument_value<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    value: &shared::ast::runtime::ArgumentValue,
) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        shared::ast::runtime::ArgumentValue::Ref(r) => {
            writer.write_string_member("_type", "ref")?;
            writer.write_string_member("name", r)?;
        }
        shared::ast::runtime::ArgumentValue::Literal(literal) => {
            writer.write_string_member("_type", "literal")?;
            writer.write_member("literal", |literal_writer| {
                write_literal(literal_writer, literal)
            })?;
        }
    }
    Ok(())
}

pub fn write_field_selection_arguments<'a, J: struson::writer::JsonWriter>(
    writer: &mut struson::writer::simple::ObjectWriter<'a, J>,
    arguments: &IndexMap<String, shared::ast::runtime::FieldSelectionArgument>,
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
