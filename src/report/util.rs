/// Generates `c` `count` times.
pub fn char_count(c: char, count: usize) -> String {
    std::iter::repeat_n(c, count).collect()
}

/// Generates the file and token string.
pub fn file_and_token(file_name: &str, line: usize, position: usize) -> String {
    let position: usize = position + 1;
    format!(" {file_name} [line={line}, position={position}]")
}
