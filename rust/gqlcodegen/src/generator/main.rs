use super::config::Config;
use crate::schema;
use schema::server::object::ObjectFieldSpec;
use schema::server::object::ObjectNonCallableFieldSpec;
use schema::server::object::ObjectType;

fn generate_enum_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    gqlenum: &schema::server::gqlenum::Enum,
) {
    let local = scope.new_enum(&gqlenum.name).vis("pub");
    for value in &gqlenum.values {
        local.push_variant(codegen::Variant::new(
            &super::shared::format_enum_variant(value),
        ));
    }
    let impl_block = scope.new_impl(&gqlenum.name).impl_trait(format!(
        "libgql::executor::GQLEnum<{}>",
        config.scalar_type
    ));
    let from_str_fn = impl_block
        .new_fn("from_string")
        .arg("s", "String")
        .ret("Result<Self, String>");
    from_str_fn.line("match s.as_str() {");
    for value in &gqlenum.values {
        from_str_fn.line(format!(
            "\"{}\" => Ok(Self::{}),",
            value,
            super::shared::format_enum_variant(value)
        ));
    }
    from_str_fn.line(format!(
        "_ => Err(format!(\"Unexpected value {{}} for enum {}\", s))",
        gqlenum.name
    ));
    from_str_fn.line("}");

    let to_str_fn = impl_block
        .new_fn("to_str")
        .arg("self", "Self")
        .ret("Result<&'static str, String>");
    to_str_fn.line("match self {");
    for value in &gqlenum.values {
        to_str_fn.line(format!(
            "Self::{} => Ok(\"{}\"),",
            super::shared::format_enum_variant(value),
            value,
        ));
    }
    to_str_fn.line("}");
}

fn generate_union_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    union: &schema::server::union::Union,
) {
    let local = scope.new_enum(&union.name).vis("pub");
    for item in union.items.keys() {
        let mut variant = codegen::Variant::new(&item);
        variant.tuple(&item);
        local.push_variant(variant);
    }

    let impl_block = scope.new_impl(&union.name).impl_trait(format!(
        "TryInto<(String, libgql::executor::Values<{}>)>",
        config.scalar_type
    ));
    impl_block.associate_type("Error", "String");
    let try_into_func = impl_block
        .new_fn("try_into")
        .arg_self()
        .ret(format!(
            "Result<(String, libgql::executor::Values<{}>), Self::Error>",
            config.scalar_type
        ));
    try_into_func.line("match self {");
    for item in union.items.keys() {
        try_into_func.line(format!("Self::{}(item) => TryInto::<(String, libgql::executor::Values::<{}>)>::try_into(item),", item, config.scalar_type));
    };
    try_into_func.line("}");
}

fn generate_input_type(
    config: &Config,
    field_type: &schema::shared::InputType,
) -> String {
    match field_type {
        schema::shared::InputType::InputType { name } => name.clone(),
        schema::shared::InputType::Enum { name } => name.clone(),
        schema::shared::InputType::Scalar { name } => {
            config.scalars_mapping[name].clone()
        }
    }
}

fn generate_object_type(
    config: &Config,
    field_type: &schema::server::object::ObjectType,
) -> String {
    match field_type {
        ObjectType::ObjectType { name } => name.clone(),
        ObjectType::Enum { name } => name.clone(),
        ObjectType::Scalar { name } => config.scalars_mapping[name].clone(),
        ObjectType::Union { name } => name.clone(),
        ObjectType::InterfaceType { name } => name.clone(),
    }
}

fn generate_input_field_spec_type(
    config: &Config,
    spec: &schema::shared::InputFieldSpec,
) -> String {
    match spec {
        schema::shared::InputFieldSpec::Array(a) => {
            let mut t = generate_input_field_spec_type(config, &a.field_type);
            if a.nullable {
                t = format!("Option<{}>", t);
            }
            return format!("Vec<{}>", t);
        }
        schema::shared::InputFieldSpec::Literal(a) => {
            return generate_input_type(config, &a.field_type);
        }
    }
}

fn generate_field_type<T>(
    config: &Config,
    field: &schema::shared::Field<T>,
    func: impl FnOnce(&Config, &T) -> String,
) -> String {
    let t = func(config, &field.spec);
    if field.nullable {
        return format!("Option<{}>", t);
    }
    return t;
}

