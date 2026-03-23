use convert_case::Casing;

use crate::schema;

use super::config::Config;

pub fn format_field_name(name: &str) -> String {
    let v = name.to_case(convert_case::Case::Snake);
    match v.as_str() {
        "type" => "r#type".to_string(),
        _ => v,
    }
}

pub fn format_enum_variant(name: &str) -> String {
    name.to_case(convert_case::Case::UpperCamel)
}

pub fn generate_field_type<T>(
    config: &Config,
    field: &schema::shared::Field<T>,
    func: impl FnOnce(&Config, &T) -> String,
    as_ref: bool,
) -> String {
    let mut t = func(config, &field.spec);
    if as_ref {
        t = format!("&{}", t)
    }
    if field.nullable {
        format!("Option<{}>", t)
    } else {
        t
    }
}
