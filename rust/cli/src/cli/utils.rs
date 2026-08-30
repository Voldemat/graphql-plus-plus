use std::sync::Arc;

use crate::cli::config;

pub fn format_lexer_error<'buffer>(
    exc: &str,
    location: libgql::lexer::tokens::TokenLocation,
    source: &Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
) -> String {
    format_error_with_range(exc, location.start, location.end, source)
}

pub fn format_parse_error<'buffer>(
    exc: &str,
    location: &libgql::lexer::tokens::TokenLocation,
    source: &Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
) -> String {
    format_error_with_range(exc, location.start, location.end, source)
}

const CONTEXT_LINES: usize = 5;
fn format_error_with_range<'buffer>(
    exc: &str,
    start: usize,
    end: usize,
    source: &Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
) -> String {
    let buffer = source.buffer;

    // Bounds checking to prevent slicing panics
    let start = start.min(buffer.len());
    let end = end.min(buffer.len()).max(start);

    // Calculate error line index (0-indexed)
    let target_line_idx = buffer[..start].lines().count().saturating_sub(1);

    // Collect lines to extract context ranges safely
    let lines: Vec<&str> = buffer.lines().collect();
    if lines.is_empty() {
        return format!("error: {}\n --> {}\n", exc, source.filepath.display());
    }

    // Determine range of lines to display
    let start_line_idx = target_line_idx.saturating_sub(CONTEXT_LINES);
    let end_line_idx = (target_line_idx + CONTEXT_LINES + 1).min(lines.len());

    // Compute column offset (1-indexed character position) for the target line
    let line_start_offset =
        buffer[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let start_col = buffer[line_start_offset..start].chars().count();
    let length = buffer[start..end].chars().count().max(1);

    // Calculate max padding based on the largest line number rendered
    let max_line_num = end_line_idx;
    let pad_len = max_line_num.to_string().len();

    let mut output = format!(
        " --> {}:{}:{}\n{:pad_len$} |\n",
        source.filepath.display(),
        target_line_idx + 1,
        start_col + 1,
        ""
    );

    // Render context lines before, target line with message under carets, and context lines after
    for line_idx in start_line_idx..end_line_idx {
        let line_number = line_idx + 1;
        let line_content = lines[line_idx];

        output.push_str(&format!(
            "{:width$} | {}\n",
            line_number,
            line_content,
            width = pad_len
        ));

        // Insert caret line with error string appended right under the targeted line
        if line_idx == target_line_idx {
            let spaces = " ".repeat(start_col);
            let carets = "^".repeat(length + 1);
            output.push_str(&format!(
                "{:pad_len$} | {}{} {}\n",
                "", spaces, carets, exc
            ));
        }
    }

    output
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

pub fn format_server_schema_error(
    error: libgql::parsers::schema::server::Error<'_>,
) -> String {
    let node_location = error.get_location();
    format_error_with_range(
        &format!("{error}"),
        node_location.start,
        node_location.end,
        &node_location.source,
    )
}

pub fn format_client_schema_error<
    's,
    S: libgql::parsers::schema::shared::ast::AsStr<'s>,
>(
    error: libgql::parsers::schema::client::errors::Error<'s, S>,
) -> String {
    let node_location = error.get_location();
    format_error_with_range(
        &format!("{error}"),
        node_location.start,
        node_location.end,
        &node_location.source,
    )
}

pub fn load_server_schema_from_inputs(
    registry: &mut libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &config::InputsConfig,
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
                    &format!("{}", e),
                    e.get_location(),
                    &source_file,
                ));
                continue;
            }
        };

        nodes.extend(file_nodes);
    }
    if errors.len() > 0 {
        return Ok(errors);
    }
    libgql::parsers::schema::server::parse_server_schema(registry, &nodes)
        .map(|_| Vec::new())
        .map_err(|e| format_server_schema_error(e))
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
                    &format!("{}", e),
                    e.get_location(),
                    &source_file,
                ));
                continue;
            }
        };
        nodes.extend(file_nodes);
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
                new_errors
                    .into_iter()
                    .map(|e| format_client_schema_error(e)),
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
    config: &'a config::Config,
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
