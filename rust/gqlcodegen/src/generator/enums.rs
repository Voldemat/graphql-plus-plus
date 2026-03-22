use crate::schema;

use super::config::Config;

pub fn generate_definition(
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
        .arg("self", "&Self")
        .ret("Result<&str, String>");
    to_str_fn.line("match self {");
    for value in &gqlenum.values {
        to_str_fn.line(format!(
            "Self::{} => Ok(\"{}\"),",
            super::shared::format_enum_variant(value),
            value,
        ));
    }
    to_str_fn.line("}");

    let resolver_value_impl_block =
        scope.new_impl(&gqlenum.name).impl_trait(format!(
            "libgql::executor::ast::ResolverValue<{}>",
            config.scalar_type
        ));
    resolver_value_impl_block.new_fn("to_value").generic("'a")
        .arg("self", "&'a Self")
        .ret(format!(
            "Result<libgql::executor::ast::ResolverIntrospectionValue<'a, {}>, String>",
            config.scalar_type
        ))
        .line(format!("libgql::executor::GQLEnum::<{}>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))", config.scalar_type));
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::generator::config::ResolversConfig;

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
        generate_definition(
            &Config {
                scalar_type: "ExampleScalar".to_string(),
                scalars_mapping: HashMap::new(),
                resolvers: ResolversConfig {
                    context_type: "()".to_string(),
                },
                field_to_resolver: HashSet::new(),
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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
