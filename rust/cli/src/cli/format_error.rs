const CONTEXT_LINES: usize = 5;
pub fn format_error<'buffer>(
    exc: &str,
    location: &libgql::lexer::tokens::TokenLocation,
    source_filepath: &std::path::Path,
    source_buffer: &str,
) -> String {
    let buffer = source_buffer;

    // Bounds checking to prevent slicing panics
    let start = location.start.min(buffer.len());
    let end = location.end.min(buffer.len()).max(start);

    // Calculate error line index (0-indexed)
    let target_line_idx = buffer[..start].lines().count().saturating_sub(1);

    // Collect lines to extract context ranges safely
    let lines: Vec<&str> = buffer.lines().collect();
    if lines.is_empty() {
        return format!("error: {}\n --> {}\n", exc, source_filepath.display());
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
        source_filepath.display(),
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
