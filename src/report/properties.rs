use crate::report::util;
use crate::Severity::Info;
use colored::Colorize;
use std::fmt::{Display, Formatter};

/// A properties entry.
///
/// # Display
///
/// ```text
///     first:  value
///     second: another value
///     third:  third value
/// ```
#[derive(Clone, Debug, Default)]
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
    #[must_use]
    pub fn with_property<S0, S1>(mut self, name: S0, value: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.add(name, value);
        self
    }
}

impl Display for Properties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_len: usize = self
            .properties
            .iter()
            .map(|(p, _)| p.len())
            .max()
            .unwrap_or(0);
        let len: usize = self.properties.len();
        for (i, (property, value)) in self.properties.iter().enumerate() {
            let spaces: String = util::char_count(' ', max_len - property.len() + 2);
            write!(
                f,
                "    {}{}{}{}",
                property.color(Info.color()),
                ":".color(Info.color()),
                spaces,
                value
            )?;
            if i + 1 < len {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Properties, Report};

    #[test]
    fn column_alignment() {
        colored::control::set_override(false);
        let entry: String = Properties::default()
            .with_property("a", "v1")
            .with_property("abc", "v2")
            .with_property("ab", "v3")
            .to_string();

        // Longest name is "abc" (3 chars).
        // "a"   (1 char) -> 3 - 1 + 2 = 4 spaces after colon
        // "abc" (3 chars) -> 3 - 3 + 2 = 2 spaces after colon
        // "ab"  (2 chars) -> 3 - 2 + 2 = 3 spaces after colon
        let lines: Vec<&str> = entry.lines().collect();
        assert_eq!(lines[0], "    a:    v1");
        assert_eq!(lines[1], "    abc:  v2");
        assert_eq!(lines[2], "    ab:   v3");
    }

    #[test]
    fn properties() {
        let properties: Properties = Properties::default()
            .with_property("one", "two")
            .with_property("three", "four")
            .with_property("five", "six");
        let code: Code = Code::error("an-error-code", "an error message");
        let report: Report = Report::from(code).with_entry(properties);
        println!("{}", report)
    }
}
