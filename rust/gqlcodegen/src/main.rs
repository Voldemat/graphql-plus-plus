use std::{collections::HashMap, fs::read_to_string};

use crate::check::run_graphql;

mod generator;
mod schema;
mod generated;
mod check;
mod scalar;

fn run_schema() {
    let server_schema: schema::server::schema::Schema =
        serde_json_path_to_error::from_str(
            &read_to_string("./server-schema.json").unwrap(),
        )
        .unwrap();
    let client_schema: schema::client::schema::Schema =
        serde_json_path_to_error::from_str(
            &read_to_string("./client-schema.json").unwrap(),
        )
        .unwrap();
    let scalars_mapping = HashMap::<String, String>::from([
        ("Datetime".into(), "chrono::DateTime<chrono::Utc>".into()),
        ("Boolean".into(), "bool".into()),
        ("String".into(), "String".into()),
        ("Int".into(), "i32".into()),
        ("UUID".into(), "uuid::Uuid".into()),
        ("Int64".into(), "i64".into()),
        ("Url".into(), "url::Url".into())
    ]);
    let s = generator::main::generate_ast(
        &generator::config::Config {
            scalars_mapping: scalars_mapping,
            scalar_type_override: Some("super::scalar::MyScalarValue".into())
        },
        &schema::Schema {
            server: server_schema,
            client: client_schema,
        },
    );
    std::fs::write("./src/generated.rs", s).unwrap();
}

fn run_server() -> Result<(), std::io::Error> {
    run_graphql()
}

fn main() -> Result<(), std::io::Error> {
    run_schema();
    Ok(())
}