fn generate_input_type_value_func(
    config: &Config,
    input_type: &schema::shared::InputType,
) -> String {
    match input_type {
        schema::shared::InputType::Scalar { name } => {
            format!(
                "<{} as libgql::executor::GQLScalar<{}>>::from_literal_value",
                config.scalars_mapping[name], config.scalar_type,
            )
        }
        schema::shared::InputType::Enum { name } => {
            format!(
                "<{} as libgql::executor::GQLEnum<{}>>::from_literal_value",
                name, config.scalar_type
            )
        }
        schema::shared::InputType::InputType { name } => {
            format!(
                "<{} as libgql::executor::GQLInput<{}>>::from_literal_value",
                name, config.scalar_type
            )
        }
    }
}

fn generate_input_field_spec_value(
    config: &Config,
    field_name: &str,
    input_expression: &str,
    spec: &schema::shared::InputFieldSpec,
) -> String {
    match spec {
        schema::shared::InputFieldSpec::Array(array) => {
            let mut element_func = format!(
                "|element: libgql::executor::Value<{}>| element.to_non_nullable_option()",
                config.scalar_type
            );
            let field_spec_value = generate_input_field_spec_value(
                config,
                field_name,
                "v",
                array.field_type.as_ref(),
            );
            if array.nullable {
                element_func +=
                    &format!(".map(|v| {}).transpose()", field_spec_value);
            } else {
                element_func += &format!(
                    ".ok_or(\"Unexpected null in non-nullable array\".to_string()).map(|v| {}).flatten()",
                    field_spec_value
                );
            };
            return format!(
                "libgql::executor::ast::extract_array({}, {})",
                input_expression, element_func
            );
        }
        schema::shared::InputFieldSpec::Literal(literal) => {
            return format!(
                "\n            {}.get_literal()\n            .ok_or(\"Unexpected array value for literal\".to_string())\n            .map({})\n            .flatten()",
                input_expression,
                generate_input_type_value_func(config, &literal.field_type)
            );
        }
    }
}

fn generate_input_field_value(
    config: &Config,
    variables_var_name: &str,
    input_name: &str,
    field_name: &str,
    field: &schema::shared::InputField,
) -> String {
    let var = format!(
        "{}.remove(\"{}\")\n        .map(libgql::executor::Value::to_non_nullable_option)\n        .flatten()\n",
        variables_var_name, field_name
    );
    if field.nullable {
        format!(
            "{}        .map(|v| {}        \n        ).transpose()?",
            var,
            generate_input_field_spec_value(
                config,
                field_name,
                "v",
                &field.spec
            )
        )
    } else {
        format!(
            "{}        .ok_or(\"{}: Required field {} is missing or null\".to_string())\n        .map(|v| {}\n        )\n        .flatten()?",
            var,
            input_name,
            field_name,
            generate_input_field_spec_value(
                config,
                field_name,
                "v",
                &field.spec,
            )
        )
    }
}

fn generate_input_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    input: &schema::server::input::Input,
) {
    let local = scope.new_struct(&input.name).vis("pub");
    for (name, field) in &input.fields {
        let field_name = super::shared::format_field_name(name);
        let mut field = codegen::Field::new(
            &field_name,
            generate_field_type(config, field, generate_input_field_spec_type),
        );
        field.visibility = Some("pub".into());
        local.push_field(field);
    }

    let impl_block = scope.new_impl(&input.name).impl_trait(format!(
        "libgql::executor::GQLInput<{}>",
        config.scalar_type
    ));
    let from_variables_func = impl_block
        .new_fn("from_variables")
        .arg(
            "mut variables",
            format!("libgql::executor::Values<{}>", config.scalar_type),
        )
        .ret("Result<Self, String>");
    let mut value_construction = format!("Ok({}{{\n", input.name);
    for (index, (field_name, field)) in input.fields.iter().enumerate() {
        let rust_name = super::shared::format_field_name(field_name);
        value_construction += "    ";
        value_construction += &rust_name;
        value_construction += ": ";
        value_construction += &generate_input_field_value(
            config,
            "variables",
            &input.name,
            field_name,
            field,
        );
        if index != input.fields.len() - 1 {
            value_construction += ",";
        }
        value_construction += "\n";
    }
    value_construction += "})";
    from_variables_func.line(value_construction);
}

