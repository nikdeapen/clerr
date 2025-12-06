use crate::report::util;
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
            util::char_count(' ', line_number.len()).normal(),
            util::arrow(),
            util::file_and_token(self.file_name, self.line, self.position),
            "\n".normal(),
            // 2 - empty
            util::char_count(' ', line_number.len()).normal(),
            util::vertical(),
            "\n".normal(),
            // 3 - line text
            line_number.bright_blue(),
            util::vertical(),
            self.line_text.normal(),
            "\n".normal(),
            // 4 - error message
            util::char_count(' ', line_number.len()).normal(),
            util::vertical(),
            util::char_count(' ', self.position).normal(),
            util::char_count('^', self.token_len).color(self.severity.color()),
            " --- ".color(self.severity.color()),
            self.message.color(self.severity.color()),
            "\n".normal(),
            // 5 - empty
            util::char_count(' ', line_number.len()).normal(),
            util::vertical(),
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
