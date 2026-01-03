use std::{fs, rc::Rc};

use libgql::{
    lexer::{tokens::Location, utils::parse_buffer_into_tokens},
    parsers::{
        file::{
            self,
            shared::ast::{NodeLocation, SourceFile},
            tokens_sources::VecTokensSource,
        },
        schema::{
            client::parse_client_schema, server::parse_server_schema,
            type_registry::TypeRegistry,
        },
    },
};

pub fn format_line(
    line: &str,
    current_line: u32,
    location: &Location,
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
    location: Location,
    source: Rc<SourceFile>,
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

fn main() {
    let mut server_nodes = Vec::<file::server::ast::ASTNode>::new();
    let mut client_nodes = Vec::<file::client::ast::ASTNode>::new();
    let mut server_files = glob::glob("./graphql/**/*.graphql").into_iter();
    while let Some(filepaths) = server_files.next() {
        for filepath in filepaths.map(|f| f.unwrap()) {
            let buffer = fs::read_to_string(&filepath).unwrap();
            println!("{} read file", filepath.to_string_lossy());
            let tokens = parse_buffer_into_tokens(buffer.as_str()).unwrap();
            let source_file = Rc::new(SourceFile { filepath, buffer });
            let tokens_source =
                VecTokensSource::new(tokens, source_file.clone());
            let mut local_nodes =
                match libgql::parsers::file::server::Parser::new(tokens_source)
                    .parse_ast_nodes()
                {
                    Ok(n) => n,
                    Err(e) => {
                        println!(
                            "{}",
                            format_error(
                                format!("{:?}", e).as_str(),
                                e.get_location(),
                                source_file
                            )
                        );
                        panic!("something");
                    }
                };
            server_nodes.append(&mut local_nodes);
        }
    }
    let mut client_files = glob::glob("./client/**/*.graphql").into_iter();
    while let Some(filepaths) = client_files.next() {
        for filepath in filepaths.map(|f| f.unwrap()) {
            let buffer = fs::read_to_string(&filepath).unwrap();
            println!("{} read file", filepath.to_string_lossy());
            let tokens = parse_buffer_into_tokens(buffer.as_str()).unwrap();
            let source_file = Rc::new(SourceFile { filepath, buffer });
            let tokens_source =
                VecTokensSource::new(tokens, source_file.clone());
            let mut local_nodes =
                match libgql::parsers::file::client::Parser::new(tokens_source)
                    .parse_ast_nodes()
                {
                    Ok(n) => n,
                    Err(e) => {
                        println!(
                            "{}",
                            format_error(
                                format!("{:?}", e).as_str(),
                                e.get_location(),
                                source_file
                            )
                        );
                        panic!("something");
                    }
                };
            client_nodes.append(&mut local_nodes);
        }
    }
    let mut registry = TypeRegistry::new();
    let server_schema =
        parse_server_schema(&mut registry, &server_nodes).unwrap();
    println!("{:?}", server_schema);
    let client_schema =
        parse_client_schema(&mut registry, &client_nodes).unwrap();
    println!("{:?}", client_schema);
}
