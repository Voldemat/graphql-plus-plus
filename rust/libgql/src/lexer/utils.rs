use super::{
    Lexer,
    tokens::Token,
    types::{Error, LexerSuccessTokenResult},
};

pub fn parse_buffer_into_tokens<'buffer>(
    buffer: &'buffer str,
) -> Result<Vec<Token<'buffer>>, Vec<Error>> {
    let mut lexer = Lexer::new(buffer);
    let mut tokens: Vec<Token<'buffer>> = Vec::new();
    let mut errors: Vec<crate::lexer::Error> = Vec::new();
    for c in buffer.chars() {
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
    if errors.len() > 0 {
        return Err(errors);
    };
    if let Some(last_token) = lexer.maybe_extract_token() {
        tokens.push(last_token)
    }
    return Ok(tokens);
}
