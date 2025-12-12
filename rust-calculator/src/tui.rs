//! This module contains the terminal user interface for the calculator.

use std::io;
use std::io::Write as _;

/// Prompt the user for input, returning the raw (whitespace-trimmed) input as a string.
pub fn prompt() -> Box<str> {
    let mut input = String::new();

    loop {
        print!(" > ");
        io::stdout().flush().expect("Failed to flush stdout");

        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {e}. Please retry.");
            input.clear();
            continue;
        }

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }
        break input.into();
    }
}
