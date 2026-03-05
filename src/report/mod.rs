use colored::ColoredString;

pub use properties::*;
pub use report::*;
pub use token_info::*;

mod properties;
mod report;
mod token_info;

pub(crate) mod util;

/// A report entry that can be added to a [`Report`].
pub trait ReportEntry {
    /// Constructs the report entry.
    fn entry(self) -> Vec<ColoredString>;
}

impl ReportEntry for Vec<ColoredString> {
    fn entry(self) -> Vec<ColoredString> {
        self
    }
}
