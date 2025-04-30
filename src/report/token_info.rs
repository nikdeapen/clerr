use crate::{Report, Severity};
use colored::{ColoredString, Colorize};

impl Report {
    //! Token Info

    /// Generates the token info entry.
    ///
    /// # Display
    ///  --> the/file/name.ext
    ///   |
    /// 8 | the line text
    ///   |     ^^^^ the message
    ///   |
    pub fn token_info(
        file_name: &str,
        line: usize,
        position: usize,
        line_text: &str,
        token_len: usize,
        severity: Severity,
        message: &str,
    ) -> Vec<ColoredString> {
        let line_number: String = line.to_string();
        vec![
            // 1 - file name
            Self::char_count(' ', line_number.len()).normal(),
            Self::arrow(),
            Self::file_and_token(file_name, line, position),
            "\n".normal(),
            // 2 - empty
            Self::char_count(' ', line_number.len()).normal(),
            Self::vertical(),
            "\n".normal(),
            // 3 - line text
            line_number.bright_blue(),
            Self::vertical(),
            line_text.normal(),
            "\n".normal(),
            // 4 - error message
            Self::char_count(' ', line_number.len()).normal(),
            Self::vertical(),
            Self::char_count(' ', position).normal(),
            Self::char_count('^', token_len).color(severity.color()),
            " --- ".color(severity.color()),
            message.color(severity.color()),
            "\n".normal(),
            // 5 - empty
            Self::char_count(' ', line_number.len()).normal(),
            Self::vertical(),
        ]
    }

    /// Adds the token info entry.
    ///
    /// See `Report::token_info`.
    pub fn add_token_info(
        &mut self,
        file_name: &str,
        line: usize,
        position: usize,
        line_text: &str,
        token_len: usize,
        severity: Severity,
        message: &str,
    ) {
        self.add_entry(Self::token_info(
            file_name, line, position, line_text, token_len, severity, message,
        ));
    }

    /// Adds the token info entry.
    ///
    /// See `Report::token_info`.
    pub fn with_token_info(
        mut self,
        file_name: &str,
        line: usize,
        position: usize,
        line_text: &str,
        token_len: usize,
        severity: Severity,
        message: &str,
    ) -> Self {
        self.add_token_info(
            file_name, line, position, line_text, token_len, severity, message,
        );
        self
    }
}

impl Report {
    //! Token Info Utils

    /// Generates `c` `count` times.
    pub(in crate::report) fn char_count(c: char, count: usize) -> String {
        let mut s: String = String::with_capacity(c.len_utf8() * count);
        for _ in 0..count {
            s.push(c);
        }
        s
    }

    fn arrow() -> ColoredString {
        "-->".bright_blue()
    }

    fn vertical() -> ColoredString {
        " | ".bright_blue()
    }

    fn file_and_token(file_name: &str, line: usize, position: usize) -> ColoredString {
        format!(" {} [line={}, position={}]", file_name, line, position).normal()
    }
}

#[cfg(test)]
mod tests {
    use crate::Report;
    use crate::Severity::Warning;
    use colored::ColoredString;

    #[test]
    #[ignore]
    fn token_info() {
        let info: Vec<ColoredString> = Report::token_info(
            "the/file/name.ext",
            12,
            4,
            "the line text",
            4,
            Warning,
            "the 'line' token",
        );

        for s in info {
            print!("{}", s);
        }
    }
}
