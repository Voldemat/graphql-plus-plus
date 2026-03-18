use convert_case::Casing;

pub fn format_field_name(name: &str) -> String {
    let v = name.to_case(convert_case::Case::Snake);
    match v.as_str() {
    "type" => "r#type".to_string(),
    _ => v
    }
}

pub fn format_enum_variant(name: &str) -> String {
    name.to_case(convert_case::Case::UpperCamel)
}
