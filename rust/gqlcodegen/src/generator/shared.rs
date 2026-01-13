use convert_case::Casing;

pub fn format_field_name(name: &str) -> String {
    name.to_case(convert_case::Case::Snake)
}
