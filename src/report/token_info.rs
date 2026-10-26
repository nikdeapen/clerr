use crate::Severity;
use crate::report::util;
use colored::{Color, ColoredString, Colorize};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use unicode_width::UnicodeWidthStr;

/// A token info entry.
///
/// # Display
///
/// ```text
///  --> the/file/name.ext [line=8, position=5]
///   |
/// 8 | the line text
///   |     ^^^^ --- the message text
///   |
/// ```
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TokenInfo<'a> {
    /// The severity used for the underline and message color.
    pub severity: Severity,
    /// The file name.
    pub file_name: &'a str,
    /// The 1-indexed line number.
    pub line: usize,
    /// The 1-indexed column position within the line.
    pub position: usize,
    /// The text content of the line.
    pub line_text: &'a str,
    /// The length of the token in characters. A value of 0 produces no carets.
    pub token_len: usize,
    /// The message displayed under the token.
    pub message: &'a str,
}

impl<'a> TokenInfo<'a> {
    //! Construction

    /// Creates a new token info for the token at the byte `range` of the `source`.
    ///
    /// The line, position, line text & token length are computed from the `source`. The line text
    /// excludes the line ending, and a token that spans multiple lines is cut off at the end of
    /// its first line.
    ///
    /// # Panics
    /// Panics if the `range` is out of bounds or not on char boundaries of the `source`.
    pub fn from_source(
        severity: Severity,
        file_name: &'a str,
        source: &'a str,
        range: Range<usize>,
        message: &'a str,
    ) -> Self {
        let before: &str = &source[..range.start];
        let line_start: usize = before.rfind('\n').map_or(0, |i| i + 1);
        let line_text: &str = source[line_start..].lines().next().unwrap_or("");
        let position: usize = before[line_start..].chars().count();
        let token_len: usize = source[range]
            .chars()
            .count()
            .min(line_text.chars().count().saturating_sub(position));
        Self {
            severity,
            file_name,
            line: before.matches('\n').count() + 1,
            position: position + 1,
            line_text,
            token_len,
            message,
        }
    }
}

impl<'a> From<TokenInfo<'a>> for Vec<ColoredString> {
    fn from(info: TokenInfo<'a>) -> Vec<ColoredString> {
        let char_count: usize = info.line_text.chars().count();
        debug_assert!(info.line >= 1, "line must be 1-indexed");
        debug_assert!(info.position >= 1, "position must be 1-indexed");
        debug_assert!(
            info.position - 1 + info.token_len <= char_count,
            "token exceeds line_text"
        );

        let line_number: String = info.line.to_string();
        let spaces: String = util::char_count(' ', line_number.len());
        let ln_spaces: ColoredString = format!("\n{spaces}").normal();
        let color: Color = info.severity.color();
        let position: usize = info.position - 1;
        let prefix: String = info.line_text.chars().take(position).collect();
        let indent: String = util::char_count(' ', prefix.width());
        let token: String = info
            .line_text
            .chars()
            .skip(position)
            .take(info.token_len)
            .collect();
        let carets: String = util::char_count('^', token.width());

        vec![
            // 1 - file name
            spaces.as_str().normal(),
            "-->".bright_blue(),
            util::file_and_token(info.file_name, info.line, info.position).normal(),
            // 2 - empty
            ln_spaces.clone(),
            " | ".bright_blue(),
            // 3 - line text
            "\n".normal(),
            line_number.bright_blue(),
            " | ".bright_blue(),
            info.line_text.normal(),
            // 4 - error message
            ln_spaces.clone(),
            " | ".bright_blue(),
            indent.normal(),
            carets.color(color),
            " --- ".color(color),
            info.message.color(color),
            // 5 - empty
            ln_spaces,
            " | ".bright_blue(),
        ]
    }
}

impl<'a> Display for TokenInfo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<ColoredString> = Vec::from(*self);
        for string in &strings {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Severity::{Error, Warning};
    use crate::TokenInfo;
    use colored::Colorize;

    fn from_source(source: &str, start: usize, end: usize) -> (usize, usize, &str, usize) {
        let info: TokenInfo = TokenInfo::from_source(Error, "f", source, start..end, "m");
        (info.line, info.position, info.line_text, info.token_len)
    }

    #[test]
    fn from_source_first_line() {
        assert_eq!(from_source("let x = 42;", 4, 5), (1, 5, "let x = 42;", 1));
    }

    #[test]
    fn from_source_later_line() {
        assert_eq!(from_source("a\nbc\n  def\n", 7, 10), (3, 3, "  def", 3));
    }

    #[test]
    fn from_source_crlf() {
        assert_eq!(from_source("a\r\nbc\r\n", 3, 5), (2, 1, "bc", 2));
    }

    #[test]
    fn from_source_counts_chars() {
        assert_eq!(
            from_source("// 你好\nstruct 你 {}", 17, 20),
            (2, 8, "struct 你 {}", 1)
        );
    }

    #[test]
    fn from_source_end_of_input() {
        assert_eq!(from_source("a {\n", 4, 4), (2, 1, "", 0));
    }

    #[test]
    fn from_source_cuts_multi_line_token() {
        assert_eq!(from_source("x // note\ny", 2, 10), (1, 3, "x // note", 7));
    }

    #[test]
    fn display() {
        let info: TokenInfo = TokenInfo {
            severity: Warning,
            file_name: "the/file/name.ext",
            line: 12,
            position: 5,
            line_text: "the line text",
            token_len: 4,
            message: "the 'line' token",
        };

        let result: String = info.to_string();
        let expected: String = [
            "  ".normal(),
            "-->".bright_blue(),
            " the/file/name.ext [line=12, position=5]".normal(),
            "\n  ".normal(),
            " | ".bright_blue(),
            "\n".normal(),
            "12".bright_blue(),
            " | ".bright_blue(),
            "the line text".normal(),
            "\n  ".normal(),
            " | ".bright_blue(),
            "    ".normal(),
            "^^^^".bright_yellow(),
            " --- ".bright_yellow(),
            "the 'line' token".bright_yellow(),
            "\n  ".normal(),
            " | ".bright_blue(),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(result, expected);
    }
}
