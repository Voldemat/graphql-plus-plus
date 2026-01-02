use std::{fs, rc::Rc};

use libgql::{
    lexer::{tokens::Location, utils::parse_buffer_into_tokens},
    parsers::{
        file::{
            self, shared::ast::SourceFile, tokens_sources::VecTokensSource,
        },
        schema::{server::parse_server_schema, type_registry::TypeRegistry},
    },
};

pub fn format_line(
    line: &str,
    current_line: u32,
    location: &Location,
    exc: &file::server::Error,
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
        underline.push_str(format!(" Error: {:?}\n", exc).as_str());

        buffer.push_str(&underline);
    }

    buffer
}

pub fn format_error(
    exc: &file::server::Error,
    source: Rc<SourceFile>,
) -> String {
    let location = exc.get_location();

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
    let mut tmp = glob::glob("./graphql/**/*.graphql").into_iter();
    let mut nodes = Vec::<file::server::ast::ASTNode>::new();
    while let Some(filepaths) = tmp.next() {
        for filepath in filepaths.map(|f| f.unwrap()) {
            let buffer = fs::read_to_string(&filepath).unwrap();
            let tokens = parse_buffer_into_tokens(buffer.as_str()).unwrap();
            println!("{}", filepath.to_str().unwrap());
            println!("{}", buffer);
            println!("{:?}", tokens);
            let source_file = Rc::new(SourceFile { filepath, buffer });
            let tokens_source =
                VecTokensSource::new(tokens, source_file.clone());
            let mut local_nodes =
                match libgql::parsers::file::server::Parser::new(tokens_source)
                    .parse_ast_nodes()
                {
                    Ok(n) => n,
                    Err(e) => {
                        println!("{}", format_error(&e, source_file));
                        panic!("something");
                    },
                };
            nodes.append(&mut local_nodes);
        }
    }
    let mut registry = TypeRegistry::new();
    let schema = parse_server_schema(&mut registry, &nodes).unwrap();
    println!("{:?}", schema);
}
