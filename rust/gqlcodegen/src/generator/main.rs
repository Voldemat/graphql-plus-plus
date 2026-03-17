use super::config::Config;
use crate::schema;
use schema::server::object::ObjectFieldSpec;
use schema::server::object::ObjectNonCallableFieldSpec;
use schema::server::object::ObjectType;

fn generate_enum_definition(
    scope: &mut codegen::Scope,
    gqlenum: &schema::server::gqlenum::Enum,
) {
    let local = scope.new_enum(&gqlenum.name).vis("pub");
    for value in &gqlenum.values {
        local.push_variant(codegen::Variant::new(
            &super::shared::format_enum_variant(value),
        ));
    }
    let impl_block = scope
        .new_impl(&gqlenum.name)
        .impl_trait("libgql::executor::GQLEnum".to_string());
    let from_str_fn = impl_block
        .new_fn("from_str")
        .arg("s", "&str")
        .ret("Result<Self, String>");
    from_str_fn.line("match s {");
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
}

fn generate_input_type(
    config: &Config,
    field_type: &schema::shared::InputType,
) -> String {
    match field_type {
        schema::shared::InputType::InputType { name } => name.clone(),
        schema::shared::InputType::Enum { name } => name.clone(),
        schema::shared::InputType::Scalar { name } => {
            println!("{}", name);
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
        ObjectType::Scalar { name } => {
            println!("{}", name);
            config.scalars_mapping[name].clone()
        }
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

fn generate_input_type_value(
    config: &Config,
    input_expression: &str,
    input_type: &schema::shared::InputType,
) -> String {
    match input_type {
        schema::shared::InputType::Scalar { name } => {
            let mut m = format!("match {} {{", input_expression);
            m += "libgql::executor::LiteralValue::Scalar(scalar) => ";
            m += &format!(
                "<{} as libgql::executor::GQLScalar<{}>>::from_scalar(scalar),\n",
                config.scalars_mapping[name], config.scalar_type
            );
            m += "libgql::executor::LiteralValue::Object(_, o) => Err(";
            m += "format!(\"Unexpected object value for scalar field: {:?}\", o)";
            m += ")\n";
            m += "}";
            return m;
        }
        schema::shared::InputType::Enum { name } => {
            let mut m = format!("match {} {{", input_expression);
            m += "libgql::executor::LiteralValue::Scalar(scalar) => ";
            m += &format!(
                "<{} as libgql::executor::GQLEnum>::from_str({}),\n",
                name,
                format!(
                    "<{} as libgql::executor::Scalar>::get_str(scalar)",
                    config.scalar_type
                ) + &format!(
                    ".ok_or(\"Unexpected non-string scalar for enum: {}\".to_string())?",
                    name
                )
            );
            m += "libgql::executor::LiteralValue::Object(_, o) => Err(";
            m += "format!(\"Unexpected object value for scalar field: {:?}\", o)";
            m += ")\n";
            m += "}";
            return m;
        }
        schema::shared::InputType::InputType { name } => {
            let mut m = format!("match {} {{", input_expression);
            m += "libgql::executor::LiteralValue::Object(_, o) => ";
            m += &format!(
                "<{} as libgql::executor::GQLInput<{}>>",
                name, config.scalar_type
            );
            m += &format!("::from_variables(o),\n");
            m += "libgql::executor::LiteralValue::Scalar(scalar) => Err(";
            m += "format!(\"Unexpected scalar value for input field: {:?}\", scalar)";
            m += ")\n";
            m += "}";
            return m;
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
            let mut m = format!("match {} {{", input_expression);
            m += "libgql::executor::NonNullableValue::Array(array) =>";
            m += "array.iter().map(|element| match element {\n";
            m += "libgql::executor::Value::Null => ";
            if array.nullable {
                m += "Ok(None)";
            } else {
                m += "Err(\"Unexpected null in non-nullable array\".to_string())";
            };
            m += ",\n";
            m += "libgql::executor::Value::NonNullable(n) => ";
            let field_spec_value = generate_input_field_spec_value(
                    config,
                    field_name,
                    "n",
                    array.field_type.as_ref()
                );
            if array.nullable {
                m += &format!("{}.map(|v| Some(v))", field_spec_value);
            } else {
                m += &field_spec_value;
            };
            m += "})\n";
            m += ".collect::<Result<Vec<_>, String>>()";
            m += ",\n";
            m += "libgql::executor::NonNullableValue::Literal(l) => Err(";
            m += &format!(
                "format!(\"{}: Unexpected literal value for array: {{:?}}\", l))",
                field_name
            );
            m += "}";
            return m;
        }
        schema::shared::InputFieldSpec::Literal(literal) => {
            let mut m = format!("match {} {{", input_expression);
            m += "libgql::executor::NonNullableValue::Literal(l) => ";
            m += &generate_input_type_value(config, "l", &literal.field_type);
            m += ",\n";
            m += "libgql::executor::NonNullableValue::Array(a) => Err(";
            m += &format!(
                "format!(\"{}: Unexpected array value for literal: {{:?}}\", a))",
                field_name
            );
            m += "}";
            return m;
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
    let mut var = format!(
        "{}.get(\"{}\").map(|v| match v {{\n",
        variables_var_name, field_name
    );
    var += &format!("    libgql::executor::Value::Null => None,\n");
    var += &format!("    libgql::executor::Value::NonNullable(n) => Some(n)\n");
    var += &format!("}}).flatten()");
    if field.nullable {
        format!(
            "{}.map(|v| {}).transpose()?",
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
            "{}?",
            generate_input_field_spec_value(
                config,
                field_name,
                &format!(
                    "{}.ok_or(\"{}: Required field {} is missing or null\")?",
                    var, input_name, field_name
                ),
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
        let mut field_name = super::shared::format_field_name(name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        };
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
            "variables",
            format!("&libgql::executor::Values<{}>", config.scalar_type),
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

fn generate_object_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    object: &schema::server::object::Object,
) {
    let local = scope.new_struct(&object.name).vis("pub");
    for (name, field) in &object.fields {
        let mut field_name = super::shared::format_field_name(name);
        if field_name == "type" {
            field_name = "r#type".to_string();
        }
        let mut field = codegen::Field::new(
            &field_name,
            generate_field_type(config, field, generate_object_field_spec_type),
        );
        field.visibility = Some("pub".into());
        local.push_field(field);
    }
}

pub fn generate_ast(config: &Config, schema: &crate::schema::Schema) -> String {
    let mut scope = codegen::Scope::new();
    for gqlenum in schema.server.enums.values() {
        generate_enum_definition(&mut scope, gqlenum);
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
        generate_enum_definition(&mut scope, &gqlenum);
        let output = scope.to_string();
        assert_eq!(
            output,
            r#"pub enum Check {
    FirstValue,
    SecondValue,
    ThirdValue,
}

impl libgql::executor::GQLEnum for Check {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
        "FIRST_VALUE" => Ok(Self::FirstValue),
        "SECOND_VALUE" => Ok(Self::SecondValue),
        "THIRD_VALUE" => Ok(Self::ThirdValue),
        _ => Err(format!("Unexpected value {} for enum Check", s))
        }
    }
}"#
        )
    }
}
