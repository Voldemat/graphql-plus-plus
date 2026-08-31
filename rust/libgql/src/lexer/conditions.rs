use super::token_type::ComplexTokenType;

#[derive(Debug, PartialEq, Eq)]
pub enum ConditionResult {
    True,
    False { is_char_part_of_token: bool },
    UnexpectedChar,
}

pub trait Condition {
    fn evaluate(self: &mut Self, c: char) -> ConditionResult;
}

#[derive(Debug, Default)]
pub struct StringCondition {}

impl Condition for StringCondition {
    fn evaluate(self: &mut Self, c: char) -> ConditionResult {
        if c != '"' {
            ConditionResult::True
        } else {
            ConditionResult::False {
                is_char_part_of_token: true,
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct MultilineStringCondition {
    pub consecutive_quote_count: u8,
}

impl Condition for MultilineStringCondition {
    fn evaluate(self: &mut Self, c: char) -> ConditionResult {
        if c == '"' && self.consecutive_quote_count == 2 {
            return ConditionResult::False {
                is_char_part_of_token: true,
            };
        };
        if c == '"' {
            self.consecutive_quote_count += 1;
        } else {
            self.consecutive_quote_count = 0;
        };
        ConditionResult::True
    }
}

#[derive(Debug)]
pub struct NumberCondition {
    pub contains_point: bool,
    pub last_char_is_digit: bool,
}

impl Default for NumberCondition {
    fn default() -> Self {
        Self {
            contains_point: false,
            last_char_is_digit: true,
        }
    }
}

impl Condition for NumberCondition {
    fn evaluate(self: &mut Self, c: char) -> ConditionResult {
        if c == 'f' {
            return ConditionResult::False {
                is_char_part_of_token: true,
            };
        };
        if c == '.' {
            if self.contains_point {
                return ConditionResult::False {
                    is_char_part_of_token: false,
                };
            } else {
                self.contains_point = true;
                return ConditionResult::True;
            }
        }
        if c.is_digit(10) {
            ConditionResult::True
        } else {
            ConditionResult::False {
                is_char_part_of_token: false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_condition() {
        let mut condition = NumberCondition::default();
        assert_eq!(condition.evaluate('0'), ConditionResult::True);
        assert_eq!(condition.evaluate('.'), ConditionResult::True);
        assert_eq!(
            condition.evaluate('.'),
            ConditionResult::False {
                is_char_part_of_token: false
            }
        );
    }
}

#[derive(Debug)]
pub struct SpreadCondition {
    pub dot_count: u8,
}

impl Default for SpreadCondition {
    fn default() -> Self {
        Self { dot_count: 1 }
    }
}

impl Condition for SpreadCondition {
    fn evaluate(self: &mut Self, c: char) -> ConditionResult {
        if c == '.' {
            if self.dot_count == 2 {
                ConditionResult::False {
                    is_char_part_of_token: true,
                }
            } else {
                self.dot_count += 1;
                ConditionResult::True
            }
        } else {
            ConditionResult::UnexpectedChar
        }
    }
}

#[derive(Debug, Default)]
pub struct IdentifierCondition {}

impl Condition for IdentifierCondition {
    fn evaluate(self: &mut Self, c: char) -> ConditionResult {
        if c.is_alphabetic() || c.is_digit(10) || c == '_' || c == '-' {
            ConditionResult::True
        } else {
            ConditionResult::False {
                is_char_part_of_token: false,
            }
        }
    }
}

pub fn get_condition_for_token_type(
    token_type: ComplexTokenType,
) -> Box<dyn Condition> {
    match token_type {
        ComplexTokenType::String => Box::new(StringCondition::default()),
        ComplexTokenType::MultilineString => {
            Box::new(MultilineStringCondition::default())
        }
        ComplexTokenType::Number => Box::new(NumberCondition::default()),
        ComplexTokenType::Spread => Box::new(SpreadCondition::default()),
        ComplexTokenType::Boolean | ComplexTokenType::Identifier => {
            Box::new(IdentifierCondition::default())
        }
    }
}
