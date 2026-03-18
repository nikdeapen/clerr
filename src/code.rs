use crate::Severity;
use crate::Severity::{Error, Info, Warning};
use colored::{Color, Colorize};
use std::fmt::{Display, Formatter};

/// A command-line report code with a severity, identifier, and message.
///
/// # Display
///
/// ```text
/// severity[id]: message
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Code {
    severity: Severity,
    id: String,
    message: String,
}

impl Code {
    //! Construction

    /// Creates a new command-line report code.
    pub fn new<S0, S1>(severity: Severity, id: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self {
            severity,
            id: id.into(),
            message: message.into(),
        }
    }

    /// Creates a new error code.
    pub fn error<S0, S1>(id: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(Error, id, message)
    }

    /// Creates a new warning code.
    pub fn warning<S0, S1>(id: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(Warning, id, message)
    }

    /// Creates a new info code.
    pub fn info<S0, S1>(id: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(Info, id, message)
    }
}

impl Code {
    //! Properties

    /// Gets the severity.
    pub fn severity(&self) -> Severity {
        self.severity
    }

    /// Gets the identifier.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Gets the message.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color: Color = self.severity.color();
        write!(
            f,
            "{}{}{}{}{}",
            self.severity,
            "[".color(color),
            self.id.color(color),
            "]: ".color(color),
            self.message.bright_white().bold()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Code;

    #[test]
    fn display() {
        let code: Code = Code::error("12345", "the error message");
        println!("code:\n{}\n", code);
    }
}
