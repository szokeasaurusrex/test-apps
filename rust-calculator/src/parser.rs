//! Parsing logic for the calculator.

use std::sync::LazyLock;

use regex::Regex;

const INPUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?:(?<number>\d+)\s*(?<operator>[+\-*/]))\s*)*(?:(?<last_number>\d+))$")
        .unwrap()
});

pub fn parse_input(input: &str) -> Vec<&str> {
    INPUT_REGEX
        .captures(input)
        .map(|captures| captures.iter())
        .iter()
        .flatten()
        .collect()
}
