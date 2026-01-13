use crate::{lexer, parsers::file};

fn get_source_text(
    source_buffer: &str,
    start_token_location: &lexer::tokens::Location,
    end_token_location: &lexer::tokens::Location,
) -> String {
    let mut buffer = String::new();
    let mut current_line: u32 = 1;

    for line in source_buffer.lines() {
        if start_token_location.get_line() == current_line {
            let start = start_token_location.get_start();
            buffer.push_str(&line[(start as usize)..]);
        }

        if end_token_location.get_line() == current_line {
            let start = end_token_location.get_start();
            let end = end_token_location.get_end() + 1;
            buffer.push_str(&line[(start as usize)..(end as usize)]);
        }

        if current_line > start_token_location.get_line()
            && current_line < end_token_location.get_line()
        {
            buffer.push_str(line);
        }

        current_line += 1;
    }

    // Equivalent of std::unique removing consecutive spaces
    let mut deduped = String::with_capacity(buffer.len());
    let mut prev_space = false;

    for ch in buffer.chars() {
        match (ch, prev_space) {
            (' ', true) => {}
            (' ', false) => {
                deduped.push(ch);
                prev_space = true;
            }
            _ => {
                deduped.push(ch);
                prev_space = false;
            }
        }
    }

    deduped
}

pub fn extract_from_fragment(
    node: &file::client::ast::FragmentDefinition,
) -> String {
    get_source_text(
        &node.location.source.buffer,
        &node.location.start_token.location,
        &node.location.end_token.location,
    )
}

pub fn extract_from_operation(
    node: &file::client::ast::OperationDefinition,
) -> String {
    get_source_text(
        &node.location.source.buffer,
        &node.location.start_token.location,
        &node.location.end_token.location,
    )
}
