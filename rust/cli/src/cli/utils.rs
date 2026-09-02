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
    conf: &super::config::InputsConfig,
) -> Result<Vec<String>, String> {
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
        match super::shared::buffer_to_server_ast(graphql_path, buffer) {
            Ok(file_nodes) => {
                nodes.extend(file_nodes);
            }
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
    }
    if errors.len() > 0 {
        return Ok(errors);
    }
    libgql::parsers::schema::server::parse_server_schema(registry, &nodes)
        .map(|_| Vec::new())
        .map_err(|e| super::format_error::format_server_schema_error(e))
}

pub fn load_client_schema_from_inputs(
    server_registry: &libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    registry: &mut libgql::parsers::schema::client::type_registry::TypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &super::config::InputsConfig,
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
        match super::shared::buffer_to_client_ast(graphql_path, buffer) {
            Ok(file_nodes) => nodes.extend(file_nodes),
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
    }
    match libgql::parsers::schema::client::parse_client_schema(
        server_registry,
        registry,
        &nodes,
    )
    .err()
    {
        None => {}
        Some(new_errors) => {
            errors.extend(
                new_errors.into_iter().map(|e| {
                    super::format_error::format_client_schema_error(e)
                }),
            );
        }
    };
    if errors.len() > 0 {
        return Err(errors);
    }
    Ok(())
}

pub fn run_config_action<'a>(
    config_path: &std::path::Path,
    config: &'a super::config::Config,
    json_callback: Box<dyn Fn(&str, &std::path::Path, &str) + 'a>,
) -> Result<(), String> {
    let mut server_registry =
        libgql::parsers::schema::server::type_registry::HashMapTypeRegistry::new();
    let Some(config_server) = config.server.as_ref() else {
        return Err("config.server is not defined".to_string());
    };
    match load_server_schema_from_inputs(
        &mut server_registry,
        config_path.parent().unwrap(),
        &config_server.inputs,
    ) {
        Ok(errors) => {
            if errors.len() > 0 {
                for e in errors {
                    println!("{}", e);
                }
                return Ok(());
            }
        }
        Err(error) => {
            println!("{}", error);
            return Ok(());
        }
    };
    let client_registry = match config.client.as_ref().map(|client_config| {
        let mut c_registry =
            libgql::parsers::schema::client::type_registry::TypeRegistry::new();
        match load_client_schema_from_inputs(
            &server_registry,
            &mut c_registry,
            config_path.parent().unwrap(),
            &client_config.inputs,
        ) {
            Ok(()) => Some(c_registry),
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
    if let Some(outputs) = config_server.outputs.as_ref() {
        let json_string =
            libgql::json::serializers::schema::server::serialize_server_schema(
                &server_registry,
                if outputs.only_used_in_operations {
                    client_registry.as_ref()
                } else {
                    None
                },
                outputs.pretty,
            )?;
        json_callback(&json_string, &outputs.filepath, "Server");
    };

    if let Some(client_config) = &config.client
        && let Some(outputs) = &client_config.outputs
        && let Some(c_registry) = client_registry
    {
        let json_string =
            libgql::json::serializers::schema::client::serialize_client_schema(
                &c_registry,
                outputs.pretty,
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
