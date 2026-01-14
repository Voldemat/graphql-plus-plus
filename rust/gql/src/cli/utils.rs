use std::rc::Rc;

use crate::cli::config;

pub fn format_line(
    line: &str,
    current_line: u32,
    location: &libgql::lexer::tokens::Location,
    exc: &str,
) -> String {
    let linestr = current_line.to_string();
    let mut buffer = format!("{}: {}\n", linestr, line);

    if current_line == location.get_line() {
        let mut underline = String::new();

        // spaces before the underline
        let spaces = location.get_start() as usize + 2 + linestr.len();
        for _ in 0..spaces {
            underline.push(' ');
        }

        // underline itself
        for _ in location.get_start()..=location.get_end() {
            underline.push('~');
        }

        // error message
        underline.push_str(format!(" Error: {}\n", exc).as_str());

        buffer.push_str(&underline);
    }

    buffer
}

pub fn format_error(
    exc: &str,
    location: libgql::lexer::tokens::Location,
    source: &Rc<libgql::parsers::file::shared::ast::SourceFile>,
) -> String {
    let mut buffer = format!("{}\n", source.filepath.display());

    let line_num = location.get_line() as i32;

    let first_line_to_show = std::cmp::max(line_num - 4, 1) as u32;
    let last_line_to_show = (line_num + 4) as u32;

    let mut current_line: u32 = 1;

    for line in source.buffer.lines() {
        if first_line_to_show <= current_line
            && current_line <= last_line_to_show
        {
            buffer.push_str(&format_line(line, current_line, &location, exc));
        }

        current_line += 1;
    }

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
    registry: &mut libgql::parsers::schema::type_registry::TypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &config::InputsConfig,
) -> Result<libgql::parsers::schema::server::schema::Schema, Vec<String>> {
    let mut nodes = Vec::<libgql::parsers::file::server::ast::ASTNode>::new();
    let mut errors = Vec::<String>::new();
    let mut schema = libgql::parsers::schema::server::schema::Schema::default();
    for jsonpath in resolve_paths(config_dir_path, &conf.json_schema) {
        let buffer = std::fs::read_to_string(jsonpath).unwrap();
        let new_schema =
            libgql::json::parsers::schema::parse_server_schema(
                registry,
                serde_json_path_to_error::from_str::<
                    serde_json_path_to_error::Value,
                >(&buffer)
                .unwrap(),
            )
            .unwrap();
        schema.append_schema(new_schema);
    }
    for graphql_path in resolve_paths(&config_dir_path, &conf.graphql) {
        let buffer = std::fs::read_to_string(&graphql_path).unwrap();
        let source_file =
            std::rc::Rc::new(libgql::parsers::file::shared::ast::SourceFile {
                filepath: graphql_path.clone(),
                buffer,
            });
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
                errors.push(format_error(
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
    let new_schema =
        libgql::parsers::schema::server::parse_server_schema(registry, &nodes)
            .unwrap();
    schema.append_schema(new_schema);
    return Ok(schema);
}

pub fn load_client_schema_from_inputs(
    registry: &mut libgql::parsers::schema::type_registry::TypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &config::InputsConfig,
) -> Result<libgql::parsers::schema::client::schema::ClientSchema, Vec<String>>
{
    let mut nodes = Vec::<libgql::parsers::file::client::ast::ASTNode>::new();
    let mut errors = Vec::<String>::new();
    let mut schema =
        libgql::parsers::schema::client::schema::ClientSchema::default();
    for graphql_path in resolve_paths(&config_dir_path, &conf.graphql) {
        let buffer = std::fs::read_to_string(&graphql_path).unwrap();
        let source_file =
            std::rc::Rc::new(libgql::parsers::file::shared::ast::SourceFile {
                filepath: graphql_path.clone(),
                buffer,
            });
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
                errors.push(format_error(
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
    let new_schema =
        libgql::parsers::schema::client::parse_client_schema(registry, &nodes)
            .unwrap();
    schema.append_schema(new_schema);
    return Ok(schema);
}

pub fn run_config_action<'a>(
    config_path: &std::path::Path,
    config: &'a config::Config,
    json_callback: Box<dyn Fn(&str, &std::path::Path, &str) + 'a>,
) -> Result<(), String> {
    let mut registry =
        libgql::parsers::schema::type_registry::TypeRegistry::new();
    let server_schema = match load_server_schema_from_inputs(
        &mut registry,
        config_path.parent().unwrap(),
        &config.server.inputs,
    ) {
        Ok(schema) => schema,
        Err(errors) => {
            for e in errors {
                println!("{}", e);
            }
            return Ok(())
        }
    };
    let client_schema = config.client.as_ref().map(|client_config| {
        load_client_schema_from_inputs(
            &mut registry,
            config_path.parent().unwrap(),
            &client_config.inputs,
        )
        .unwrap()
    });
    if let Some(outputs) = config.server.outputs.as_ref() {
        let json_string =
            libgql::json::serializers::schema::serialize_server_schema(
                &server_schema,
                if outputs.only_used_in_operations {
                    client_schema.as_ref()
                } else {
                    None
                },
            )?;
        json_callback(&json_string, &outputs.filepath, "Server");
    };

    if let Some(client_config) = &config.client
        && let Some(outputs) = &client_config.outputs
        && let Some(c_schema) = &client_schema
    {
        let json_string =
            libgql::json::serializers::schema::serialize_client_schema(
                c_schema,
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
