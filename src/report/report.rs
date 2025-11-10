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
    pub fn add_entry<E>(&mut self, entry: E)
    where
        E: Into<Vec<ColoredString>>,
    {
        self.entries.push(entry.into());
    }

    /// Adds the `entry`.
    pub fn with_entry<E>(mut self, entry: E) -> Self
    where
        E: Into<Vec<ColoredString>>,
    {
        self.add_entry(entry);
        self
    }
}

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
