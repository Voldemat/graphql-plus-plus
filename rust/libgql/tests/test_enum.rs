pub mod common;

pub struct Config {}

impl libgql::formatter::shared::config::Config for Config {
    fn get_indent_width(self: &Self) -> codeform::ir::shared::IndentWidth {
        codeform::ir::shared::IndentWidth::from_u8(4).unwrap()
    }
}

impl libgql::formatter::server::config::Config for Config {}

impl codeform::hir_to_lir::config::Config for Config {
    fn get_indent_width(self: &Self) -> codeform::ir::shared::IndentWidth {
        codeform::ir::shared::IndentWidth::from_u8(4).unwrap()
    }

    fn get_max_width(self: &Self) -> std::num::NonZeroU32 {
        std::num::NonZeroU32::try_from(80).unwrap()
    }
}

impl codeform::lir_printer::Config for Config {
    fn get_indent_width(self: &Self) -> codeform::ir::shared::IndentWidth {
        codeform::ir::shared::IndentWidth::from_u8(4).unwrap()
    }

    fn get_new_line_control_sequence(self: &Self) -> &'static [u8] {
        "\n".as_bytes()
    }
}

#[test]
fn test_format() {
    let hir_nodes = libgql::formatter::server::enum_type::format_node(
        &Config {},
        &Config {},
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
    let final_string =
        common::hir_nodes_to_string(&Config {}, &Config {}, hir_nodes);
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
