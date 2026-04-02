use colored::{Color, Colorize};
use std::fmt::{Display, Formatter};

/// The severity of a command-line report.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    //! Display

    /// Gets the label string.
    pub fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }

    /// Gets the associated color.
    pub fn color(self) -> Color {
        match self {
            Self::Error => Color::BrightRed,
            Self::Warning => Color::BrightYellow,
            Self::Info => Color::BrightBlue,
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label().color(self.color()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Severity;
    use colored::{Color, Colorize};

    #[test]
    fn display() {
        let test_cases: &[(Severity, &str, Color)] = &[
            (Severity::Error, "error", Color::BrightRed),
            (Severity::Warning, "warning", Color::BrightYellow),
            (Severity::Info, "info", Color::BrightBlue),
        ];

        for (severity, expected_label, expected_color) in test_cases {
            let result: String = severity.to_string();
            let expected: String = expected_label.color(*expected_color).to_string();

            assert_eq!(result, expected);
        }
    }
}
