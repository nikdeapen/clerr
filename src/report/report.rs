use crate::code::Code;
use colored::ColoredString;
use std::fmt::{Display, Formatter};

/// A command-line report.
///
/// # Display
///
/// ```text
/// severity[code]: message
/// entries
/// ```
#[derive(Clone, Debug)]
pub struct Report {
    code: Code,
    entries: Vec<Vec<ColoredString>>,
}

impl Report {
    //! Construction

    /// Creates a new command-line report.
    pub const fn new(code: Code) -> Self {
        Self {
            code,
            entries: vec![],
        }
    }
}

impl From<Code> for Report {
    fn from(code: Code) -> Self {
        Self::new(code)
    }
}

impl Report {
    //! Entries

    /// Adds the `entry`.
    pub fn add_entry(&mut self, entry: impl super::ReportEntry) {
        self.entries.push(entry.entry());
    }

    /// Adds the `entry`.
    #[must_use]
    pub fn with_entry(mut self, entry: impl super::ReportEntry) -> Self {
        self.add_entry(entry);
        self
    }
}

impl Report {
    //! Printing

    /// Prints the report to stderr.
    pub fn eprint(&self) {
        eprintln!("{self}");
    }
}

impl std::error::Error for Report {}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.code)?;
        for entry in &self.entries {
            for string in entry {
                write!(f, "{string}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
