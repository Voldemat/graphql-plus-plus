use std::collections::HashSet;

use libgqlcodegen::{generator, schema};

fn run_schema() {
    let server_schema: schema::server::schema::Schema = serde_json_path_to_error::from_str(
        &std::fs::read_to_string("../graphql/server-schema.json").unwrap(),
    )
    .unwrap();
    let client_schema = schema::client::schema::Schema::default();
    let scalars_mapping = indexmap::IndexMap::<String, String>::from([
        ("Boolean".into(), "bool".into()),
        ("String".into(), "String".into()),
        ("Int".into(), "i32".into()),
        ("Float".into(), "f32".into()),
    ]);
    let s = generator::main::generate_ast(
        &generator::config::Config {
            scalars_mapping: scalars_mapping,
            scalar_type: "super::scalar::Scalar".into(),
            resolvers: generator::config::ResolversConfig {
                context_type: "super::context::Context".to_string(),
            },
            field_to_resolver: HashSet::from_iter([]),
        },
        &schema::Schema {
            server: server_schema,
            client: client_schema,
        },
    );
    let filepath = "./src/api/generated.rs";
    if std::env::var("GQL_OVERWRITE").unwrap_or("".to_string()) == "true" {
        std::fs::write(filepath, s).unwrap();
    } else {
        let current_content = std::fs::read_to_string(filepath).unwrap();
        if current_content != s {
            eprintln!(
                "./src/api/generated.rs contains stale code, try to build with env GQL_OVERWRITE=true to update code"
            );
        }
    };
}

fn main() {
    println!("cargo::rerun-if-changed=../graphql/server-schema.json");
    run_schema()
}
