use crate::Severity;
use crate::report::util;
use colored::{Color, ColoredString, Colorize};
use std::fmt::{Display, Formatter};

/// A token info entry.
///
/// # Display
///
/// ```text
///  --> the/file/name.ext
///   |
/// 8 | the line text
///   |     ^^^^ the message text
///   |
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TokenInfo<'a> {
    /// The severity used for the underline and message color.
    pub severity: Severity,
    /// The file name.
    pub file_name: &'a str,
    /// The 1-based line number.
    pub line: usize,
    /// The 0-based column position within the line.
    pub position: usize,
    /// The text content of the line.
    pub line_text: &'a str,
    /// The length of the token in characters.
    pub token_len: usize,
    /// The message displayed under the token.
    pub message: &'a str,
}

impl<'a> From<TokenInfo<'a>> for Vec<ColoredString> {
    fn from(info: TokenInfo<'a>) -> Vec<ColoredString> {
        let line_number: String = info.line.to_string();
        let spaces: String = util::char_count(' ', line_number.len());
        let color: Color = info.severity.color();
        let indent: String = util::char_count(' ', info.position);
        let carets: String = util::char_count('^', info.token_len);

        vec![
            // 1 - file name
            spaces.clone().normal(),
            "-->".bright_blue(),
            util::file_and_token(info.file_name, info.line, info.position).normal(),
            // 2 - empty
            format!("\n{spaces}").normal(),
            " | ".bright_blue(),
            // 3 - line text
            "\n".normal(),
            line_number.bright_blue(),
            " | ".bright_blue(),
            info.line_text.normal(),
            // 4 - error message
            format!("\n{spaces}").normal(),
            " | ".bright_blue(),
            indent.normal(),
            carets.color(color),
            " --- ".color(color),
            info.message.color(color),
            // 5 - empty
            format!("\n{spaces}").normal(),
            " | ".bright_blue(),
        ]
    }
}

impl<'a> Display for TokenInfo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line_number: String = self.line.to_string();
        let spaces: String = util::char_count(' ', line_number.len());
        let color: Color = self.severity.color();
        let indent: String = util::char_count(' ', self.position);
        let carets: String = util::char_count('^', self.token_len);

        // 1 - file name
        write!(
            f,
            "{}{}{}",
            spaces,
            "-->".bright_blue(),
            util::file_and_token(self.file_name, self.line, self.position)
        )?;
        // 2 - empty
        write!(f, "\n{}{}", spaces, " | ".bright_blue())?;
        // 3 - line text
        write!(
            f,
            "\n{}{}{}",
            line_number.bright_blue(),
            " | ".bright_blue(),
            self.line_text
        )?;
        // 4 - error message
        write!(
            f,
            "\n{}{}{}{}{}{}",
            spaces,
            " | ".bright_blue(),
            indent,
            carets.color(color),
            " --- ".color(color),
            self.message.color(color)
        )?;
        // 5 - empty
        write!(f, "\n{}{}", spaces, " | ".bright_blue())
    }
}

#[cfg(test)]
mod tests {
    use crate::Severity::Warning;
    use crate::{Code, Report, TokenInfo};

    #[test]
    fn token_info() {
        let info: TokenInfo = TokenInfo {
            severity: Warning,
            file_name: "the/file/name.ext",
            line: 12,
            position: 4,
            line_text: "the line text",
            token_len: 4,
            message: "the 'line' token",
        };
        let code: Code = Code::error("an-error-code", "an error message");
        let report: Report = Report::from(code).with_entry(info);
        println!("token-info:\n{}\n", report);
    }
}
