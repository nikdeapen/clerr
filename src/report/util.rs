use colored::{ColoredString, Colorize};

/// Generates `c` `count` times.
pub fn char_count(c: char, count: usize) -> String {
    let mut s: String = String::with_capacity(c.len_utf8() * count);
    for _ in 0..count {
        s.push(c);
    }
    s
}

/// Generates the two-dash arrow
pub fn arrow() -> ColoredString {
    "-->".bright_blue()
}

/// Generates a vertical line with surrounding spaces.
pub fn vertical() -> ColoredString {
    " | ".bright_blue()
}

/// Generates the file and token string.
pub fn file_and_token(file_name: &str, line: usize, position: usize) -> ColoredString {
    let position: usize = position + 1;
    format!(" {file_name} [line={line}, position={position}]").normal()
}
