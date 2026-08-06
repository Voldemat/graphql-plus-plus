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

    #[test]
    fn test_float() {
        let input = "type Query {\ntest(arg: Float! = 0.1): Int!\n}";

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
                    lexeme: "type",
                    location: TokenLocation { start: 0, end: 3 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "Query",
                    location: TokenLocation { start: 5, end: 9 }
                },
                Token {
                    token_type: SimpleTokenType::LeftBrace.into(),
                    lexeme: "{",
                    location: TokenLocation { start: 11, end: 11 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "check",
                    location: TokenLocation { start: 13, end: 16 }
                },
                Token {
                    token_type: SimpleTokenType::LeftParen.into(),
                    lexeme: "(",
                    location: TokenLocation { start: 17, end: 17 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "arg",
                    location: TokenLocation { start: 18, end: 20 }
                },
                Token {
                    token_type: SimpleTokenType::Colon.into(),
                    lexeme: ":",
                    location: TokenLocation { start: 21, end: 21 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "Float",
                    location: TokenLocation { start: 23, end: 27 }
                },
                Token {
                    token_type: SimpleTokenType::Bang.into(),
                    lexeme: "!",
                    location: TokenLocation { start: 28, end: 28 }
                },
                Token {
                    token_type: SimpleTokenType::Equal.into(),
                    lexeme: "=",
                    location: TokenLocation { start: 30, end: 30 }
                },
                Token {
                    token_type: ComplexTokenType::Number.into(),
                    lexeme: "0.1",
                    location: TokenLocation { start: 32, end: 34 }
                },
                Token {
                    token_type: SimpleTokenType::RightParen.into(),
                    lexeme: ")",
                    location: TokenLocation { start: 35, end: 35 }
                },
                Token {
                    token_type: SimpleTokenType::Colon.into(),
                    lexeme: ":",
                    location: TokenLocation { start: 36, end: 36 }
                },
                Token {
                    token_type: ComplexTokenType::Identifier.into(),
                    lexeme: "Int",
                    location: TokenLocation { start: 38, end: 40 }
                },
                Token {
                    token_type: SimpleTokenType::Bang.into(),
                    lexeme: "!",
                    location: TokenLocation { start: 41, end: 41 }
                },
                Token {
                    token_type: SimpleTokenType::RightBrace.into(),
                    lexeme: "}",
                    location: TokenLocation { start: 43, end: 43 }
                },
            ]
        );
    }
}
