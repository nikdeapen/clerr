use crate::report::char_count;
use crate::report::utils::{arrow, file_and_token, vertical};
use crate::Severity;
use colored::{ColoredString, Colorize};

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
    pub file_name: &'a str,
    pub line: usize,
    pub position: usize,
    pub line_text: &'a str,
    pub token_len: usize,
    pub severity: Severity,
    pub message: &'a str,
}

impl<'a> TokenInfo<'a> {
    //! Entry

    /// Constructs the report entry.
    pub fn entry(&self) -> Vec<ColoredString> {
        let line_number: String = self.line.to_string();
        vec![
            // 1 - file name
            char_count(' ', line_number.len()).normal(),
            arrow(),
            file_and_token(self.file_name, self.line, self.position),
            "\n".normal(),
            // 2 - empty
            char_count(' ', line_number.len()).normal(),
            vertical(),
            "\n".normal(),
            // 3 - line text
            line_number.bright_blue(),
            vertical(),
            self.line_text.normal(),
            "\n".normal(),
            // 4 - error message
            char_count(' ', line_number.len()).normal(),
            vertical(),
            char_count(' ', self.position).normal(),
            char_count('^', self.token_len).color(self.severity.color()),
            " --- ".color(self.severity.color()),
            self.message.color(self.severity.color()),
            "\n".normal(),
            // 5 - empty
            char_count(' ', line_number.len()).normal(),
            vertical(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::Severity::Warning;
    use crate::{Code, Report, TokenInfo};

    #[test]
    #[ignore]
    fn token_info() {
        let info: TokenInfo = TokenInfo {
            file_name: "the/file/name.ext",
            line: 12,
            position: 4,
            line_text: "the line text",
            token_len: 4,
            severity: Warning,
            message: "the 'line' token",
        };
        let code: Code = Code::error("an-error-code", "an error message");
        let report: Report = Report::new(code).with_entry(info.entry());
        println!("{}", report)
    }
}
