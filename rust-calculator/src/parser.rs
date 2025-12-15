//! Parsing and evaluation logic for the calculator.

use std::sync::LazyLock;

use regex::Regex;

/// Represents a math operator in the calculator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Represents either a number or an operator.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Operator(Operator),
}

// Matches the full input for validity (numbers and operators in alternating order).
const INPUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?:(?<number>\d+)\s*(?<operator>[+\-*/]))\s*)*(?:(?<last_number>\d+))$")
        .unwrap()
});

// Extracts every number or operator in order.
const TOKEN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+|[+\-*/]").expect("Invalid token regex"));

/// Parses the user input into a sequence of tokens.
pub fn parse_input(input: &str) -> Result<Vec<Token>, &'static str> {
    let trimmed = input.trim();
    if !INPUT_REGEX.is_match(trimmed) {
        return Err("Invalid input: only positive numbers and + - * / are allowed");
    }

    let tokens = TOKEN_REGEX
        .find_iter(trimmed)
        .map(|m| m.as_str())
        .map(|raw| match raw {
            "+" => Ok(Token::Operator(Operator::Add)),
            "-" => Ok(Token::Operator(Operator::Subtract)),
            "*" => Ok(Token::Operator(Operator::Multiply)),
            "/" => Ok(Token::Operator(Operator::Divide)),
            number => number
                .parse::<i64>()
                .map(Token::Number)
                .map_err(|_| "Failed to parse number"),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tokens)
}

/// Evaluates a sequence of tokens strictly from left to right (no precedence).
pub fn evaluate(tokens: &[Token]) -> Result<i64, &'static str> {
    let mut iter = tokens.iter();
    let mut total = match iter.next() {
        Some(Token::Number(n)) => *n,
        _ => return Err("Expression must start with a number"),
    };

    while let (Some(Token::Operator(op)), Some(Token::Number(n))) = (iter.next(), iter.next()) {
        match op {
            Operator::Add => total += n,
            Operator::Subtract => total -= n,
            Operator::Multiply => total *= n,
            Operator::Divide => total /= n, // Zero-division intentionally not handled per requirements.
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_expression() {
        let tokens = parse_input("1 + 2 * 3 / 4 - 5").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Operator(Operator::Add),
                Token::Number(2),
                Token::Operator(Operator::Multiply),
                Token::Number(3),
                Token::Operator(Operator::Divide),
                Token::Number(4),
                Token::Operator(Operator::Subtract),
                Token::Number(5),
            ]
        );
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(parse_input("1 + - 2").is_err());
        assert!(parse_input("(1 + 2)").is_err());
        assert!(parse_input("abc").is_err());
    }

    #[test]
    fn evaluates_left_to_right() {
        let tokens = parse_input("10 - 3 * 2").unwrap();
        let result = evaluate(&tokens).unwrap();
        assert_eq!(result, 14); // (10 - 3) = 7; 7 * 2 = 14
    }
}
