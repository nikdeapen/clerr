use crate::Severity;
use crate::Severity::{Error, Info, Warning};
use colored::Colorize;
use std::fmt::{Display, Formatter};

/// An error code with an associated severity and message.
///
/// # Display
/// severity[code]: message
#[derive(Clone, Debug)]
pub struct Code {
    code: String,
    severity: Severity,
    message: String,
}

impl Code {
    //! Construction

    /// Creates a new code.
    pub fn new<S0, S1>(code: S0, severity: Severity, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        let code: String = code.into();
        let message: String = message.into();
        Self {
            code,
            severity,
            message,
        }
    }

    /// Creates a new error code.
    pub fn error<S0, S1>(error_code: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(error_code, Error, message)
    }

    /// Creates a new warning code.
    pub fn warning<S0, S1>(error_code: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(error_code, Warning, message)
    }

    /// Creates a new info code.
    pub fn info<S0, S1>(error_code: S0, message: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self::new(error_code, Info, message)
    }
}

impl Code {
    //! Properties

    /// Gets the code.
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Gets the severity.
    pub fn severity(&self) -> Severity {
        self.severity
    }

    /// Gets the message.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.severity.to_str().color(self.severity.color()))?;
        write!(f, "{}", "[".color(self.severity.color()))?;
        write!(f, "{}", self.code.color(self.severity.color()))?;
        write!(f, "{}", "]: ".color(self.severity.color()))?;
        write!(f, "{}", self.message.bright_white().bold())
    }
}
