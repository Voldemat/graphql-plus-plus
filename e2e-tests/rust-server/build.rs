use std::collections::HashSet;

use libgqlcodegen::{format, generator, schema};

fn run_schema() {
    let server_schema: schema::server::schema::Schema =
        serde_json::from_str(&std::fs::read_to_string("../graphql/server-schema.json").unwrap())
            .unwrap();
    let client_schema = schema::client::schema::Schema::default();
    let scalars_mapping = indexmap::IndexMap::<String, String>::from([
        ("Boolean".into(), "bool".into()),
        ("String".into(), "String".into()),
        ("Int".into(), "i32".into()),
        ("Float".into(), "f32".into()),
        ("UUID".into(), "uuid::Uuid".into())
    ]);
    let filepath = "./src/api/generated.rs";
    let (imports, code_map) =
        generator::main::extract_resolvers_code(std::fs::read_to_string(filepath).unwrap());
    let mut scope = codegen::Scope::new();
    if imports != "" {
        scope.raw(imports);
    }
    generator::main::generate_ast(
        &generator::config::Config {
            scalars_mapping: scalars_mapping,
            scalar_type: "super::scalar::Scalar".into(),
            resolvers: generator::config::ResolversConfig {
                context_type: "super::context::Context".to_string(),
            },
            field_to_resolver: HashSet::from_iter([
                ("User".to_string(), "email".to_string())
            ]),
        },
        &schema::Schema {
            server: server_schema,
            client: client_schema,
        },
        code_map,
        &mut scope,
    );
    let formatted = format::format_using_rustfmt("./", &scope.to_string()).unwrap();
    if std::env::var("GQL_OVERWRITE").unwrap_or("".to_string()) == "true" {
        std::fs::write(filepath, formatted).unwrap();
    } else {
        let current_content = std::fs::read_to_string(filepath).unwrap();
        if current_content != formatted {
            eprintln!(
                "./src/api/generated.rs contains stale code, try to build with env GQL_OVERWRITE=true to update code"
            );
            std::process::exit(1);
        }
    };
}

fn main() {
    println!("cargo::rerun-if-changed=./../graphql/server-schema.json");
    run_schema()
}
