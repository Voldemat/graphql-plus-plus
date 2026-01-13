use crate::lexer::{Error, Lexer, tokens::Token};

pub fn parse_buffer_into_tokens(
    buffer: &str,
) -> Result<Vec<Token>, Vec<Error>> {
    let mut lexer = Lexer::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<crate::lexer::Error> = Vec::new();
    for c in buffer.chars() {
        match lexer.feed(c) {
            Ok(result) => {
                let Some(r) = result.0 else {
                    continue;
                };
                match r {
                    crate::lexer::LexerSuccessTokenResult::One(t) => {
                        tokens.push(t)
                    }
                    crate::lexer::LexerSuccessTokenResult::Two(t1, t2) => {
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
    return Ok(tokens);
}
