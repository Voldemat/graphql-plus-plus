use std::sync::Arc;

use crate::cli::config;

pub fn format_lexer_error<'buffer>(
    exc: &str,
    location: (usize, usize),
    source: &Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
) -> String {
    let buffer = format!("{}\n", source.filepath.display());

    buffer
}

pub fn format_parse_error<'buffer>(
    exc: &str,
    location: &libgql::lexer::tokens::TokenLocation,
    source: &Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
) -> String {
    let buffer = format!("{}\n", source.filepath.display());

    buffer
}

pub fn read_buffer_from_filepath(filepath: &std::path::Path) -> String {
    if filepath == Into::<std::path::PathBuf>::into("-") {
        let mut temp = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin().lock(), &mut temp)
            .unwrap();
        temp
    } else {
        std::fs::read_to_string(filepath).unwrap()
    }
}

pub fn print_result<T: serde::Serialize>(pretty: bool, value: T) {
    let func = if pretty {
        serde_json::to_writer_pretty
    } else {
        serde_json::to_writer
    };
    func(std::io::stdout(), &value).unwrap();
    print!("\n");
}

pub fn resolve_paths(
    config_dir_path: &std::path::Path,
    patterns: &[std::path::PathBuf],
) -> Vec<std::path::PathBuf> {
    patterns
        .iter()
        .map(|pattern| {
            glob::glob(
                std::path::Path::join(config_dir_path, pattern)
                    .to_str()
                    .expect("Pattern is not valid utf-8 string"),
            )
            .unwrap()
            .map(|result| result.unwrap())
        })
        .flatten()
        .collect()
}

pub fn load_server_schema_from_inputs(
    registry: &mut libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &config::InputsConfig,
) -> Result<(), Vec<String>> {
    let mut nodes = Vec::<libgql::parsers::file::server::ast::ASTNode>::new();
    let mut errors = Vec::<String>::new();
    for jsonpath in resolve_paths(config_dir_path, &conf.json_schema) {
        let buffer = std::fs::read_to_string(jsonpath).unwrap();
        libgql::json::parsers::schema::parse_server_schema(
            registry,
            serde_json::from_str::<serde_json::Value>(&buffer).unwrap(),
        )
        .unwrap();
    }
    let mut buffers = Vec::new();
    for graphql_path in resolve_paths(&config_dir_path, &conf.graphql) {
        let buffer = std::fs::read_to_string(&graphql_path).unwrap();
        buffers.push(buffer);
    }
    for (graphql_path, buffer) in resolve_paths(&config_dir_path, &conf.graphql)
        .iter()
        .zip(&buffers)
    {
        let source_file = std::sync::Arc::new(
            libgql::parsers::file::shared::ast::SourceFile {
                filepath: graphql_path.clone(),
                buffer: buffer.as_str(),
            },
        );
        let tokens =
            libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
                .unwrap();
        let file_nodes = match libgql::parsers::file::server::Parser::new(
            libgql::parsers::file::tokens_sources::VecTokensSource::new(
                tokens,
                source_file.clone(),
            ),
        )
        .parse_ast_nodes()
        {
            Ok(n) => n,
            Err(e) => {
                errors.push(format_parse_error(
                    &format!("{:?}", e),
                    e.get_location(),
                    &source_file,
                ));
                continue;
            }
        };

        nodes.extend(file_nodes);
    }
    if errors.len() > 0 {
        return Err(errors);
    }
    libgql::parsers::schema::server::parse_server_schema(registry, &nodes)
        .unwrap();
    Ok(())
}

pub fn load_client_schema_from_inputs(
    server_registry: &libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    registry: &mut libgql::parsers::schema::client::type_registry::TypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &config::InputsConfig,
) -> Result<(), Vec<String>> {
    let mut nodes = Vec::<libgql::parsers::file::client::ast::ASTNode>::new();
    let mut errors = Vec::<String>::new();
    let mut buffers = Vec::new();
    for graphql_path in resolve_paths(&config_dir_path, &conf.graphql) {
        let buffer = std::fs::read_to_string(&graphql_path).unwrap();
        buffers.push(buffer);
    }
    for (graphql_path, buffer) in resolve_paths(&config_dir_path, &conf.graphql)
        .iter()
        .zip(&buffers)
    {
        let source_file = std::sync::Arc::new(
            libgql::parsers::file::shared::ast::SourceFile {
                filepath: graphql_path.clone(),
                buffer: buffer.as_str(),
            },
        );
        let tokens =
            libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
                .unwrap();
        let file_nodes = match libgql::parsers::file::client::Parser::new(
            libgql::parsers::file::tokens_sources::VecTokensSource::new(
                tokens,
                source_file.clone(),
            ),
        )
        .parse_ast_nodes()
        {
            Ok(n) => n,
            Err(e) => {
                errors.push(format_lexer_error(
                    &format!("{:?}", e),
                    e.get_location(),
                    &source_file,
                ));
                continue;
            }
        };
        nodes.extend(file_nodes);
    }
    if errors.len() > 0 {
        return Err(errors);
    }
    match libgql::parsers::schema::client::parse_client_schema(
        server_registry,
        registry,
        &nodes,
    ) {
        Ok(_) => {}
        Err(error) => {
            errors.push(format!("{:?}", error));
            return Err(errors);
        }
    };
    return Ok(());
}

pub fn run_config_action<'a>(
    config_path: &std::path::Path,
    config: &'a config::Config,
    json_callback: Box<dyn Fn(&str, &std::path::Path, &str) + 'a>,
) -> Result<(), String> {
    let mut server_registry =
        libgql::parsers::schema::server::type_registry::HashMapTypeRegistry::new();
    match load_server_schema_from_inputs(
        &mut server_registry,
        config_path.parent().unwrap(),
        &config.server.inputs,
    ) {
        Ok(_) => {}
        Err(errors) => {
            for e in errors {
                println!("{}", e);
            }
            return Ok(());
        }
    };
    let client_registry = match config.client.as_ref().map(|client_config| {
        let mut client_registry =
            libgql::parsers::schema::client::type_registry::TypeRegistry::new();
        match load_client_schema_from_inputs(
            &server_registry,
            &mut client_registry,
            config_path.parent().unwrap(),
            &client_config.inputs,
        ) {
            Ok(_) => None,
            Err(errors) => {
                for e in errors {
                    println!("{}", e);
                }
                return None;
            }
        }
    }) {
        None => None,
        Some(None) => return Ok(()),
        Some(s) => s,
    };
    if let Some(outputs) = config.server.outputs.as_ref() {
        let json_string =
            libgql::json::serializers::schema::serialize_server_schema(
                &server_registry,
                if outputs.only_used_in_operations {
                    client_registry
                } else {
                    None
                },
            )?;
        json_callback(&json_string, &outputs.filepath, "Server");
    };

    if let Some(client_config) = &config.client
        && let Some(outputs) = &client_config.outputs
        && let Some(c_registry) = client_registry
    {
        let json_string =
            libgql::json::serializers::schema::serialize_client_schema(
                c_registry,
            )?;
        json_callback(&json_string, &outputs.filepath, "Client");
    };
    return Ok(());
}

pub fn does_file_have_changes(
    filepath: &std::path::Path,
    json_string: &str,
    schema_name: &str,
) -> Result<(), String> {
    if std::fs::read_to_string(filepath)
        .map_err(|e| {
            format!("Failed to read file: {:?} {}", filepath, e.to_string())
        })
        .unwrap()
        != json_string
    {
        return Err(format!("{} schema is not up to date", schema_name));
    }
    return Ok(());
}
