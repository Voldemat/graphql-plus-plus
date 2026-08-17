pub mod common;

#[test]
fn test_format() {
    let hir_nodes = libgql::formatter::server::enum_type::format_node(
        &libgql::formatter::config::Config {
            indent_width: codeform::ir::shared::IndentWidth::from_u8(4)
                .unwrap(),
        },
        &common::builders::build_enum(
            "CheckEnum",
            &[
                "FirstVariant",
                "SecondVariant",
                "ThirdVariant",
                "FourthVariant",
            ],
        ),
    );
    let final_string = common::hir_nodes_to_string(hir_nodes);
    pretty_assertions::assert_eq!(
        final_string,
        r#"enum CheckEnum {
    FirstVariant
    SecondVariant
    ThirdVariant
    FourthVariant
}"#
    );
}
