#[cfg(test)]
mod tests {
    use crate::lexer::{
        ComplexTokenType, Lexer,
        token_type::SimpleTokenType,
        tokens::{Token, TokenLocation},
        types::{Error, LexerSuccessTokenResult},
    };

    #[test]
    fn test_lexer() {
        let input = "fragment ProductFragment(\"something\") {\ninternal {\n...ProductInternalFragment\n}\n}";

        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();
        for c in input.chars() {
            match lexer.feed(c) {
                Ok(result) => {
                    let Some(r) = result.0 else {
                        continue;
                    };
                    match r {
                        LexerSuccessTokenResult::One(t) => {
                            println!("LexerSuccessTokenResult::One({:?})", t);
                            tokens.push(t)
                        }
                        LexerSuccessTokenResult::Two(t1, t2) => {
                            println!(
                                "LexerSuccessTokenResult::Two({:?}, {:?})",
                                t1, t2
                            );
                            tokens.push(t1);
                            tokens.push(t2);
                        }
                    }
                }
                Err(error) => errors.push(error),
            }
        }
        println!("{:?}", tokens);
        pretty_assertions::assert_eq!(
            errors,
            Vec::<crate::lexer::Error>::new()
        );
        pretty_assertions::assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "fragment",
                    location: TokenLocation { start: 0, end: 7 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "ProductFragment",
                    location: TokenLocation { start: 9, end: 23 }
                },
                Token {
                    token_type: SimpleTokenType::LeftParen.into(),
                    lexeme: "(",
                    location: TokenLocation { start: 24, end: 24 }
                },
                Token {
                    token_type: ComplexTokenType::String.into(),
                    lexeme: "something",
                    location: TokenLocation { start: 25, end: 35 }
                },
                Token {
                    token_type: SimpleTokenType::RightParen.into(),
                    lexeme: ")",
                    location: TokenLocation { start: 36, end: 36 }
                },
                Token {
                    token_type: SimpleTokenType::LeftBrace.into(),
                    lexeme: "{",
                    location: TokenLocation { start: 38, end: 38 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "internal",
                    location: TokenLocation { start: 40, end: 47 }
                },
                Token {
                    token_type: SimpleTokenType::LeftBrace.into(),
                    lexeme: "{",
                    location: TokenLocation { start: 49, end: 49 }
                },
                Token {
                    token_type: ComplexTokenType::Spread.into(),
                    lexeme: "...",
                    location: TokenLocation { start: 51, end: 53 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "ProductInternalFragment",
                    location: TokenLocation { start: 54, end: 76 }
                },
                Token {
                    token_type: SimpleTokenType::RightBrace.into(),
                    lexeme: "}",
                    location: TokenLocation { start: 78, end: 78 }
                },
                Token {
                    token_type: SimpleTokenType::RightBrace.into(),
                    lexeme: "}",
                    location: TokenLocation { start: 80, end: 80 }
                },
            ]
        );
    }
}
