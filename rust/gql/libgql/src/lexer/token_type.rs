#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleTokenType {
    Equal,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Bang,
    Semicolon,
    Colon,
    Comma,
    Vslash,
    LeftBracket,
    RightBracket,
    AtSign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexTokenType {
    Identifier,
    String,
    Number,
    Boolean,
    Spread,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::From)]
pub enum TokenType {
    Simple(SimpleTokenType),
    Complex(ComplexTokenType),
}

impl TryFrom<char> for TokenType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_alphabetic() || value == '_' {
            return Ok(ComplexTokenType::Identifier.into());
        }
        if value.is_digit(10) {
            return Ok(ComplexTokenType::Number.into());
        }
        match value {
            '!' => Ok(SimpleTokenType::Bang.into()),
            '=' => Ok(SimpleTokenType::Equal.into()),
            '(' => Ok(SimpleTokenType::LeftParen.into()),
            ')' => Ok(SimpleTokenType::RightParen.into()),
            '{' => Ok(SimpleTokenType::LeftBrace.into()),
            '}' => Ok(SimpleTokenType::RightBrace.into()),
            ';' => Ok(SimpleTokenType::Semicolon.into()),
            ':' => Ok(SimpleTokenType::Colon.into()),
            ',' => Ok(SimpleTokenType::Comma.into()),
            '|' => Ok(SimpleTokenType::Vslash.into()),
            '[' => Ok(SimpleTokenType::LeftBracket.into()),
            ']' => Ok(SimpleTokenType::RightBracket.into()),
            '@' => Ok(SimpleTokenType::AtSign.into()),
            '"' => Ok(ComplexTokenType::String.into()),
            '.' => Ok(ComplexTokenType::Spread.into()),
            '$' => Ok(ComplexTokenType::Identifier.into()),
            _ => Err(()),
        }
    }
}
