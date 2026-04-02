use crate::Severity;
use crate::report::util;
use colored::{Color, ColoredString, Colorize};
use std::fmt::{Display, Formatter};
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
    use crate::Severity::Warning;
    use crate::TokenInfo;
    use colored::Colorize;

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