fn generate_object_noncallable_field_spec_type(
    config: &Config,
    spec: &ObjectNonCallableFieldSpec,
) -> String {
    match spec {
        ObjectNonCallableFieldSpec::Array(a) => {
            let mut t = generate_object_noncallable_field_spec_type(
                config,
                &a.field_type,
            );
            if a.nullable {
                t = format!("Option<{}>", t);
            }
            return format!("Vec<{}>", t);
        }
        ObjectNonCallableFieldSpec::Literal(a) => {
            return generate_object_type(config, &a.field_type);
        }
    }
}

fn generate_object_field_spec_type(
    config: &Config,
    spec: &ObjectFieldSpec,
) -> String {
    match spec {
        ObjectFieldSpec::Array(a) => {
            let mut t = generate_object_noncallable_field_spec_type(
                config,
                &a.field_type,
            );
            if a.nullable {
                t = format!("Option<{}>", t);
            }
            return format!("Vec<{}>", t);
        }
        ObjectFieldSpec::Literal(a) => {
            return generate_object_type(config, &a.field_type);
        }
        ObjectFieldSpec::Callable(a) => {
            return generate_object_noncallable_field_spec_type(
                &config,
                &a.return_type,
            );
        }
    }
}

fn generate_object_type_value(
    config: &Config,
    input_expression: &str,
    type_spec: &schema::server::object::ObjectType,
) -> String {
    match type_spec {
        schema::server::object::ObjectType::ObjectType { name: _ } => {
            format!("TryInto::<(String, libgql::executor::Values::<{}>)>::try_into({})?.into()", config.scalar_type, input_expression)
        }
        schema::server::object::ObjectType::Union { name: _ } => {
            format!("TryInto::<(String, libgql::executor::Values::<{}>)>::try_into({})?.into()", config.scalar_type, input_expression)
        }
        schema::server::object::ObjectType::InterfaceType { name: _ } => {
            format!("TryInto::<(String, libgql::executor::Values::<{}>)>::try_into({})?.into()", config.scalar_type, input_expression)
        }
        schema::server::object::ObjectType::Enum { name } => {
            format!("<{} as libgql::executor::GQLEnum<{}>>::to_literal_value({})?", name, config.scalar_type, input_expression)
        }
        schema::server::object::ObjectType::Scalar { name } => {
            format!("<{} as libgql::executor::GQLScalar<{}>>::to_literal_value({})?", config.scalars_mapping[name], config.scalar_type, input_expression)
        }
    }
}

fn generate_object_non_callable_field_spec_value(
    config: &Config,
    input_expression: &str,
    spec: &schema::server::object::ObjectNonCallableFieldSpec,
) -> String {
    match spec {
        schema::server::object::ObjectNonCallableFieldSpec::Literal(
            literal,
        ) => {
            format!(
                "libgql::executor::NonNullableValue::Literal({})",
                generate_object_type_value(
                    config,
                    input_expression,
                    &literal.field_type
                )
            )
        }
        schema::server::object::ObjectNonCallableFieldSpec::Array(array) => {
            format!(
                "libgql::executor::NonNullableValue::Array({}.into_iter().map(|element| Ok({})).collect::<Result<Vec<_>, String>>()?)",
                input_expression,
                generate_object_field_value(
                    config,
                    "element",
                    array.nullable,
                    array.field_type.as_ref()
                )
            )
        }
    }
}

fn generate_object_field_value(
    config: &Config,
    input_expression: &str,
    nullable: bool,
    spec: &schema::server::object::ObjectNonCallableFieldSpec,
) -> String {
    if !nullable {
        format!(
            "libgql::executor::Value::NonNullable({})",
            generate_object_non_callable_field_spec_value(
                config,
                input_expression,
                spec
            )
        )
    } else {
        format!(
            "{}.map(|v| -> Result<libgql::executor::Value<{}>, String> {{Ok(libgql::executor::Value::NonNullable({}))}}).transpose()?.unwrap_or(libgql::executor::Value::Null)",
            input_expression,
            config.scalar_type,
            generate_object_non_callable_field_spec_value(config, "v", spec)
        )
    }
}

