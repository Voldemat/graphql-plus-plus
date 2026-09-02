pub fn generate(old_text: &str, new_text: &str) -> Vec<lsp_types::TextEdit> {
    let diff = similar::TextDiff::from_lines(old_text, new_text);
    let mut edits = Vec::new();

    // Grouping operations isolates clusters of changes for efficient edits
    for hunk in diff.grouped_ops(3) {
        for op in hunk {
            match op {
                similar::DiffOp::Equal { .. } => {}
                similar::DiffOp::Delete {
                    old_index, old_len, ..
                } => {
                    edits.push(lsp_types::TextEdit {
                        range: lsp_types::Range::new(
                            lsp_types::Position::new(old_index as u32, 0),
                            lsp_types::Position::new(
                                (old_index + old_len) as u32,
                                0,
                            ),
                        ),
                        new_text: String::new(),
                    });
                }
                similar::DiffOp::Insert {
                    old_index,
                    new_index,
                    new_len,
                } => {
                    let text: String = new_text
                        .lines()
                        .skip(new_index)
                        .take(new_len)
                        .map(|l| format!("{l}\n"))
                        .collect(); // careful with final newline / CRLF
                    edits.push(lsp_types::TextEdit {
                        range: lsp_types::Range::new(
                            lsp_types::Position::new(old_index as u32, 0),
                            lsp_types::Position::new(old_index as u32, 0),
                        ),
                        new_text: text,
                    });
                }
                similar::DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    let text: String = new_text
                        .lines()
                        .skip(new_index)
                        .take(new_len)
                        .map(|l| format!("{l}\n"))
                        .collect();
                    edits.push(lsp_types::TextEdit {
                        range: lsp_types::Range::new(
                            lsp_types::Position::new(old_index as u32, 0),
                            lsp_types::Position::new(
                                (old_index + old_len) as u32,
                                0,
                            ),
                        ),
                        new_text: text,
                    });
                }
            }
        }
    }

    edits
}
