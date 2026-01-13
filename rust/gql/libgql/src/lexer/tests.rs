#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use crate::lexer::{
        token_type::SimpleTokenType, tokens::{Location, Token}, ComplexTokenType, Lexer
    };

    #[test]
    fn test_lexer() {
        let input = "fragment ProductFragment {\ninternal {\n...ProductInternalFragment\n}\n}";
        let mut lexer = Lexer::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<crate::lexer::Error> = Vec::new();
        for c in input.chars() {
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
        println!("{:?}", tokens);
        assert_eq!(errors, Vec::<crate::lexer::Error>::new());
        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "fragment".into(),
                    location: Location::new(1, Wrapping(0), Wrapping(7))
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "ProductFragment".into(),
                    location: Location::new(1, Wrapping(9), Wrapping(23))
                },
                Token {
                    token_type: SimpleTokenType::LeftBrace.into(),
                    lexeme: "{".into(),
                    location: Location::new(1, Wrapping(25), Wrapping(25))
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "internal".into(),
                    location: Location::new(2, Wrapping(0), Wrapping(7))
                },
                Token {
                    token_type: SimpleTokenType::LeftBrace.into(),
                    lexeme: "{".into(),
                    location: Location::new(2, Wrapping(9), Wrapping(9))
                },
                Token {
                    token_type: ComplexTokenType::Spread.into(),
                    lexeme: "...".into(),
                    location: Location::new(3, Wrapping(0), Wrapping(2))
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "ProductInternalFragment".into(),
                    location: Location::new(3, Wrapping(3), Wrapping(25))
                },
                Token {
                    token_type: SimpleTokenType::RightBrace.into(),
                    lexeme: "}".into(),
                    location: Location::new(4, Wrapping(0), Wrapping(0))
                },
                Token {
                    token_type: SimpleTokenType::RightBrace.into(),
                    lexeme: "}".into(),
                    location: Location::new(5, Wrapping(0), Wrapping(0))
                },
            ]
        );
    }
}
