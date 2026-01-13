use super::token_type::ComplexTokenType;

pub type Condition = fn(char, &str) -> bool;

fn string_condition(c: char, _: &str) -> bool {
    return c != '"';
}

fn number_condition(c: char, buffer: &str) -> bool {
    let last_char = buffer.chars().last().unwrap();
    let has_fchar = last_char == 'f';
    if has_fchar {
        return false;
    }
    let is_digit = c.is_digit(10);
    let has_point = buffer.contains('.');
    if has_point && is_digit {
        return true;
    };
    let last_char_is_digit = last_char.is_digit(10);
    let is_char = c == '.' || c == 'f';
    return is_digit || (last_char_is_digit && is_char);
}

fn spread_condition(c: char, _: &str) -> bool {
    return c == '.';
}

fn identifier_condition(c: char, _: &str) -> bool {
    return c.is_alphabetic() || c.is_digit(10) || c == '_' || c == '-';
}

pub fn get_condition_for_token_type(token_type: ComplexTokenType) -> Condition {
    match token_type {
        ComplexTokenType::String => string_condition,
        ComplexTokenType::Number => number_condition,
        ComplexTokenType::Spread => spread_condition,
        ComplexTokenType::Boolean | ComplexTokenType::Identifier => {
            identifier_condition
        }
    }
}
