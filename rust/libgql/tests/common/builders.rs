use libgql::parsers::file::{server, shared};

pub fn build_location<'s>() -> shared::ast::NodeLocation<'s> {
    shared::ast::NodeLocation {
        start: 0,
        end: 0,
        source: std::sync::Arc::new(shared::ast::SourceFile {
            buffer: "",
            filepath: std::path::PathBuf::new(),
        }),
    }
}

pub fn build_name<'s>(name: &'s str) -> shared::ast::NameNode<'s> {
    shared::ast::NameNode {
        name,
        location: build_location(),
    }
}

pub fn build_enum_value<'s>(
    name: &'s str,
) -> server::ast::EnumValueDefinitionNode<'s> {
    server::ast::EnumValueDefinitionNode {
        location: build_location(),
        value: build_name(name),
        directives: Vec::new(),
    }
}

pub fn build_enum<'s>(
    name: &'s str,
    values: &[&'s str],
) -> server::ast::EnumDefinitionNode<'s> {
    server::ast::EnumDefinitionNode {
        location: build_location(),
        name: build_name(name),
        directives: Vec::new(),
        values: values.iter().map(|value| build_enum_value(value)).collect(),
    }
}
