use colored::{ColoredString, Colorize};

use crate::Severity;

/// Info concerning a single token of a file.
///
/// # Display
///  --> the/file/name.ext
///   |
/// 8 | the line text
///   |     ^^^^ the error message
#[derive(Copy, Clone, Debug)]
pub struct TokenInfo<'a> {
    /// The file name.
    pub file_name: &'a str,

    /// The 1-indexed file line number of the token.
    pub line: usize,

    /// The 0-indexed position of the first byte of the token within the line.
    pub position: usize,

    /// The token length.
    pub len: usize,

    /// The file line text.
    pub line_text: &'a str,

    /// The error message.
    pub message: &'a str,

    /// The severity.
    pub severity: Severity,
}

impl<'a> TokenInfo<'a> {
    //! Private Utils

    /// Gets `c` repeated `count` times.
    fn char_count(&self, c: char, count: usize) -> String {
        let mut s: String = String::with_capacity(c.len_utf8() * count);
        for _ in 0..count {
            s.push(c);
        }
        s
    }

    /// Gets the blue arrow.
    fn arrow(&self) -> ColoredString {
        "-->".bright_blue()
    }

    /// Gets the blue vertical.
    fn vertical(&self) -> ColoredString {
        " | ".bright_blue()
    }

    /// Gets the file and token string.
    fn file_and_token(&self) -> ColoredString {
        format!(" {}[{}:{}]", self.file_name, self.line, self.position).normal()
    }
}

impl<'a> Into<Vec<ColoredString>> for TokenInfo<'a> {
    fn into(self) -> Vec<ColoredString> {
        let line_number: String = self.line.to_string();
        vec![
            // 1 - file name
            self.char_count(' ', line_number.len()).normal(),
            self.arrow(),
            self.file_and_token(),
            "\n".normal(),
            // 2 - empty
            self.char_count(' ', line_number.len()).normal(),
            self.vertical(),
            "\n".normal(),
            // 3 - line text
            line_number.bright_blue(),
            self.vertical(),
            self.line_text.normal(),
            "\n".normal(),
            // 4 - error message
            self.char_count(' ', line_number.len()).normal(),
            self.vertical(),
            self.char_count(' ', self.position).normal(),
            self.char_count('^', self.len).color(self.severity.color()),
            " --- ".color(self.severity.color()),
            self.message.color(self.severity.color()),
            "\n".normal(),
            // 5 - empty
            self.char_count(' ', line_number.len()).normal(),
            self.vertical(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::text_file::TokenInfo;
    use crate::Severity::{Error, Warning};
    use crate::{Code, Report};

    #[test]
    #[ignore]
    fn display() {
        let info: TokenInfo = TokenInfo {
            file_name: "/the/file/name.ext",
            line: 123,
            position: 7,
            len: 5,
            line_text: "Hello, World!",
            message: "this is `World`",
            severity: Warning,
        };
        let report: Report =
            Report::new(Code::new("CODE", Error, "warning message")).with_entry(info);
        println!("{}", report);
    }
}
