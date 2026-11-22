/// Generates `c` `count` times.
pub fn char_count(c: char, count: usize) -> String {
    std::iter::repeat_n(c, count).collect()
}

/// Generates the file and token string.
///
/// ```text
///  the/file/name.ext:8:5
/// ```
///
/// # Note
/// - This is the `file:line:column` form that editors & terminals link to the token.
pub fn file_and_token(file_name: &str, line: usize, position: usize) -> String {
    format!(" {file_name}:{line}:{position}")
}