fn generate_object_fields_value(
    config: &Config,
    fields: &indexmap::IndexMap<String, schema::server::object::ObjectField>,
) -> String {
    let mut s = "[".to_string();
    for (field_name, nullable, spec) in
        fields
            .iter()
            .filter_map(|(field_name, field)| match &field.spec {
                schema::server::object::ObjectFieldSpec::Literal(l) => Some((
                    field_name,
                    field.nullable,
                    schema::server::object::ObjectNonCallableFieldSpec::Literal(
                        l.clone(),
                    ),
                )),
                schema::server::object::ObjectFieldSpec::Array(a) => Some((
                    field_name,
                    field.nullable,
                    schema::server::object::ObjectNonCallableFieldSpec::Array(
                        a.clone(),
                    ),
                )),
                schema::server::object::ObjectFieldSpec::Callable(_) => None,
            })
    {
        s += &format!(
            "(\"{}\".to_string(), {}),\n",
            field_name,
            generate_object_field_value(
                config,
                &format!(
                    "self.{}",
                    super::shared::format_field_name(field_name)
                ),
                nullable,
                &spec
            )
        )
    }
    s += "]";
    return s;
}

fn generate_object_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    object: &schema::server::object::Object,
) {
    let local = scope.new_struct(&object.name).vis("pub");
    for (name, field) in &object.fields {
        let field_name = super::shared::format_field_name(name);
        let mut field = codegen::Field::new(
            &field_name,
            generate_field_type(config, field, generate_object_field_spec_type),
        );
        field.visibility = Some("pub".into());
        local.push_field(field);
    }
    let impl_block = scope.new_impl(&object.name).impl_trait(format!(
        "TryInto<(String, libgql::executor::Values<{}>)>",
        config.scalar_type
    ));
    impl_block.associate_type("Error", "String");
    impl_block
        .new_fn("try_into")
        .arg_self()
        .ret(format!(
            "Result<(String, libgql::executor::Values<{}>), Self::Error>",
            config.scalar_type
        ))
        .line(format!(
            "Ok((\"{}\".to_string(), libgql::executor::Values::from_iter({})))",
            object.name,
            generate_object_fields_value(config, &object.fields)
        ));
}

pub fn generate_ast(config: &Config, schema: &crate::schema::Schema) -> String {
    let mut scope = codegen::Scope::new();
    for gqlenum in schema.server.enums.values() {
        generate_enum_definition(config, &mut scope, gqlenum);
    }
    for input in schema.server.inputs.values() {
        generate_input_definition(config, &mut scope, input);
    }
    for object in schema.server.objects.values() {
        generate_object_definition(config, &mut scope, object);
    }
    for union in schema.server.unions.values() {
        generate_union_definition(config, &mut scope, union);
    }
    return scope.to_string();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_generate_enum_definition() {
        let gqlenum = schema::server::gqlenum::Enum {
            name: "Check".into(),
            values: vec![
                "FIRST_VALUE".into(),
                "SECOND_VALUE".into(),
                "THIRD_VALUE".into(),
            ],
        };
        let mut scope = codegen::Scope::new();
        generate_enum_definition(
            &Config {
                scalar_type: "ExampleScalar".to_string(),
                scalars_mapping: HashMap::new(),
            },
            &mut scope,
            &gqlenum,
        );
        let output = scope.to_string();
        pretty_assertions::assert_eq!(
            output,
            r#"pub enum Check {
    FirstValue,
    SecondValue,
    ThirdValue,
}

impl libgql::executor::GQLEnum<ExampleScalar> for Check {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
        "FIRST_VALUE" => Ok(Self::FirstValue),
        "SECOND_VALUE" => Ok(Self::SecondValue),
        "THIRD_VALUE" => Ok(Self::ThirdValue),
        _ => Err(format!("Unexpected value {} for enum Check", s))
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
        Self::FirstValue => Ok("FIRST_VALUE"),
        Self::SecondValue => Ok("SECOND_VALUE"),
        Self::ThirdValue => Ok("THIRD_VALUE"),
        }
    }
}"#
        )
    }
}
