use crate::schema;

use super::config::Config;

pub fn generate_input_type(
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

pub fn generate_input_field_spec_type(
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

pub fn generate_input_field_spec_value(
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
                    &format!(".map(|v|{}).transpose()", field_spec_value);
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

pub fn generate_input_field_value(
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
            "{}        .map(|v|{}\n        ).transpose()?",
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
            "{}        .ok_or(\"{}: Required field {} is missing or null\".to_string())\n        .map(|v|{}\n        )\n        .flatten()?",
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

pub fn generate_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    input: &schema::server::input::Input,
) {
    let local = scope.new_struct(&input.name).vis("pub");
    for (name, field) in &input.fields {
        let field_name = super::shared::format_field_name(name);
        let mut field = codegen::Field::new(
            &field_name,
            super::shared::generate_field_type(
                config,
                field,
                generate_input_field_spec_type,
                false,
            ),
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

pub fn generate_input_type_downcast_type(
    config: &Config,
    input_type: &schema::shared::InputType,
) -> String {
    match input_type {
        schema::shared::InputType::Scalar { name } => {
            format!("{}", config.scalars_mapping[name])
        }
        schema::shared::InputType::Enum { name } => name.clone(),
        schema::shared::InputType::InputType { name } => name.clone(),
    }
}

pub fn generate_extract_input_field_spec_expression(
    config: &Config,
    input_expression: &str,
    spec: &schema::shared::InputFieldSpec,
) -> String {
    match spec {
        schema::shared::InputFieldSpec::Literal(literal) => {
            format!(
                "{}.downcast_ref::<{}>().unwrap()",
                input_expression,
                generate_input_type_downcast_type(config, &literal.field_type)
            )
        }
        schema::shared::InputFieldSpec::Array(array) => {
            match array.field_type.as_ref() {
                schema::shared::InputFieldSpec::Array(_) => {
                    format!(
                        "{}.downcast_ref::<Vec<Box<dyn std::any::Any>>>().unwrap().iter().map(|element| {}).collect()",
                        input_expression,
                        generate_extract_input_field_spec_expression(
                            config,
                            "element",
                            array.field_type.as_ref(),
                        )
                    )
                }
                schema::shared::InputFieldSpec::Literal(literal) => {
                    let downcast_type = generate_input_type_downcast_type(
                        config,
                        &literal.field_type,
                    );
                    let element_downcast_type = if array.nullable {
                        format!("Option<{}>", downcast_type)
                    } else {
                        downcast_type
                    };

                    format!(
                        "{}.downcast_ref::<Vec<{}>>().unwrap()",
                        input_expression, element_downcast_type
                    )
                }
            }
        }
    }
}

pub fn generate_extract_arg_expression(
    config: &Config,
    variables_var_name: &str,
    arg_name: &str,
    arg: &schema::shared::InputField,
) -> String {
    let var = format!("{}.get(\"{}\")", variables_var_name, arg_name);
    if arg.nullable {
        format!(
            "{}.map(|v| {})",
            var,
            generate_extract_input_field_spec_expression(
                config, "v", &arg.spec
            )
        )
    } else {
        format!(
            "{}",
            generate_extract_input_field_spec_expression(
                config,
                &format!("{}.unwrap()", var),
                &arg.spec
            )
        )
    }
}
