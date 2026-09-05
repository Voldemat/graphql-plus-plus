use super::shared::{
    OpenBuffers, TokensToASTResult, buffer_to_client_ast, buffer_to_server_ast,
};

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
        .map(|file_path| {
            std::path::Path::new("./").join(
                file_path
                    .strip_prefix(&config_dir_path)
                    .unwrap_or(&file_path),
            )
        })
        .collect()
}

pub trait SchemaASTWrapper {
    type FileASTNode<'buffer>;
    type FileParserError<'buffer>: libgql::parsers::file::shared::error::Error;
    type SchemaParserError<'buffer>: libgql::parsers::schema::shared::error::Error<'buffer>;
    fn tokens_to_ast_nodes<'buffer>(
        source_file: &std::sync::Arc<
            libgql::parsers::file::shared::ast::SourceFile<'buffer>,
        >,
        tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        Self::FileASTNode<'buffer>,
        Self::FileParserError<'buffer>,
    >;
}

pub struct ServerSchemaASTWrapper {}
impl SchemaASTWrapper for ServerSchemaASTWrapper {
    type FileASTNode<'buffer> =
        libgql::parsers::file::server::ast::ASTNode<'buffer>;
    type FileParserError<'buffer> =
        libgql::parsers::file::server::Error<'buffer>;
    type SchemaParserError<'buffer> =
        libgql::parsers::schema::server::Error<'buffer>;
    fn tokens_to_ast_nodes<'buffer>(
        source_file: &std::sync::Arc<
            libgql::parsers::file::shared::ast::SourceFile<'buffer>,
        >,
        tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        Self::FileASTNode<'buffer>,
        Self::FileParserError<'buffer>,
    > {
        buffer_to_server_ast(source_file, tokens)
    }
}

pub struct ClientSchemaASTWrapper {}
impl SchemaASTWrapper for ClientSchemaASTWrapper {
    type FileASTNode<'buffer> =
        libgql::parsers::file::client::ast::ASTNode<'buffer>;
    type FileParserError<'buffer> =
        libgql::parsers::file::client::Error<'buffer>;
    type SchemaParserError<'buffer> =
        libgql::parsers::schema::client::errors::Error<'buffer, String>;
    fn tokens_to_ast_nodes<'buffer>(
        source_file: &std::sync::Arc<
            libgql::parsers::file::shared::ast::SourceFile<'buffer>,
        >,
        tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        Self::FileASTNode<'buffer>,
        Self::FileParserError<'buffer>,
    > {
        buffer_to_client_ast(source_file, tokens)
    }
}

pub trait SchemaErrorSerializer<TASTWrapper: SchemaASTWrapper> {
    type Error;

    fn lexer_error(
        source_file: &libgql::parsers::file::shared::ast::SourceFile,
        error: libgql::lexer::types::Error,
    ) -> Self::Error;
    fn file_parser_error(
        source_file: &libgql::parsers::file::shared::ast::SourceFile,
        error: TASTWrapper::FileParserError<'_>,
    ) -> Self::Error;
    fn schema_parser_error(
        error: TASTWrapper::SchemaParserError<'_>,
    ) -> Self::Error;
}

pub struct StringErrorSerializer {}

impl<TASTWrapper: SchemaASTWrapper> SchemaErrorSerializer<TASTWrapper>
    for StringErrorSerializer
{
    type Error = String;
    fn lexer_error(
        source_file: &libgql::parsers::file::shared::ast::SourceFile,
        error: libgql::lexer::types::Error,
    ) -> Self::Error {
        super::format_error::format_error(
            &error.to_string(),
            error.get_location(),
            &source_file.filepath,
            source_file.buffer,
        )
    }

    fn file_parser_error(
        source_file: &libgql::parsers::file::shared::ast::SourceFile,
        error: TASTWrapper::FileParserError<'_>,
    ) -> Self::Error {
        super::format_error::format_error(
            &error.to_string(),
            libgql::parsers::file::shared::error::Error::get_location(&error),
            &source_file.filepath,
            source_file.buffer,
        )
    }

    fn schema_parser_error(
        error: TASTWrapper::SchemaParserError<'_>,
    ) -> Self::Error {
        let location =
            libgql::parsers::schema::shared::error::Error::get_location(&error);
        super::format_error::format_error(
            &error.to_string(),
            &location.location,
            &location.source.filepath,
            location.source.buffer,
        )
    }
}

