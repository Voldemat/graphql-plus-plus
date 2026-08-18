pub fn get_diff_string(
    initial_string: &str,
    formatted_string: &str,
) -> Option<String> {
    let text_diff =
        similar::TextDiff::from_lines(initial_string, formatted_string);
    if text_diff.ratio() == 1.0 {
        return None;
    }
    let mut buffer = String::new();
    for hunk in text_diff.grouped_ops(3) {
        for op in hunk {
            for change in text_diff.iter_changes(&op) {
                let old_ln = change
                    .old_index()
                    .map(|idx| (idx + 1).to_string())
                    .unwrap_or_default();
                let new_ln = change
                    .new_index()
                    .map(|idx| (idx + 1).to_string())
                    .unwrap_or_default();
                match change.tag() {
                    similar::ChangeTag::Delete => {
                        buffer += &format!(
                            "{:3} {:3} | {}",
                            console::style(&old_ln).red(),
                            console::style(&new_ln).dim(),
                            console::style(format!("- {}", change)).red()
                        );
                    }
                    similar::ChangeTag::Insert => {
                        buffer += &format!(
                            "{:3} {:3} | {}",
                            console::style(&old_ln).dim(),
                            console::style(&new_ln).green(),
                            console::style(format!("+ {}", change)).green()
                        );
                    }
                    similar::ChangeTag::Equal => {
                        buffer += &format!(
                            "{:3} {:3} |  {}",
                            console::style(&old_ln).dim(),
                            console::style(&new_ln).dim(),
                            change
                        );
                    }
                }
            }
        }
    }
    Some(buffer)
}
