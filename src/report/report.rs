use crate::code::Code;
use std::fmt::{Display, Formatter};

/// A command-line report.
///
/// # Display
///
/// ```text
/// severity[id]: message
/// entries
/// ```
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Report {
    code: Code,
    entries: String,
}

impl From<Code> for Report {
    fn from(code: Code) -> Self {
        Self {
            code,
            entries: String::new(),
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
    pub fn entries(&self) -> &str {
        self.entries.as_str()
    }
}

impl Report {
    //! Entries

    /// Adds the `entry`.
    pub fn add_entry<D>(&mut self, entry: D)
    where
        D: Display,
    {
        let entry: String = entry.to_string();
        if self.entries.is_empty() {
            self.entries.push_str(entry.as_str());
        } else {
            self.entries.reserve(1 + entry.len());
            self.entries.push('\n');
            self.entries.push_str(entry.as_str());
        }
    }

    /// Adds the `entry`.
    #[must_use]
    pub fn with_entry<D>(mut self, entry: D) -> Self
    where
        D: Display,
    {
        self.add_entry(entry);
        self
    }
}

impl std::error::Error for Report {}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.entries.is_empty() {
            write!(f, "{}", self.code)
        } else {
            write!(f, "{}\n{}", self.code, self.entries)
        }
    }
}