pub trait ParseSchemaFromNodes<'sr, TASTWrapper: SchemaASTWrapper> {
    fn parse<'buffer>(
        self: &mut Self,
        nodes: &[TASTWrapper::FileASTNode<'buffer>],
    ) -> Result<(), Vec<TASTWrapper::SchemaParserError<'buffer>>>
    where
        'sr: 'buffer;
}

pub struct ServerParseSchemaFromNodes<'r> {
    registry: &'r mut libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
}

impl<'r> ParseSchemaFromNodes<'r, ServerSchemaASTWrapper>
    for ServerParseSchemaFromNodes<'r>
{
    fn parse<'buffer>(
        self: &mut Self,
        nodes: &[<ServerSchemaASTWrapper as SchemaASTWrapper>::FileASTNode<
            'buffer,
        >],
    ) -> Result<
        (),
        Vec<
            <ServerSchemaASTWrapper as SchemaASTWrapper>::SchemaParserError<
                'buffer,
            >,
        >,
    >
    where
        'r: 'buffer,
    {
        libgql::parsers::schema::server::parse_server_schema(
            self.registry,
            nodes,
        )
        .map_err(|error| vec![error])
    }
}

pub struct ClientParseSchemaFromNodes<'sr, 'cr> {
    server_registry: &'sr libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    registry: &'cr mut libgql::parsers::schema::client::type_registry::TypeRegistry,
}

impl<'sr, 'cr> ParseSchemaFromNodes<'sr, ClientSchemaASTWrapper>
    for ClientParseSchemaFromNodes<'sr, 'cr>
{
    fn parse<'buffer>(
        self: &mut Self,
        nodes: &[<ClientSchemaASTWrapper as SchemaASTWrapper>::FileASTNode<
            'buffer,
        >],
    ) -> Result<
        (),
        Vec<
            <ClientSchemaASTWrapper as SchemaASTWrapper>::SchemaParserError<
                'buffer,
            >,
        >,
    >
    where
        'sr: 'buffer,
    {
        libgql::parsers::schema::client::parse_client_schema(
            self.server_registry,
            self.registry,
            nodes,
        )
    }
}

pub fn load_schema_from_graphql_inputs<
    'r,
    TASTWrapper: SchemaASTWrapper,
    TSchemaErrorSerializer: SchemaErrorSerializer<TASTWrapper>,
>(
    config_dir_path: &std::path::Path,
    path_patterns: &[std::path::PathBuf],
    open_buffers: &impl std::ops::Deref<Target = OpenBuffers>,
    mut parse_schema_from_nodes: impl ParseSchemaFromNodes<'r, TASTWrapper>,
) -> std::collections::HashMap<
    std::path::PathBuf,
    Vec<TSchemaErrorSerializer::Error>,
