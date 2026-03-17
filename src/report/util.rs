use colored::Colorize;

/// Generates `c` `count` times.
pub fn char_count(c: char, count: usize) -> String {
    std::iter::repeat_n(c, count).collect()
}

/// Generates the two-dash arrow.
pub fn arrow() -> String {
    format!("{}", "-->".bright_blue())
}

/// Generates a vertical line with surrounding spaces.
pub fn vertical() -> String {
    format!("{}", " | ".bright_blue())
}

/// Generates the file and token string.
pub fn file_and_token(file_name: &str, line: usize, position: usize) -> String {
    let position: usize = position + 1;
    format!(" {file_name} [line={line}, position={position}]")
}
