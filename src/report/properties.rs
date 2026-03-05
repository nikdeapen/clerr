use crate::report::util;
use crate::Severity::Info;
use colored::{ColoredString, Colorize};

/// A properties entry.
///
/// # Display
///
/// ```text
///     first:  value
///     second: another value
///     third:  third value
/// ```
#[derive(Default)]
pub struct Properties {
    properties: Vec<(String, String)>,
}

impl Properties {
    //! Properties

    /// Gets the properties.
    pub fn properties(&self) -> &[(String, String)] {
        self.properties.as_slice()
    }

    /// Adds the property.
    pub fn add<S0, S1>(&mut self, name: S0, value: S1)
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.properties.push((name.into(), value.into()));
    }

    /// Adds the property.
    pub fn with<S0, S1>(mut self, name: S0, value: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.add(name, value);
        self
    }
}

impl Properties {
    //! Entry

    /// Constructs the report entry.
    pub fn entry(mut self) -> Vec<ColoredString> {
        let max_len: usize = self
            .properties
            .iter()
            .map(|(p, _)| p.len())
            .max()
            .unwrap_or(0);
        let len = self.properties.len();
        let mut entry: Vec<ColoredString> = Vec::with_capacity(len * 6);
        for (i, (property, value)) in self.properties.drain(..).enumerate() {
            let prop_len: usize = property.len();
            let spaces: usize = max_len - prop_len + 2;

            entry.push("    ".normal());
            entry.push(property.color(Info.color()));
            entry.push(":".color(Info.color()));
            entry.push(util::char_count(' ', spaces).normal());
            entry.push(value.normal());
            if i + 1 < len {
                entry.push("\n".normal());
            }
        }
        entry
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Properties, Report};
    use std::ops::Deref;

    #[test]
    fn column_alignment() {
        let entry = Properties::default()
            .with("a", "v1")
            .with("abc", "v2")
            .with("ab", "v3")
            .entry();

        // Each property produces 5 or 6 elements: indent, name, colon, spaces, value, [newline]
        // Extract the spacing element (index 3) from each property's group
        let spacing = |group: usize| -> &str {
            let base = group * 6; // 6 elements per non-last group, but last has 5
            let idx = if group < 2 { base + 3 } else { base + 3 };
            entry[idx].deref()
        };

        // Longest name is "abc" (3 chars).
        // "a"   (1 char) -> 3 - 1 + 2 = 4 spaces
        // "abc" (3 chars) -> 3 - 3 + 2 = 2 spaces
        // "ab"  (2 chars) -> 3 - 2 + 2 = 3 spaces
        assert_eq!(spacing(0), "    ");
        assert_eq!(spacing(1), "  ");
        assert_eq!(spacing(2), "   ");
    }

    #[test]
    #[ignore]
    fn properties() {
        let properties: Properties = Properties {
            properties: vec![
                ("one".to_string(), "two".to_string()),
                ("three".to_string(), "four".to_string()),
                ("five".to_string(), "six".to_string()),
            ],
        };
        let code: Code = Code::error("an-error-code", "an error message");
        let report: Report = Report::new(code).with_entry(properties.entry());
        println!("{}", report)
    }
}