> {
    let mut buffers =
        std::collections::HashMap::<std::path::PathBuf, String>::new();
    let graphql_paths = resolve_paths(&config_dir_path, path_patterns);
    for graphql_path in graphql_paths.iter().filter_map(|graphql_path| {
        if open_buffers.contains_key(graphql_path) {
            None
        } else {
            Some(graphql_path)
        }
    }) {
        buffers.insert(
            graphql_path.clone(),
            std::fs::read_to_string(graphql_path).unwrap(),
        );
    }
    let mut source_files = Vec::<
        std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile>,
    >::new();
    let mut tokens = Vec::<Vec<libgql::lexer::tokens::Token>>::new();
    let mut file_errors = std::collections::HashMap::<
        std::path::PathBuf,
        Vec<TSchemaErrorSerializer::Error>,
    >::new();
    for graphql_path in graphql_paths {
        let buffer = open_buffers
            .get(&graphql_path)
            .map(|buffer_ref| buffer_ref.content.as_str())
            .unwrap_or_else(|| buffers.get(&graphql_path).unwrap().as_str());
        let parse_result = libgql::lexer::utils::parse_buffer(buffer);
        let source_file = std::sync::Arc::new(
            libgql::parsers::file::shared::ast::SourceFile {
                filepath: graphql_path.clone(),
                buffer,
                new_line_positions: parse_result.new_line_positions,
            },
        );
        if !file_errors.contains_key(&graphql_path) {
            file_errors.insert(graphql_path.clone(), Vec::new());
        };
        let errors = file_errors.get_mut(&graphql_path).unwrap();
        errors.extend(parse_result.errors.into_iter().map(|lexer_error| {
            TSchemaErrorSerializer::lexer_error(&source_file, lexer_error)
        }));
        tokens.push(parse_result.tokens);
        source_files.push(source_file);
    }
    let mut nodes = Vec::<TASTWrapper::FileASTNode<'_>>::new();
    for (source_file, tokens) in source_files.iter().zip(tokens) {
        let result = TASTWrapper::tokens_to_ast_nodes(source_file, tokens);
        nodes.extend(result.ast_nodes);
        if !file_errors.contains_key(&source_file.filepath) {
            file_errors.insert(source_file.filepath.clone(), Vec::new());
        };
        let errors = file_errors.get_mut(&source_file.filepath).unwrap();
        errors.extend(result.parser_errors.into_iter().map(|parser_error| {
            TSchemaErrorSerializer::file_parser_error(source_file, parser_error)
        }));
    }
    for schema_parser_error in parse_schema_from_nodes
        .parse(&nodes)
        .err()
        .unwrap_or(Vec::new())
    {
        let location =
            libgql::parsers::schema::shared::error::Error::get_location(
                &schema_parser_error,
            );
        file_errors
            .get_mut(&location.source.filepath)
            .unwrap()
            .push(TSchemaErrorSerializer::schema_parser_error(
                schema_parser_error,
            ));
    }
    file_errors
}

pub fn load_server_schema_from_inputs<
    TSchemaErrorSerializer: SchemaErrorSerializer<ServerSchemaASTWrapper>,
>(
    registry: &mut libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &super::config::InputsConfig,
    open_buffers: &impl std::ops::Deref<Target = OpenBuffers>,
) -> std::collections::HashMap<
    std::path::PathBuf,
    Vec<TSchemaErrorSerializer::Error>,
> {
    for jsonpath in resolve_paths(config_dir_path, &conf.json_schema) {
        let buffer = std::fs::read_to_string(jsonpath).unwrap();
        libgql::json::parsers::schema::parse_server_schema(
            registry,
            serde_json::from_str::<serde_json::Value>(&buffer).unwrap(),
        )
        .unwrap();
    }
    load_schema_from_graphql_inputs::<
        ServerSchemaASTWrapper,
        TSchemaErrorSerializer,
    >(
        config_dir_path,
        &conf.graphql,
        open_buffers,
        ServerParseSchemaFromNodes { registry },
    )
}

pub fn load_client_schema_from_inputs<
    TSchemaErrorSerializer: SchemaErrorSerializer<ClientSchemaASTWrapper>,
>(
    server_registry: &libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    registry: &mut libgql::parsers::schema::client::type_registry::TypeRegistry,
    config_dir_path: &std::path::Path,
    conf: &super::config::InputsConfig,
    open_buffers: &impl std::ops::Deref<Target = OpenBuffers>,
) -> std::collections::HashMap<
    std::path::PathBuf,
    Vec<TSchemaErrorSerializer::Error>,
> {
    load_schema_from_graphql_inputs::<
        ClientSchemaASTWrapper,
        TSchemaErrorSerializer,
    >(
        config_dir_path,
        &conf.graphql,
        open_buffers,
        ClientParseSchemaFromNodes {
            server_registry,
            registry,
        },
    )
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
    let server_errors = load_server_schema_from_inputs::<StringErrorSerializer>(
        &mut server_registry,
        config_path.parent().unwrap(),
        &config_server.inputs,
        &&Default::default(),
    );
    if server_errors.len() > 0 {
        for e in server_errors.values().flatten() {
            println!("{}", e);
        }
        return Ok(());
    }
    let client_registry = match config.client.as_ref().map(|client_config| {
        let mut c_registry =
            libgql::parsers::schema::client::type_registry::TypeRegistry::new();
        let client_errors =
            load_client_schema_from_inputs::<StringErrorSerializer>(
                &server_registry,
                &mut c_registry,
                config_path.parent().unwrap(),
                &client_config.inputs,
                &&Default::default(),
            );
        if client_errors.len() == 0 {
            return Some(c_registry);
        };
        for e in client_errors.values().flatten() {
            println!("{}", e);
        }
        return None;
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
