use crate::Severity;
use crate::Severity::{Error, Info, Warning};
use colored::Colorize;
use std::fmt::{Display, Formatter};

/// A command-line report code with an associated severity and message.
///
/// # Display
///
/// ```text
/// severity[code]: message
/// ```
#[derive(Clone, Debug)]
pub struct Code {
    severity: Severity,
    code: String,
    message: String,
}

impl Code {
    //! Construction

    /// Creates a new command-line report code.
    pub const fn new(severity: Severity, code: String, message: String) -> Self {
        Self {
            severity,
            code,
            message,
        }
    }

    /// Creates a new error code.
    pub fn error<S0, S1>(code: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(Error, code.into(), message.into())
    }

    /// Creates a new warning code.
    pub fn warning<S0, S1>(code: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(Warning, code.into(), message.into())
    }

    /// Creates a new info code.
    pub fn info<S0, S1>(code: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(Info, code.into(), message.into())
    }
}

impl Code {
    //! Properties

    /// Gets the severity.
    pub fn severity(&self) -> Severity {
        self.severity
    }

    /// Gets the code.
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Gets the message.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.severity,
            "[".color(self.severity.color()),
            self.code.color(self.severity.color()),
            "]: ".color(self.severity.color()),
            self.message.bright_white().bold()
        )
    }
}

impl std::error::Error for Code {}

#[cfg(test)]
mod tests {
    use crate::Code;

    #[test]
    #[ignore]
    fn display() {
        let code: Code = Code::warning("12345", "the error message");
        println!("{}", code);
    }
}
