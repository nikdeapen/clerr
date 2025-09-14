use colored::{Color, Colorize};
use std::fmt::{Display, Formatter};

/// The severity of a command-line report.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    //! Color

    /// Gets the associated color.
    pub fn color(&self) -> Color {
        match self {
            Self::Error => Color::BrightRed,
            Self::Warning => Color::BrightYellow,
            Self::Info => Color::BrightBlue,
        }
    }
}

impl Severity {
    //! Display

    /// Gets the label string.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label().color(self.color()))
    }
}
