use crate::schema;

use super::config::Config;

pub fn generate_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    gqlenum: &schema::server::gqlenum::Enum,
) {
    let local = scope
        .new_enum(&gqlenum.name)
        .derive("libgqlcodegen::macros::GQLEnum")
        .r#macro(format!("#[gql(scalar={})]", config.scalar_type))
        .vis("pub");
    for value in &gqlenum.values {
        local.push_variant(codegen::Variant::new(
            &super::shared::format_enum_variant(value),
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use indexmap::IndexMap;

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
                scalars_mapping: IndexMap::new(),
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
            r#"#[derive(libgqlcodegen::macros::GQLEnum)]
#[gql(scalar=ExampleScalar)]
pub enum Check {
    FirstValue,
    SecondValue,
    ThirdValue,
}"#
        )
    }
}
