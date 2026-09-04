use std::sync::Arc;

const CONTEXT_LINES: usize = 5;
pub fn format_error_with_range<'buffer>(
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

pub fn format_parse_error<'buffer>(
    exc: &str,
    location: &libgql::lexer::tokens::TokenLocation,
    source: &Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
) -> String {
    format_error_with_range(exc, location.start, location.end, source)
}

pub fn format_server_schema_error(
    error: libgql::parsers::schema::server::Error<'_>,
) -> String {
    let node_location = error.get_location();
    format_error_with_range(
        &format!("{error}"),
        node_location.location.start,
        node_location.location.end,
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
        node_location.location.start,
        node_location.location.end,
        &node_location.source,
    )
}
