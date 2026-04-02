use colored::ColoredString;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

/// A report entry that wraps [Vec<ColoredString>] and ignores color on comparison operations.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Entry {
    pub strings: Vec<ColoredString>,
}

impl From<Vec<ColoredString>> for Entry {
    fn from(strings: Vec<ColoredString>) -> Self {
        Self { strings }
    }
}

impl From<Entry> for Vec<ColoredString> {
    fn from(entry: Entry) -> Self {
        entry.strings
    }
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.strings.len() == other.strings.len()
            && self
                .strings
                .iter()
                .zip(other.strings.iter())
                .all(|(a, b)| a.as_bytes() == b.as_bytes())
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strings
            .iter()
            .map(|fragment| fragment.as_bytes())
            .cmp(other.strings.iter().map(|fragment| fragment.as_bytes()))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strings.len().hash(state);
        for fragment in &self.strings {
            fragment.as_bytes().hash(state);
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for fragment in &self.strings {
            write!(f, "{}", fragment)?;
        }
        Ok(())
    }
}
