use crate::code::Code;
use crate::report::entry::Entry;
use colored::ColoredString;
use std::fmt::{Display, Formatter};

/// A command-line report.
///
/// # Display
///
/// ```text
/// severity[id]: message
/// entries
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Report {
    code: Code,
    entries: Vec<Entry>,
}

impl From<Code> for Report {
    fn from(code: Code) -> Self {
        Self {
            code,
            entries: Vec::new(),
        }
    }
}

impl Report {
    //! Properties

    /// Gets the code.
    pub fn code(&self) -> &Code {
        &self.code
    }

    /// Gets the entries.
    pub fn entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }
}

impl Report {
    //! Entries

    /// Adds the `entry`.
    pub fn add_entry<E>(&mut self, entry: E)
    where
        E: Into<Vec<ColoredString>>,
    {
        let entry: Vec<ColoredString> = entry.into();
        if !entry.is_empty() {
            self.entries.push(Entry::from(entry));
        }
    }

    /// Adds the `entry`.
    #[must_use]
    pub fn with_entry<E>(mut self, entry: E) -> Self
    where
        E: Into<Vec<ColoredString>>,
    {
        self.add_entry(entry);
        self
    }
}

impl std::error::Error for Report {}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)?;
        for entry in &self.entries {
            writeln!(f)?;
            write!(f, "{}", entry)?;
        }
        Ok(())
    }
}
