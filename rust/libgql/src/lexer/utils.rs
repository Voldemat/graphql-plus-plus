use super::{
    Lexer,
    tokens::Token,
    types::{Error, LexerSuccessTokenResult},
};

pub type NewLinePositions = Vec<usize>;

pub struct ParseBufferResult<'buffer> {
    pub new_line_positions: NewLinePositions,
    pub tokens: Vec<Token<'buffer>>,
    pub errors: Vec<Error>,
}

pub fn parse_buffer<'buffer>(
    buffer: &'buffer str,
) -> ParseBufferResult<'buffer> {
    let mut lexer = Lexer::new(buffer);
    let mut new_line_positions = NewLinePositions::new();
    let mut tokens: Vec<Token<'buffer>> = Vec::new();
    let mut errors: Vec<crate::lexer::Error> = Vec::new();
    for (index, c) in buffer.chars().enumerate() {
        if c == '\n' {
            new_line_positions.push(index);
        }
        match lexer.feed(c) {
            Ok(result) => {
                let Some(r) = result.0 else {
                    continue;
                };
                match r {
                    LexerSuccessTokenResult::One(t) => tokens.push(t),
                    LexerSuccessTokenResult::Two(t1, t2) => {
                        tokens.push(t1);
                        tokens.push(t2);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }
    if let Some(last_token) = lexer.maybe_extract_token() {
        tokens.push(last_token)
    }
    ParseBufferResult {
        new_line_positions,
        tokens,
        errors,
    }
}
