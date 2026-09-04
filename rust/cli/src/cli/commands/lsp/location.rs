use libgql::lexer::tokens::TokenLocation;

pub fn index_to_position(
    new_line_positions: &[usize],
    index: usize,
) -> lsp_types::Position {
    match new_line_positions.binary_search(&index) {
        Ok(line) => {
            let col = if line == 0 {
                index
            } else {
                index - new_line_positions[line - 1] - 1
            };
            lsp_types::Position {
                line: line as u32,
                character: col as u32,
            }
        }
        Err(line) => {
            let col = if line == 0 {
                index
            } else {
                index - new_line_positions[line - 1] - 1
            };
            lsp_types::Position {
                line: line as u32,
                character: col as u32,
            }
        }
    }
}

pub fn token_location_to_range(
    new_line_positions: &[usize],
    location: &TokenLocation,
) -> lsp_types::Range {
    lsp_types::Range {
        start: index_to_position(new_line_positions, location.start),
        end: index_to_position(new_line_positions, location.end),
    }
}

pub fn lsp_position_to_index(
    new_line_positions: &[usize],
    lsp_position: &lsp_types::Position,
) -> usize {
    if lsp_position.line == 0 {
        return lsp_position.character as usize;
    };
    let line_start_index =
        new_line_positions[lsp_position.line as usize - 1] + 1;
    let character_offset = lsp_position.character as usize;

    line_start_index + character_offset
}

pub fn lsp_range_to_index_range(
    new_line_positions: &[usize],
    lsp_range: lsp_types::Range,
) -> std::ops::Range<usize> {
    lsp_position_to_index(new_line_positions, &lsp_range.start)
        ..lsp_position_to_index(new_line_positions, &lsp_range.end)
}
