use std::fmt::{Display, Formatter};

use colored::ColoredString;

use crate::code::Code;

/// A command-line report.
#[derive(Clone, Debug)]
pub struct Report {
    code: Code,
    entries: Vec<Vec<ColoredString>>,
}

impl Report {
    //! Construction

    /// Creates a new report.
    pub fn new(code: Code) -> Self {
        Self {
            code,
            entries: vec![],
        }
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
        write!(f, "{}\n", self.code)?;
        for entry in &self.entries {
            for string in entry {
                write!(f, "{}", string)?;
            }
        }
        Ok(())
    }
}
